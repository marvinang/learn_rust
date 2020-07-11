use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

/// sender发送的信息的类型的枚举
///
/// NewJob - 表示新的任务
///
/// Terminate - 表示结束
enum Message {
    NewJob(Job),
    Terminate,
}

/// 线程池的结构体
/// # Arguments
///
/// * workers - 是实际的任务运行者
/// * sender - 用于发送任务
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    /// 创建线程池。
    ///
    /// 线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic。
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // 一个sender 对应 一个 receiver
        let (sender, receiver) = mpsc::channel();

        // Mutex为receiver加锁
        // Arc使receiver能在多个线程中调用
        let receiver = Arc::new(Mutex::new(receiver));

        // 构建Worker的容器Vector，初始化容量为size
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// 执行任务
    ///
    /// 由sender发送任务信息
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    /// 当sender发送终止的消息时，停掉所有的worker
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// 构建Worker
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // 接受到消息时，此处会被回调
            let message = receiver.lock().unwrap().recv().unwrap();

            // 对消息类型进行匹配，决定如何执行
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job.call_box();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

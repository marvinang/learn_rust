use a20_webserver;

use a20_webserver::ThreadPool;

use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    // 绑定本地IP:Port监听
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 创建线程池
    let pool = ThreadPool::new(8);

    // 遍历监听的listener
    // take(10) 表示只接受10个请求
    for stream in listener.incoming().take(10) {
        let stream = stream.unwrap();

        // 使用线程池处理请求过来的stream
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

/// 处理tcp连接请求，进行不同的响应
fn handle_connection(mut stream: TcpStream) {
    // 将stream存入buffer
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // 判断请求以什么开头，决定响应不同的内容
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        // 线程睡5秒
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // 读取本地文件，并转换为String
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();

    // 封装响应体
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, contents.len(), contents);

    // 返回数据
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

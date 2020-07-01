/*
错误处理

Rust 有一套独特的处理异常情况的机制，它并不像其它语言中的 try 机制那样简单。
recoverable and unrecoverable
首先，程序中一般会出现两种错误：可恢复错误和不可恢复错误。
可恢复错误的典型案例是文件访问错误，如果访问一个文件失败，有可能是因为它正在被占用，是正常的，我们可以通过等待来解决。

但还有一种错误是由编程中无法解决的逻辑错误导致的，例如访问数组末尾以外的位置。
大多数编程语言不区分这两种错误，并用 Exception （异常）类来表示错误。在 Rust 中没有 Exception。
对于可恢复错误用 Result<T, E> 类来处理，对于不可恢复错误使用 panic! 宏来处理。

*/

// 可以加  RUST_BACKTRACE=1 参数来打印异常栈

// 不可恢复错误
// panic!宏执行时，程序打印一个失败消息，释放和清理栈空间，然后退出程序。
// 如果不想程序进行清理工作，要直接abort，让操作系统来清理空间。
// 可以在Cargo.toml文件里加上如下片段:
// [profile.release]
// panic = "abort"
fn panic_error() {
    panic!("error occured");
    println!("Hello, Rust");
}

// 可恢复的错误
// 此概念十分类似于 Java 编程语言中的异常。
// 实际上在 C 语言中我们就常常将函数返回值设置成整数来表达函数遇到的错误，
// 在 Rust 中通过 Result<T, E> 枚举类作返回值来进行异常表达：

/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

use std::fs::File;
fn deal_file() {
    // 不必关闭f, rust会自动清理
    let f = File::open("hello.txt");
    // 这里不需要写 Result::Ok(file), 因为Result Enum已经被prelude引入了
    match f {
        Ok(file) => {
            println!("file {:?} opened successfully.", file);
        }
        Err(err) => {
            println!("file opened failed.");
            panic!("Problem opening the file: {:?}", err);
        }
    }
}

// match 不同的错误类型
use std::io::ErrorKind;
fn match_error_kind() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

// 使用 if let 简化
fn deal_file_2() {
    let f = File::open("hello.txt");
    if let Ok(file) = f {
        println!("File opened successfully.");
    } else {
        println!("Failed to open the file.");
    }
}

// 如果想使一个可恢复错误按不可恢复错误处理，
// Result 类提供了两个办法：unwrap() 和 expect(message: &str) ：
//
// 这段程序相当于在 Result 为 Err 时调用 panic! 宏。
// 两者的区别在于 expect 能够向 panic! 宏发送一段指定的错误信息。
fn deal_file_error() {
    let f1 = File::open("hello.txt").unwrap();
    let f2 = File::open("hello.txt").expect("Failed to open.");
}

// 可恢复的错误的传递
// 之前所讲的是接收到错误的处理方式，但是如果我们自己编写一个函数在遇到错误时想传递出去怎么办呢？
fn deliver_error(i: i32) {
    let r = f(i);
    if let Ok(v) = r {
        println!("Ok: f(-1) = {}", v);
    } else {
        println!("Err");
    }
}

fn f(i: i32) -> Result<i32, bool> {
    if i >= 0 {
        Ok(i)
    } else {
        Err(false)
    }
}

// 函数 g 传递了函数 f 可能出现的错误（这里的 g 只是一个简单的例子，
// 实际上传递错误的函数一般还包含很多其它操作）。
fn g1(i: i32) -> Result<i32, bool> {
    let t = f(i);
    return match t {
        Ok(i) => Ok(i),
        Err(b) => Err(b),
    };
}

// 这样写有些冗长，Rust 中可以在 Result 对象后添加 ? 操作符将同类的 Err 直接传递出去：
// ? 符的实际作用是将 Result 类非异常的值直接取出，如果有异常就将异常 Result 返回出去。
// 所以，? 符仅用于返回值类型为 Result<T, E> 的函数，
// 其中 E 类型必须和 ? 所处理的 Result 的 E 类型一致。
fn g(i: i32) -> Result<i32, bool> {
    let t = f(i)?;
    Ok(t) // 因为确定 t 不是 Err, t 在这里已经是 i32 类型
}

// kind 方法
/*
Rust 似乎没有像 try 块一样可以令任何位置发生的同类异常都直接得到相同的解决的语法，
但这样并不意味着 Rust 实现不了：
我们完全可以把 try 块在独立的函数中实现，将所有的异常都传递出去解决。
实际上这才是一个分化良好的程序应当遵循的编程方法：应该注重独立功能的完整性。

但是这样需要判断 Result 的 Err 类型，获取 Err 类型的函数是 kind()。
*/

use std::io;
use std::io::Read;

fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn deal_good() {
    let str_file = read_text_from_file("hello.txt");
    match str_file {
        Ok(s) => println!("{}", s),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                println!("No such file");
            }
            _ => {
                println!("Cannot read the file");
            }
        },
    }
}

fn main() {
    // panic_error();
    deal_file();
    match_error_kind();
    // deal_file_2();

    // deal_file_error();

    // deliver_error(10);
    // deliver_error(-1);

    // deal_good();
}

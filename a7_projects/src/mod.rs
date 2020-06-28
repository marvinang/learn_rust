/*
在 Rust 中，模块就像是 Java 中的类包装，
但是文件一开头就可以写一个主函数，这该如何解释呢？
每一个 Rust 文件的内容都是一个"难以发现"的模块。
让我们用两个文件来揭示这一点：
*/

mod second_module;

fn main() {
    println!("This is the main module.");
    println!("{}", second_module::message());
}

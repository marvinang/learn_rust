

// 在 Rust 中，std::io 模块提供了标准输入（可认为是命令行输入）的相关功能：

use std::io::stdin;

fn main() {
    let mut str_buf = String::new();
    stdin().read_line(&mut str_buf).expect("Failed to read line.");
    println!("Your input line is \n{}", str_buf);
}

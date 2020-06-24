
// 文件读取

use std::fs;

fn main() {
    let text = fs::read_to_string("text.txt").unwrap();
    println!("{}", text);
    
    let content = fs::read("binary.bin").unwrap();
    println!("{:?}", content);

}

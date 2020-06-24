

// 文件写入分为一次性写入和流式写入。流式写入需要打开文件，打开方式有"新建"（create）和"追加"（append）两种。

// 一次性写入：
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    fs::write("text.txt", "From rust program").unwrap();

    // 流式写入
    // 如果想使用流的方式写入文件内容，可以使用 std::fs::File 的 create 方法：
    
    let mut file = File::create("write.txt").unwrap();
    file.write(b"From rust program").unwrap();
}

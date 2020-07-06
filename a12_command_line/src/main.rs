/*
 *
 * 一个命令行程序
 */

use std::env;
use std::process;

use a12_command_line::{self, Config};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // 使用迭代器
    // let config = Config::new(&args).unwrap_or_else(|err| {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    if let Err(e) = a12_command_line::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

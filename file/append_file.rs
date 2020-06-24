
// File 类中不存在 append 静态方法，
// 但是我们可以使用 OpenOptions 来实现用特定方法打开文件：
//

use std::io::prelude::*;
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new().append(true).open("text.txt")?;

    file.write(b"appedn word\n")?;

    Ok(())
}



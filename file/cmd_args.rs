

// Rust 中主函数是个无参函数，环境参数需要开发者通过 std::env 模块取出，过程十分简单
fn main() {
    let args = std::env::args();
    println!("{:?}", args);
}

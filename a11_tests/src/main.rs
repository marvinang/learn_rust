/*

rust使用test属性标记函数成为一个测试案例。
属性是关于一段rust代码的元数据。




*/
#[cfg(test)]
mod tests {
    #[test]
    fn another() {
        panic!("Make this test fail");
    }
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn main() {
    println!("Hello, test!");
}

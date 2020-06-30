/*
在 Rust 中，模块就像是 Java 中的类包装，
但是文件一开头就可以写一个主函数，这该如何解释呢？
每一个 Rust 文件的内容都是一个"难以发现"的模块。
让我们用两个文件来揭示这一点：
*/

// 在结尾使用;而不是{}，告诉编译器加载了一个同名文件作为此模块，这里就是front_of_house.rs文件。
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn main() {
    eat_at_restaurant();
}

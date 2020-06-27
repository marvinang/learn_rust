/*
 *
 * Option 是 Rust 标准库中的枚举类，这个类用于填补 Rust 不支持 null 引用的空白。
 * null 经常在开发者把一切都当作不是 null 的时候给予程序致命一击：
 * 毕竟只要出现一个这样的错误，程序的运行就要彻底终止。
 * Rust 在语言层面彻底不允许空值 null 的存在，但无奈null 可以高效地解决少量的问题，
 * 所以 Rust 引入了 Option 枚举类：
 *
 */

// enum Option<T> {
//    Some(T),
//    None,
// }


// 如果你的变量刚开始是空值，你体谅一下编译器，
// 它怎么知道值不为空的时候变量是什么类型的呢？
// 所以初始值为空的 Option 必须明确类型：

fn option_func() {
    // let opt = Option::Some("Hello");
    let opt: Option<&str> = Option::None;
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }
}

// Option 是一种特殊的枚举类，它可以含值分支选择：
fn option_match() {
    let t = Some(64);
    match t {
        Some(64) => println!("Yes"),
        _ => println!("No"),
    }
}

// if let 语法
// if let 语法可以认为是之区分两种情况的 match 语句的"语法糖".
// 对于枚举类依然适用：
fn if_let() {
    let i = 0;
    match i {
        0 => println!("zero"),
        _ => {},
    }

    if let 0 = i {
        println!("zero");
    }

    enum Book {
        Papery(u32),
        Electronic(String)
    }
    let book = Book::Electronic(String::from("url"));
    if let Book::Papery(index) = book {
        println!("Papery {}", index);
    } else {
        println!("Not papery book");
    }

}


fn main() {

    option_func();
    option_match();

    if_let();
}

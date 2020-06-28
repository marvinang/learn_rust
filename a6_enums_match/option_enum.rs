/*
 *
 * Option 是 Rust 标准库中的枚举类，这个类用于填补 Rust 不支持 null 引用的空白。
 * null 经常在开发者把一切都当作不是 null 的时候给予程序致命一击：
 * 毕竟只要出现一个这样的错误，程序的运行就要彻底终止。
 * Rust 在语言层面彻底不允许空值 null 的存在，但无奈null 可以高效地解决少量的问题，
 * 所以 Rust 引入了 Option 枚举类：
 *
 * 如果没有包含在Option中的类型不需要担心null值，也不需要处理，
 * 如果声明了Option则必须判断null值，并且需要手动处理包含的值。
 * 一般可以通过match操作来处理。
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
        }
        Option::None => {
            println!("opt is nothing");
        }
    }
}

// Opton<T> 定义了许多方法可以使用
fn define_optoin() {
    let x: Option<u32> = Some(20);
    assert_eq!(x.is_some(), true);
    let x: Option<u32> = None;
    assert_eq!(x.is_none(), true);

    // Some可以省略类型，编译器会自动判断
    let some_nubmer = Some(5);
    let some_string = Some("a string");
    // 如果使用None的时候，必须明确标识类型
    let absent_number: Option<i32> = None;
}

// Option 是一种特殊的枚举类，它可以含值分支选择：
fn option_match() {
    let t = Some(64);
    match t {
        Some(64) => println!("Yes"),
        _ => (),
    }

    // 如果用if let
    if let Some(64) = t {
        println!("Yes");
    }
}

// if let 语法
// if let 语法以一种简洁的方式匹配一种情况，而忽略其他情况。
// if let 语法可以认为是之区分两种情况的 match 语句的"语法糖".
// 对于枚举类依然适用：
fn if_let() {
    let i = 0;
    match i {
        0 => println!("zero"),
        _ => println!("none"),
    }

    // if let 还可以加一个else分支表示 _
    if let 0 = i {
        println!("zero");
    } else {
        println!("none");
    }

    enum Book {
        Papery(u32),
        Electronic(String),
    }
    let book = Book::Electronic(String::from("url"));
    // 可以加一个else分支
    if let Book::Papery(index) = book {
        println!("Papery {}", index);
    } else {
        println!("Not papery book");
    }
}

fn main() {
    option_func();
    define_optoin();
    option_match();
    if_let();
}

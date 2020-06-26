/*
 *
 * Rust 中的条件表达式必须是 bool 类型
 *
 * 如果有多个else if 分支，rust只执行第一个匹配上的，
 * 后续的条件不会执行
 *
 */

fn main() {
    f1();
    f2();
    triple();
}

fn f1() {
    let number = 3;
    if number < 5 {
        println!("条件为 true");
    } else {
        println!("条件为 false");
    }
}

// eles if
fn f2() {
    let a = 12;
    let b;
    if a > 0 {
        b = 1;
    } else if a < 0 {
        b = -1;
    } else {
        b = 0;
    };
    println!("b is {}", b);
}


// if是表达式，所以可以用来赋值
fn triple() {
    let a = 2;
    let number = if a > 0 { 1 } else { -1 };
    println!("number = {}", number);

    let condition = true;
    // 两个block返回的值必须是同一种类型
    let number = if condition {5} else {"six"};
    println!("The last number is {}", number);
}

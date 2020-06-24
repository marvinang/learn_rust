/*
 *
 * Rust 中的条件表达式必须是 bool 类型
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

fn f2() {
    let a = 12;
    let b;
    if a > 0 {
        b = 1;
    }  
    else if a < 0 {
        b = -1;
    }  
    else {
        b = 0;
    };
    println!("b is {}", b);
}

fn triple() {
    let a = 2;
    let number = if a > 0 { 1 } else { -1 };
    println!("number = {}", number);
}

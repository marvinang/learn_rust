

fn main() {
    while_loop();
    for_loop();
    loop_loop();
    loop_find();

}


// rust中没有c的三元for循环
fn while_loop() {
    let mut number = 1;
    while number != 4 {
        println!("{}", number);
        number += 1;
    }
    println!("exit");

}


// for 循环是最常用的循环结构，常用来遍历一个线性数据据结构（比如数组）
fn for_loop() {
    let a = [10,20,30,40];
    for i in a.iter() {
        println!("value= {}", i);
    }
    // or
    for i in 0..3 {
        println!("a[{}] = {}", i, a[i]);
    }

}


fn loop_loop() {
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == 'O' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    }
}


// loop 循环可以通过 break 关键字类似于 return 一样使整个循环退出并给予外部一个返回值。
// 这是一个十分巧妙的设计，因为 loop 这样的循环常被用来当作查找工具使用，
// 如果找到了某个东西当然要将这个结果交出去：
fn loop_find() {
    let s = ['r', 'u', 'n', 'o', '0', 'b'];
    let mut i = 0;
    let location = loop {
        let ch = s[i];
        if ch == '0' {
            break i;
        }
        i += 1;
    };
    println!("\'0\' index: {}", location);
}

/*
变量
1. rust默认的变量都是不可变的（immutable）, let x = 0;
2. 可变变量需要在前面加 mut
3. const

const vs mut
1. 不允许mut和const一起使用，const始终是不可变的
2. const变量使用const而不是let声明， 值必须有类型标注（type annotated)。
3. const可以在任何作用域声明
4. const变量的值仅可以在consant表达式被设置，而不是函数调用和运行时计算出来的值

shadowing, 遮蔽
let x = 5;
let x = x+1;
重复使用名称声明变量，后一个会遮蔽前一个变量

shadowing vs mut
shadowing时必须使用let重新声明
shadowing在转换后可以立即变为immutable变量

shadowing最大的作用是重复使用变量名改变变量类型
let space = " ";
let space = spaces.len();

let mut space = "  ";
space = spaces.len(); // error


数据类型
标量和复合类型

标量：
整型：
i8 u8
i16 u16
i32 u32
i64 u64
i128 u128
isize usize // 64位机器64bit, 32位机器32bit

数字表示：
出了byte literal外的所有数字都可以使用类型后缀， 比如 57u8, 和_作为分隔，如 1_1000,
Decimal  89_222
Hex      0xff
Octal    0o77
Binary   0b1111_0000
Byte(u8 only) b'A'

浮点型


布尔型

字符型


符合类型
Tuple

Array




方法

注释

控制流


*/

fn variabels() {
    let a = 123;
    println!("the valule is {}", a);
    // a = 30; error
    let mut a = 123;
    a = 456;
    println!("{}", a);
    let a = 40;
    println!("shadowing a value is {}", a);
    let mut a = 50;
    println!("shadowing a mut value is {}", a);

    let b = 123;
    let b = "456";

    const MAX_POINT: u32 = 1000_1000;
    // let const w = 10;
    // let c = 456; error
    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("the value of x is: {}", x);
}

fn func() {
    println!("Hello, func");
}

// 参数
fn hello(x: i32, y: i32) {
    println!(" x 的值为: {}", x);
    println!(" y 的值为: {}", y);
}

// 返回值
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    variabels();
    func();
    hello(10, 11);

    let x = 5;
    // 语句块, 函数体表达式
    let y = {
        let x = 3;
        // 这里没有分号，是语句块的值
        x + 1
    };
    hello(x, y);

    // 嵌套的函数
    fn five() -> i32 {
        5
    }
    println!("five() 的值 {}", five());
    println!("add(3, 2) = {}", add(3, 2));
}

/*
 *
 * Slice类型没有所有权。
 *
 * ownership, borrowing 和slice确保rust程序在编译时的内存安全，
 * let s ="hello world";
 * 那s的类型就是&str，右边称为字符串字面量literal，
 * 程序编译成二进制文件后，这个字符串会被保存在文件内部，所以s是特定位置字符串的引用，这就是为什么s是&str类型。
 *
 * &str由于保存在二进制文件内，所以&str类型不存在生命周期的概念，它是整个程序生命周期'static内都能访问的。
 *
 *
 */

// 获取第一个单词的最后索引
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn get_word() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear(); // empties the string s, ""
               // world is invalid index for string s;
}

fn get_word_2() {
    let mut s = String::from("hello world");
    let first = get_first_word(&s);
    s.clear(); // borrow the ownership here
               // println!("first world is {}", first); // error
}

// &str 为”String slice" 类型
fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// 字符串切片
// slice 不是引用全部字符串，而只是引用一部分。
// x..y 表示 [x, y) 的数学含义。.. 两边可以没有运算数
// ..y 等价于 0..y
// x.. 等价于位置 x 到数据结束
// .. 等价于位置 0 到结束
fn string_slice() {
    let s = String::from("broadcast");
    let part1 = &s[0..5];
    let part2 = &s[5..9];

    println!("{}={}+{}", s, part1, part2);

    // 被切片引用的字符串禁止更改其值：
    // s.push_str("yes"); // 错误
}

// 字符串字面量是Slice类型
fn str_slice() {
    // type of s here is &str
    // &str是一个指向特定的二进制
    // 这也是为什么字符串字面量不能修改的原因，&str是一个不可变引用
    let s = "Hello, world";
    let s1 = String::from("heloo");

    let r1 = first_word_improvement(&s1);
    let r2 = first_word_improvement(s);
    println!("==== {}, {}", r1, r2);
}

// 既可以接受 &String, 也可以接受 &str
// 编译器会自动转换
fn first_word_improvement(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// 非字符串切片
// 除了字符串以外，其他一些线性数据结构也支持切片操作，例如数组：
fn array_slice() {
    let arr = [1, 2, 3, 4];
    // 类型为 &[i32]
    let part = &arr[..2];
    for i in part.iter() {
        println!("{}", i);
    }
}

fn main() {
    get_word_2();
    str_slice();
    string_slice();
    array_slice();
}

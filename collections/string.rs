// 字符串类（String）到本章为止已经使用了很多。
// 本章主要介绍字符串的方法和 UTF-8 性质

fn main() {
    // 新建字符串
    let str = String::new();

    // 基础类型转换
    let one = 1.to_string();
    let float = 1.3.to_string();
    let slice = "slice".to_string();


    // utf-8
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");


    // 字符串追加
    let mut s = String::from("run");
    s.push_str(" boo");
    s.push('!');

    // 用 + 拼接字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // format! 宏
    let s = format!("{}-{}-{}", s1, s2, s3);

    // 字符串长度
    let len = s.len();

    let s = "hello你好";
    let len = s.chars().count();
    
    // 从字符串取单个字符
    let a = s.chars().nth(2);

}

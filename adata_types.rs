fn main() {
    /*
     * 整型
     * i8 u8
     * i16 u16
     * i32 u32
     * i64 u64
     * i128 u128
     * isize usize
     */

    /*
     * 进制
     * 98_222
     * 0xff
     * 0o77
     * ob1111_0000
     * b'A'
     */

    /* 
     * 浮点数
     * f32
     * f64
     * 默认为f64
     */

    let x = 2.0; // f64
    let y: f32 = 3.0;

    let sum = 5 + 10; // 加
    let difference = 95.5 - 4.3; // 减
    let product = 4 * 30; // 乘
    let quotient = 56.7 / 32.2; // 除
    let remainder = 43 % 5; // 求余


    /*
     * 布尔型
     * true
     * false
     */

    /*
     * 字符型
     * char
     *
     */

    /*
     *
     * 复合型
     * 元组 ()
     * 数组 []
     *
     */

    let tup: (i32, f64, u8) = (89, 6.4, 10);
    let (x, y, z) = tup;

    let a = [1, 2, 3, 4, 5];
    // a 是一个长度为 5 的整型数组
    let b = ["January", "February", "March"];
    // b 是一个长度为 3 的字符串数组
    let c: [i32; 5] = [1, 2, 3, 4, 5];
    // c 是一个长度为 5 的 i32 数组
    let d = [3; 5];
    // 等同于 let d = [3, 3, 3, 3, 3];
    let first = a[0];
    let second = a[1];
    // 数组访问

    // a[0] = 123; // 错误：数组 a 不可变
    let mut a = [1, 2, 3];
    a[0] = 4; // 正确
}

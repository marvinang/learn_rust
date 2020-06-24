


// 字符串切片
//
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


// 非字符串切片
// 除了字符串以外，其他一些线性数据结构也支持切片操作，例如数组：
fn array_slice() {
    let arr = [1,2,3,4];
    let part = &arr[..2];
    for i in part.iter() {
        println!("{}", i);
    }
}


fn main() {
    string_slice();
    array_slice();
}

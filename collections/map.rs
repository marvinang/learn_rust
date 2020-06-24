
// 映射表（Map）在其他语言中广泛存在。其中应用最普遍的就是键值散列映射表（Hash Map）


use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert("color", "red");
    map.insert("size", "10 m^2");

    map.insert("age", "a");

    // 如果你想"安全地插入"，就是在确认当前不存在某个键时才执行的插入动作，可以这样
    map.entry("color").or_insert("blue");

    // 在已经确定有某个键的情况下如果想直接修改对应的值，有更快的办法：
    if let Some(x) = map.get_mut(&"size") {
        *x = "b";
    }

    println!("{}", map.get("color").unwrap());

    for p in map.iter() {
        println!("{:?}", p);
    }

}

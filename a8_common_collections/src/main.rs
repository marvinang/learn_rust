/*
* String slice 类型为 str
* 字符串字面值类型为  str
* 但是一般使用的是其借用形式 &str
   let s = String::from("broadcast");
   let part1 = s[0..5];
   let part2 = &s[5..9];

在rust中一般提到strings指的是String和&str。
而且String和string slice都是使用UTF-8编码的。

除了以上两种还有需要string类型，比如OsString, OsStr, CString, CStr


String 其实是包装了Vec<u8>
*/

// 字符串类（String）到本章为止已经使用了很多。
// 本章主要介绍字符串的方法和 UTF-8 性质
fn strings() {
    // 新建字符串
    let str = String::new();

    // 使用字面量
    let data = "initial contents";
    // to_string() 是Display trait的方法
    let s = data.to_string();

    // 也可以简写
    let s = "Initail contents".to_string();

    // 使用from方法
    // utf-8
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    println!("hello={}", hello);
    let hello = String::from("こんにちは");
    println!("hello={}", hello);
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    println!("hello={}", hello);
    let hello = String::from("Olá");
    println!("hello={}", hello);
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // 字符串追加
    let mut s = String::from("run");
    s.push_str(" boo");
    // push是追加一个字符
    s.push('!');
    println!("s = {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    // 注意这里，push_str的参数是&str, 但是传递s2也没有问题，不会获取他的ownership
    // 这是应该是编译器自动转换了吧？？？？
    s1.push_str(s2);
    // s1.push_str(&s2);
    println!("s2 = {}", s2);

    // 基础类型转换
    let one = 1.to_string();
    let float = 1.3.to_string();
    let slice = "slice".to_string();

    // 用 + 拼接字符串
    // + 操作符其实调用的是add方法，add内部使用的是push_str 所以第二个参数必须是&str
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // let ss = s1.add(&s2);
    let s3 = s1 + &s2;
    println!("s3 = {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s = {}", s);

    // format! 宏
    let s = format!("{}-{}-{}", s, s2, s3);
    println!("s = {}", s);

    // String 索引
    // String不支持按照下标索引
    let hello = String::from("hola"); // 4 bytes
                                      // println!("hello[0]={}", &hello[0]);
    println!("len={}, chars={}", hello.len(), hello.chars().count());
    // 没一个unicode字面值用 2 bytes存储
    let hello = String::from("Здравствуйте");
    println!("len={}, chars={}", hello.len(), hello.chars().count());
    // grapheme clusters
    let hello = String::from("नमस्ते");
    println!("len={}, chars={}", hello.len(), hello.chars().count());

    // 字符串长度
    let len = s.len();
    // 字符数量
    let s = "hello你好";
    let len = s.chars().count();

    // 从字符串取单个字符
    let a = s.chars().nth(2);

    // slicing String
    // String支持切片
    let hello = String::from("Здравствуйте");
    // 注意如果阶段unicode字符，会panic
    let s = &hello[..4];
    println!("hello={}, &hell[..4]={}", hello, s);

    // iterating
    let hello = String::from("नमस्ते");
    for c in hello.chars() {
        println!("{}", c);
    }

    for c in hello.bytes() {
        println!("{}", c);
    }
}

/*
向量（Vector）是一个存放多值的单数据结构，该结构将相同类型的值线性的存放在堆中。
向量是线性表，在 Rust 中的表示是 Vec<T>。
向量的使用方式类似于列表（List），我们可以通过这种方式创建指定类型的向量：

*/

fn vector() {
    // 使用new新建一个指定类型并且空的Vector
    let v: Vec<i32> = Vec::new();
    // 如果不指定类型，必须添加数据可以让rust推断出类型
    let mut v1 = Vec::new();
    v1.push(5);
    println!("v1 = {:?}", v1);

    // 为了方便起见，rust提供了vec!宏。
    let mut vector = vec![1, 2, 4, 8];
    vector.push(16);
    vector.push(32);
    vector.push(64);
    println!("vector = {:?}", vector);

    // 也可以在宏创建的时候指定类型
    let mut v2: Vec<u8> = vec![8];
    // push更新
    v2.push(10);
    println!("v2 = {:?}", v2);

    let mut v3 = vec![1, 2, 4, 8];
    let mut v4 = vec![16, 32, 64];
    // 附加
    v3.append(&mut v4);
    println!("v4 = {:?}", v3);

    // 删除，作用域结束时，会自动删除
    {
        let v = vec![1, 2, 3, 4];
        println!("deleted v when this scope end, v = {:?}", v);
    }

    // 读取
    let mut v = vec![1, 2, 3, 4, 5];
    // 根据index读取
    let thrid = &v[2];
    println!("the third element is {}", thrid);
    // let does_not_exist = &v[100]; // panic
    let does_not_exist = v.get(100);
    // 使用get读取，这种读取比较安全，index读取可能会越界
    println!(
        "{}",
        match v.get(0) {
            Some(value) => value.to_string(),
            None => "None".to_string(),
        }
    );
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    // 下面的语句会报错，因为当v.push的时候，v[0]已经有一个不可变的引用了。
    // 原因是因为在v.push时有可能v会重新分配内存，这是first会指向一个deadllocated memory。
    // 租借规则禁止在这种情况发生。
    // let first = &v[0];
    // v.push(6);
    // println!("the first element is : {}", first);

    // 遍历
    let v = vec![100, 32, 57];
    for i in &v {
        println!("iterating item: {}", i);
    }

    // 遍历中改变值
    let mut v = vec![100, 32, 57];
    println!("origin v = {:?}", v);
    for i in &mut v {
        *i += 50;
    }
    println!("after iterating changed: v = {:?}", v);

    // vector中只能保存同一类型的值，如果要保存不同类型，可以使用enum进行包装。
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    println!("row = {:?}", row);

    for i in &row {
        match i {
            SpreadSheetCell::Int(v) => println!("value = {}", v),
            SpreadSheetCell::Text(v) => println!("value = {}", v),
            SpreadSheetCell::Float(v) => println!("value = {}", v),
        }
    }
}

// HashMap<k, V>
// 映射表（Map）在其他语言中广泛存在。其中应用最普遍的就是键值散列映射表（Hash Map）
// HashMap没有在prelude中，需要手动引入
use std::collections::HashMap;
fn hash_map() {
    // 新建空的hashmap, 编译器会自动推断类型
    // let mut map: HashMap<&str, &str> = HashMap::new();
    let mut map = HashMap::new();
    map.insert("color", "red");
    map.insert("size", "10 m^2");
    map.insert("age", "a");

    // 使用zip和collect组合map
    let teams = vec!["Blue", "Yello"];
    let scores = vec![10, 50];
    // 这里必须要annotation类型，<_,_> 意思是rust根据键和值自动推断类型
    let mut combine: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();

    // hashma ownership
    // 对于实现了Copy trait的类型，例如i32, 只是复制值到hashmap,
    // 但是对于owned values，例如String, 值会移动到hashmap,并且hashmap会获得这些值的ownership.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // println!("{} = {}", field_name, field_value);

    // 访问hashmap中的元素
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    // 使用get方法,get()的结果是一个Option
    let score = scores.get(&team_name);
    println!("Bule score is {:?}", score);
    if let Some(v) = score {
        println!("Bule score value is {}", v);
    }

    // 遍历
    println!("=========itering map==========");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    println!("=========itering end==========");

    // 修改map
    // 修改map有三种方式
    // 1. 替换旧值，保存新值
    // 2. 保留旧值，忽略新值
    // 3. 组合旧值和i新值

    // 覆写，直接insert()
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // 如果你想"安全地插入"，就是在确认当前不存在某个键时才执行的插入动作，可以这样
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // 在旧值的基础上修改
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert()其实返回一个插入值的可变引用，所以可以直接修改
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    let mut map = HashMap::new();
    map.insert("size", "abca");
    // 在已经确定有某个键的情况下如果想直接修改对应的值，有更快的办法：
    if let Some(x) = map.get_mut(&"size") {
        *x = "b";
    }
    println!("{:?}", map);
}

fn main() {
    println!("=========================== vector ========================");
    vector();
    println!("=========================== vector end ====================\n");
    println!("=========================== string ========================\n");
    strings();
    println!("=========================== string end ========================\n");
    println!("=========================== map ========================\n");
    hash_map();
    println!("=========================== map end ========================\n");
}

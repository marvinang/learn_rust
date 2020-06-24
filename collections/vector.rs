/*
向量（Vector）是一个存放多值的单数据结构，该结构将相同类型的值线性的存放在内存中。

向量是线性表，在 Rust 中的表示是 Vec<T>。

向量的使用方式类似于列表（List），我们可以通过这种方式创建指定类型的向量：

*/

fn main() {
    let mut vector = vec![1,2,4,8];
    vector.push(16);
    vector.push(32);
    vector.push(64);

    println!("{:?}", vector);


    let mut v1: Vec<i32> = vec![1, 2, 4, 8];
    let mut v2: Vec<i32> = vec![16, 32, 64];
    v1.append(&mut v2);
    println!("{:?}", v1);


    let mut v = vec![1, 2, 4, 8];
    println!("{}", match v.get(0) {
        Some(value) => value.to_string(),
        None => "None".to_string()
    });


    let v = vec![1, 2, 4, 8];
    println!("{}", v[1]);


    // 遍历
    let v = vec![100, 32, 57];
    for i in &v {
            println!("{}", i);
    }

    // 遍历中改变值
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

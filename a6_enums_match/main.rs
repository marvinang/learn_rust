/*
 *
 * 枚举类
 *
 */


fn book1() {
#[derive(Debug)]
    enum Book {
        Papery, Electronic
    }

    let book = Book::Papery;
    println!("{:?}", book);
}

// 可以为枚举类成员添加元组属性描述：
fn book2() {
#[derive(Debug)]
    enum Book {
        Papery(u32),
        Electronic(String),
    }

    let book = Book::Papery(1001);
    let ebook = Book::Electronic(String::from("url://..."));

    println!("{:?}", book);
}

// 如果你想为属性命名，可以用结构体语法：
// 虽然可以如此命名，但请注意，并不能像访问结构体字段一样访问枚举类绑定的属性。
// 访问的方法在 match 语法中。
fn book3() {
    enum Book {
        Papery { index: u32 },
        Electronic { url: String },
    }
    let book = Book::Papery{index: 1001};
}

// match语法
// rust中没有switch语法

/*
 match 枚举类实例 {
    分类1 => 返回值表达式,
    分类2 => 返回值表达式,
    ...
}
*/

fn func_match() {
    enum Book {
        Papery {index: u32},
        Electronic {url: String},
    }

    let book = Book::Papery{index: 1001};
    let ebook = Book::Electronic{url: String::from("url...")};

    match book {
        Book::Papery { index } => {
            println!("Papery book {}", index);
        },
        Book::Electronic { url } => {
            println!("E-book {}", url);
        }
    }
}

// match 除了能够对枚举类进行分支选择以外，
// 还可以对整数、浮点数、字符和字符串切片引用（&str）类型的数据进行分支选择。
// 其中，浮点数类型被分支选择虽然合法，但不推荐这样使用，因为精度问题可能会导致分支错误。

// 对非枚举类进行分支选择时必须注意处理例外情况，即使在例外情况下没有任何要做的事 .
// 例外情况用下划线 _ 表示：
fn other_match() {
    let t = "abcd";
    match t {
        "abc" => println!("Yes"),
        _ => { println!("others")},
    }
}
fn main() {
    book1();
    book2();
    book3();
    func_match();
    other_match();
}

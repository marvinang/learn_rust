/*
 *
 * 枚举类
 *
 */

fn book1() {
    #[derive(Debug)]
    enum Book {
        Papery,
        Electronic,
    }

    let book = Book::Papery;
    let book1 = Book::Electronic;
    println!("{:?}, {:?}", book, book1);
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

    println!("=={:?}", book);
    println!("=={:?}", ebook);
}

fn ip() {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("ipv4= {:?}", home);
    println!("ipv6= {:?}", loopback);

    // 标准库中的定义
    struct Ipv4Addr {}

    struct Ipv6Addr {}

    // enum variant中可以放置任何类型，Strings, numberic types, structs, enum
    enum IP {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
}

fn other() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 }, // anonymous struct
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // 类似以下定义
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32);
}

// 如果你想为属性命名，可以用结构体语法：
// 虽然可以如此命名，但请注意，并不能像访问结构体字段一样访问枚举类绑定的属性。
// 访问的方法在 match 语法中。
fn book3() {
    enum Book {
        Papery { index: u32 },
        Electronic { url: String },
    }
    let book = Book::Papery { index: 1001 };
}

// Enum和struct一样也可以定义方法
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("enum method:: ===message is {:?}", self);
    }
}

// ============== Option enum 查看

//====================================================== match语法
// rust中没有switch语法, 但是有match语法，
// match可以匹配字面值，变量名称，通配符和其他值
//

/*
 match 枚举类实例 {
    分类1 => 返回值表达式,
    分类2 => 返回值表达式,
    ...
}
*/

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 也可以部分匹配
        Coin::Quarter(state) => {
            println!("====> State quarter from {:?}!", state);
            25
        }
    }
}

fn func_match() {
    enum Book {
        Papery { index: u32 },
        Electronic { url: String },
    }

    let book = Book::Papery { index: 1001 };
    let ebook = Book::Electronic {
        url: String::from("url..."),
    };

    match book {
        Book::Papery { index } => {
            println!("Papery book {}", index);
        }
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
        _ => println!("others"),
    }

    let some_u8 = 0u8;
    match some_u8 {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // () 空的tuple， 在这里什么也不会发生
    }
}

fn match_option() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("====> option {:?}, {:?}", six, none);
}

fn main() {
    book1();
    book2();
    ip();
    book3();
    let m = Message::Write(String::from("hello"));
    m.call();
    value_in_cents(Coin::Quarter(UsState::Alaska));
    func_match();
    other_match();
    match_option();
}

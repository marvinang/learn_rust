/*
 * 模式匹配
 * 又来匹配值的结构
 *

模式是 Rust 中特殊的语法，它用来匹配类型中的结构，无论类型是简单还是复杂。
结合使用模式和 match 表达式以及其他结构可以提供更多对程序控制流的支配权。
模式由如下一些内容组合而成：
  1. 字面值
  2. 解构的数组、枚举、结构体或者元组
  3. 变量
  4. 通配符
  5. 占位符
  6. 这些部分描述了我们要处理的数据的形状，接着可以用其匹配值来决定程序是否拥有正确的数据来运行特定部分的代码。

我们通过将一些值与模式相比较来使用它。如果模式匹配这些值，我们对值部分进行相应处理。
回忆一下第六章讨论 match 表达式时像硬币分类器那样使用模式。
如果数据符合这个形状，就可以使用这些命名的片段。如果不符合，与该模式相关的代码则不会运行。

本章是所有模式相关内容的参考。我们将涉及到使用模式的有效位置，refutable 与 irrefutable 模式的区别，
和你可能会见到的不同类型的模式语法。
在最后，你将会看到如何使用模式创建强大而简洁的代码。
 */

// ============== 所有可能会用到模式的位置 ===============

// ----- match -----
// match 表达式必须是 穷尽（exhaustive）的，意为 match 表达式所有可能的值都必须被考虑到。
// 一个确保覆盖每个可能值的方法是在最后一个分支使用捕获所有的模式：
// 比如，一个匹配任何值的名称永远也不会失败，因此可以覆盖所有匹配剩下的情况。

// 有一个特定的模式 _ 可以匹配所有情况，不过它从不绑定任何变量。
// 这在例如希望忽略任何未指定值的情况很有用。
//
// match value {
//     a => exp,
//     a => exp,
//     a => exp,
// }

// ----- if let 表达式 --------
// 第六章讨论过了 if let 表达式，以及它是如何主要用于编写等同于只关心一个情况的 match 语句简写的。
// if let 可以对应一个可选的带有代码的 else 在 if let 中的模式不匹配时运行。
//
// 示例 18-1 展示了也可以组合并匹配 if let、else if 和 else if let 表达式。
// 这相比 match 表达式一次只能将一个值与模式比较提供了更多灵活性；
// 一系列 if let、else if、else if let 分支并不要求其条件相互关联。
//
// if let 表达式的缺点在于其穷尽性没有为编译器所检查，而 match 表达式则检查了。
// 如果去掉最后的 else 块而遗漏处理一些情况，编译器也不会警告这类可能的逻辑错误。
fn if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the backgroud", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

// -------- while let 添加循环
// 一个与 if let 结构类似的是 while let 条件循环，
// 它允许只要模式匹配就一直进行 while 循环。
//
// while 循环只要 pop 返回 Some 就会一直运行其块中的代码。
// 一旦其返回 None，while 循环停止。
fn while_let() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

// ------- for 循环 --
// for 循环是 Rust 中最常见的循环结构，
// 不过还没有讲到的是 for 可以获取一个模式。
// 在 for 循环中，模式是 for 关键字直接跟随的值，正如 for x in y 中的 x。
fn for_pattern() {
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

// ------- let 语句
// let 语句更为正式的样子如下：
// let PATTERN = EXPRESSION;
// 像 let x = 5; 这样的语句中变量名位于 PATTERN 位置，
// 变量名不过是形式特别朴素的模式。
// 我们将表达式与模式比较，并为任何找到的名称赋值。
// 所以例如 let x = 5; 的情况，x 是一个模式代表 “将匹配到的值绑定到变量 x”。
// 同时因为名称 x 是整个模式，这个模式实际上等于 “将任何值绑定到变量 x，不管值是什么”。
fn let_pattern() {
    let a = 6;
    let (x, y, z) = (1, 2, 3);
}

// -------- 函数参数
// 函数参数也可以是模式。
// 因为如第十三章所讲闭包类似于函数，也可以在闭包参数列表中使用模式。
fn foo(x: i32) {}
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({},{})", x, y);
}

// =========================== irrefutable ================
// 现在我们见过了很多使用模式的方式了，不过模式在每个使用它的地方并不以相同的方式工作；
// 在一些地方，模式必须是 irrefutable 的，意味着他们必须匹配所提供的任何值。
// 在另一些情况，他们则可以是 refutable 的。
//
// 模式有两种形式：refutable（可反驳的）和 irrefutable（不可反驳的）。
// 能匹配任何传递的可能值的模式被称为是 不可反驳的（irrefutable）。
// 一个例子就是 let x = 5; 语句中的 x，因为 x 可以匹配任何值所以不可能会失败。
// 对某些可能的值进行匹配会失败的模式被称为是 可反驳的（refutable）。
// 一个这样的例子便是 if let Some(x) = a_value 表达式中的 Some(x)；
// 如果变量 a_value 中的值是 None 而不是 Some，那么 Some(x) 模式不能匹配。
//
// 函数参数、 let 语句和 for 循环只能接受不可反驳的模式，因为通过不匹配的值程序无法进行有意义的工作。
// if let 和 while let 表达式被限制为只能接受可反驳的模式，
// 因为根据定义他们意在处理可能的失败：条件表达式的功能就是根据成功或失败执行不同的操作。

// 通常我们无需担心可反驳和不可反驳模式的区别，不过确实需要熟悉可反驳性的概念，
// 这样当在错误信息中看到时就知道如何应对。
// 遇到这些情况，根据代码行为的意图，需要修改模式或者使用模式的结构。

// ========================== 所有的模式语法 ======================

// -------- 匹配字面值 ---
fn literal_pattern() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// -------- 匹配命名变量 ---
// 命名变量是匹配任何值的不可反驳模式，这在之前已经使用过数次。
// 然而当其用于 match 表达式时情况会有些复杂。
// 因为 match 会开始一个新作用域，
// match 表达式中作为模式的一部分声明的变量会覆盖 match 结构之外的同名变量，与所有变量一样。
fn match_pattern() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // 引入了新的变量，会覆盖外部的变量y
        // 为了创建能够比较外部 x 和 y 的值，而不引入覆盖变量的 match 表达式，
        // 我们需要相应地使用带有条件的匹配守卫（match guard）。
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Defautl case, x= {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
}

// ------- 多个模式 ---
fn multi_pattern() {
    let x = 1;
    match x {
        1 | 2 => println!("one ro two"),
        3 => println!("three"),
        _ => println!("others"),
    }
}

// -------- 通过..= 匹配值的范围 ---
// 范围只允许用于数字或 char 值，因为编译器会在编译时检查范围不为空。
// char 和 数字值是 Rust 仅有的可以判断范围是否为空的类型。
fn scope_pattern() {
    let x = 5;
    // 如果 x 是 1、2、3、4 或 5，第一个分支就会匹配。
    // 这相比使用 | 运算符表达相同的意思更为方便；
    // 相比 1..=5，使用 | 则不得不指定 1 | 2 | 3 | 4 | 5。
    // 相反指定范围就简短的多，特别是在希望匹配比如从 1 到 1000 的数字的时候！
    match x {
        1..=5 => println!("one throug five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

// ------- 解构并分解值 ----
// 可以使用模式来解构结构体、枚举、元组和引用，
// 以便使用这些值的不同部分。
struct Point {
    x: i32,
    y: i32,
}

// 结构体
fn destruct_pattern() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

// 枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn enum_pattern() {
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

// 结构嵌套的结构体和枚举
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Messag {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
fn nested_pattern() {
    let msg = Messag::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Messag::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Messag::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}

// 解构结构体和元祖
fn tuple_pattern() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

// ================== 忽略模式中的值 =====================
// 有时忽略模式中的一些值是有用的，比如 match 中最后捕获全部情况的分支实际上没有做任何事，
// 但是它确实对所有剩余情况负责。
// 有一些简单的方法可以忽略模式中全部或部分值：
//  1. 使用 _ 模式（我们已经见过了），在另一个模式中使用 _ 模式，
//  2. 使用一个以下划线开始的名称，
//  3. 或者使用 .. 忽略所剩部分的值。
//  让我们来分别探索如何以及为什么要这么做。

// ------ 使用 _ 忽略整个值 -----
// 我们已经使用过下划线（_）作为匹配但不绑定任何值的通配符模式了。
// 虽然 _ 模式作为 match 表达式最后的分支特别有用，也可以将其用于任意模式，
// 包括函数参数中，如下所示：
//
// 大部分情况当你不再需要特定函数参数时，最好修改签名不再包含无用的参数。
// 在一些情况下忽略函数参数会变得特别有用，比如实现 trait 时，
// 当你需要特定类型签名但是函数实现并不需要某个参数时。
// 此时编译器就不会警告说存在未使用的函数参数，就跟使用命名参数一样。
fn parameters_pattern(_: i32, y: i32) {
    println!("This code only uses the y paramter: {}", y);
}

// ---------- 使用嵌套的_忽略部分值 ---------
// 也可以在一个模式内部使用_ 忽略部分值，
// 例如，当只需要测试部分值但在期望运行的代码中没有用到其他部分时
fn part_ignore_pattern() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customeized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    // 也可以在一个模式中的多处使用下划线来忽略特定值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
    }
}

// ----- 通过在名字前以一个下划线开头来忽略未使用的变量 ----
// 如果你创建了一个变量却不在任何地方使用它,
// Rust 通常会给你一个警告，因为这可能会是个 bug。
// 但是有时创建一个还未使用的变量是有用的，比如你正在设计原型或刚刚开始一个项目。
// 这时你希望告诉 Rust 不要警告未使用的变量，
// 为此可以用下划线作为变量名的开头。
fn used_variables() {
    let _x = 5;
    let y = 10;
}
// 注意, 只使用 _ 和使用以下划线开头的名称有些微妙的不同：
// 比如 _x 仍会将值绑定到变量，而 _ 则完全不会绑定。
// 为了展示这个区别的意义，示例 18-21 会产生一个错误。

// --------- 用..忽略剩余值 -----
// 对于有多个部分的值，可以使用 .. 语法来只使用部分并忽略其它值，
// 同时避免不得不每一个忽略值列出下划线。
// .. 模式会忽略模式中剩余的任何没有显式匹配的值部分。
// 在示例 18-23 中，有一个 Point 结构体存放了三维空间中的坐标。
// 在 match 表达式中，我们希望只操作 x 坐标并忽略 y 和 z 字段的值：
struct P {
    x: i32,
    y: i32,
    z: i32,
}

fn left_ignore_pattern() {
    let origin = P {x:0, y:0, z:0};
    match origin {
        P { x, ..} => println!("x is {}", x),
    }

    let nubmer = (2,4,8,16,32);
    match nubmer {
        (first, .., last) => {
            println!("Some nubmers: {}, {}", first, last);
        },
    }
    // 然而使用 .. 必须是无歧义的。如果期望匹配和忽略的值是不明确的，Rust 会报错。
    // match number {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     },
    // }
}


 
// =================== 匹配守卫提供的额外条件 ======
// 匹配守卫（match guard）是一个指定于 match 分支模式之后的额外 if 条件，它也必须被满足才能选择此分支。
// 匹配守卫用于表达比单独的模式所能允许的更为复杂的情况。
fn match_guard() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

// 们提到可以使用匹配守卫来解决模式中变量覆盖的问题，
// 那里 match 表达式的模式中新建了一个变量而不是使用 match 之外的同名变量。
// 新变量意味着不能够测试外部变量的值。
// 示例展示了如何使用匹配守卫修复这个问题。
fn match_guard_repair() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n= {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
}


// ========== @ 绑定 ============
// at 运算符（@）允许我们在创建一个存放值的变量的同时测试其值是否匹配模式。
enum Messa {
    Hello {id: i32},
}

fn at_binding() {
    let msg = Messa::Hello { id: 5};
    match msg {
        Messa::Hello {id: id_variable @ 3..=7} => {
            println!("Found an id in range: {}", id_variable)
        },
        Messa::Hello {id: 10..=12} => {
            println!("Found an id in another range")
        },
        Messa::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}


// ======================= 总结 =======================
// 模式是 Rust 中一个很有用的功能，它帮助我们区分不同类型的数据。
// 当用于 match 语句时，Rust 确保模式会包含每一个可能的值，否则程序将不能编译。
// let 语句和函数参数的模式使得这些结构更强大，可以在将值解构为更小部分的同时为变量赋值。
// 可以创建简单或复杂的模式来满足我们的要求。




fn main() {
    if_let();
    while_let();
    for_pattern();
    let_pattern();
    literal_pattern();
    match_pattern();
    multi_pattern();
    scope_pattern();
    destruct_pattern();
    enum_pattern();
    nested_pattern();
    tuple_pattern();
    parameters_pattern(32, 22);
    part_ignore_pattern();
    used_variables();
    left_ignore_pattern();
    at_binding();
}

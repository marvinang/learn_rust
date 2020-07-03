/*
Rust 生命周期机制是与所有权机制同等重要的资源管理机制。

lifetimes ensure that refernces are valid as long as we need them to be.
lifetimes确保我们在使用引用的过程中是有效的。
rust中的每一个引用都有生命周期，是该引用有效的范围。大多数情况下，lifetim是编译器隐式推断的。
当引用的lifetimes可能有多种不同的联系时，应该指定lifetims,
lifetime通过generic lifetime parameters来注释，确保引用在使用期间绝对有效。

之所以引入这个概念主要是应对复杂类型系统中资源管理的问题。
引用是对待复杂类型时必不可少的机制，毕竟复杂类型的数据不能被处理器轻易地复制和计算。


作用：
1. lifetime主要的作用是阻止dangling引用。
2. lifetime语法与连接传入参数的和返回值的生命周期有关，一旦建立链接，编译器有足够的信息来进行内存安全操作，
   并禁止可能会导致悬空指针或其他违反内存安全性的操作。




/// ===================================== The Borrow Checker ==========================
/// Rust编译器使用borrow checker来比较作用域来判断是否所有的borrows是有效的。
fn main() {
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
}
r has a lifetime of 'a but that it refers to memory with a lifetime of 'b.
The program is rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference.


right!!!
fn main() {
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
}
Here, x has the lifetime 'b, which in this case is larger than 'a.
This means r can reference x because Rust knows that the reference in r will always be valid while x is valid.


*/

fn null_value() {
    let r: i32;
    // println!("{}", r); // error, uninitialized
}

// fn dangling() {
//     let r;
//     {
//         let x = 5;
//         r = &x
//     } // x drop
//     println!("r: {}", r);
// }

//===================== 泛型在函数中的生命周期
/*
 *
 一直以来我们都在结构体中使用 String 而不用 &str，我们用一个案例解释原因：

fn longer(s1: &str, s2: &str) -> &str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}
longer 函数取 s1 和 s2 两个字符串切片中较长的一个返回其引用值。
但只这段代码不会通过编译，原因是返回值引用可能会返回过期的引用：
原因是编译器不能确定返回值引用的lifetime, 因为borrow cheker无法分析返回值和x，y的引用关系。



==生命周期注释语法==
生命周期注释是描述引用生命周期的办法, 描述了多个引用生命周期的关系。
虽然这样并不能够改变引用的生命周期，但可以在合适的地方生命两个引用的生命周期一致。
生命周期注释用单引号'开头，跟着一个小写单词(非常短)，一般是一个小写字母，
生命周期注释跟在&后面，然后是空格，再后面是引用类型：
 &i32        // 常规引用
 &'a i32     // 含有生命周期注释的引用
 &'a mut i32 // 可变型含有生命周期注释的引用


单个的生命周期注释没有意义，是用来标记多个引用之间的关系的。

*/

//=========================== 在方法签名上使用lifetime annotation ======================
// 下面的意思是所有的参数s1, s2和返回值应该在'a生命周期内有效。
// 这个约束是我们想要rust遵守的，但是我们指定生命周期并不会改变任何传入参数和返回值的生命周期。
// 只是让borrow checker遵守这个约束拒绝任何违反约束的传入值。
// 所以下面的loger函数并不需要只是x和y的实际的生命周期，只需要某些作用域可以替代'a来满足函数签名就可以了。
fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}

fn call_longer() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("short");
        // 返回值的生命周期至少和传入参数的最短生命周期的相同。
        let result = longer(string1.as_str(), string2.as_str());
        println!("=== the longer stting is {}", result);
    }
    // 2
    let r;
    {
        let s1 = "rust";
        let s2 = "ecmascript";
        r = longer(s1, s2);
        println!("{} is longer", r);
    }
    // 为什么还可以读到r, str类型没有被自动drop吗？
    // 参看a4 slice, str没有生命周期，整个程序的生命周期'static都可以访问
    println!("{}", r);
    // 3
    // let result;
    // {
    //     let string2 = String::from("short");
    //     // 这里返回值的生命周期长于和传入参数最短生命周期。
    //     result = longer(string1.as_str(), string2.as_str());
    // }
    //
    // println!("=== the longer string ={}", result); //error
}

//================================
// right
fn longer_1<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// error
// fn longer_2<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("long string");
//     result.as_str()
// }

//============================== Lifetime Annotation in Struct Definitions ====================
// 之前我们在结构体中定义的字段都是 owned types,结构体中也可以包含引用，但是需要声明生命周期。
//
// 这里的'a声明的含义是：ImportantExecpt的实例的生命周期不能比part引用长。
#[derive(Debug)]
struct ImportantExecpt<'a> {
    part: &'a str,
}

fn importantexecpt() {
    let novel = String::from("Call me Ishmael, Some years ago..");
    let first_sentence = novel.split(".").next().expect("Could not find a '.");
    let i = ImportantExecpt {
        part: first_sentence,
    };
    println!("the first sentect is {:?}", i);

    // error!
    // let s;
    // {
    //     let a = String::from("invalid");
    //     s = ImportantExecpt { part: a.as_str() }
    // }
    // println!("the first sentect is {:?}", s);
}

//================================== Lifetime Elision ========================
// 编码进rust的用于分析引用的规则成为 lifetime elision rules
// 省略生命周期描述
// 在早期的rust版本(pre-1.0), 引用必须加生命周期声明，但是太繁琐了，
// 后来发现而且很多都有固定的模式，可以预测出来，所以开发者内置了这些模式到编译器，
// 编译器就可以推断出这些生命周期，而不需要显示声明了。
//
// 在函数和方法上的参数生命周期称为: input lifetime
// 在返回值上的生命和走其称为: output lifetime
// Rust使用三条规则来猜测生命周期，第一条规则应用于input lifetime, 第二条和第三条应用与output lifetime,
// 如果应用后无法猜测出生命周期， 则会报错。
// 这三条规则使用于fn和impl
//
// 规则一： 每个传入参数都会获得自己的生命周期参数。
// 规则二:  如果只有一个确定生命周期输入参数，则生命周期参数会赋给所有的返回值。
// 规则三： 如果输入参数有多个，并且其中一个是&self或&mut self，则吧self的生命周期参数赋给所有的返回值。

// right
fn first(s: &str) -> &str {
    s
}
// wrong
// fn second(x: &str, y: &str) -> &str {
//     x
// }

//============================ Lifetime Annotation in Method Definations =============
// 始终必须在impl关键字之后声明结构字段的生命周期名称，
//
// 然后在结构名称之后使用，因为这些生命周期是结构类型的一部分。
impl<'a> ImportantExecpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn call_method() {
    let i = ImportantExecpt { part: "100" };
    i.level();
    i.announce_and_return_part("hah");
}

//======================== The Static Lifetime ======================
// 'static是一个特殊的生命周期。
// 它的含义是相关的引用在整个程序执行期间都是有效的。
// 例如所有的string literals拥有'static生命周期。
fn static_str() {
    let s: &'static str = "I have a cat!";
    // equals to
    let s = "I have a cat!";
}

//======================= Generic Type Parameters, Trait Bounds, and Lifetimes Togather ======
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn call_longest() {
    let a = String::from("long----");
    let b = String::from("short");
    let c = longest_with_an_announcement(a.as_str(), b.as_str(), "~~~~~");
    println!("===== call_longest is {}", c);
}

fn main() {
    null_value();
    call_longer();
    importantexecpt();
    call_method();
    static_str();
    call_longest();
}

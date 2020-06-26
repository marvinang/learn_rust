/*
 * 所有权是rust最特殊的特性，它是rust没有垃圾回收机制
 * 而确保内存安全的保证
 *
 * 所有权相关的特性： borrowing, slices, memory data layout
 *
 * 堆和栈
 * 存储在栈上数据必须有已知、固定的大小，
 * 在编译期间如果数据的大小未知或者大小可能改变，就一定保存在堆上。
 *
 * 所有权的就是为了管理堆上的数据。
 *
 *
 *
 * 所有权对大多数开发者而言是一个新颖的概念，
 * 它是 Rust 语言为高效使用内存而设计的语法机制。
 * 所有权概念是为了让 Rust 在编译阶段更有效地分析内存资源的有用性
 * 以实现内存管理而诞生的概念。
 *
 * 所有权有以下三条规则：

 1. Rust 中的每个值都有一个变量，称为其所有者。
 2. 一次只能有一个所有者。
 3. 当所有者不在程序运行范围时，该值将被删除。

 这三条规则是所有权概念的基础。

*/

// 变量范围
// 变量范围是变量的一个属性，其代表变量的可行域，
// 默认从声明变量开始有效直到变量所在域结束
fn scope() {
    // 在声明以前，变量 s 无效
    let s = "runoob";
    // 这里是变量 s 的可用范围

    // -------------------------------
    {
        let a = "hello";
    }
} // 变量范围已经结束，变量 s 无效

// 内存分配
// 不像c使用free显示释放分配在堆中的对象
// Rust 之所以没有明示释放的步骤是因为在变量范围结束的时候，
// Rust 编译器[自动]添加了调用释放资源函数的步骤,
// 其实是自动调用了一个特殊的方法 drop(), drop会自动地在}处调用。

// 这种机制看似很简单了：它不过是帮助程序员在适当的地方添加了一个释放资源的函数调用而已。
// 但这种简单的机制可以有效地解决一个史上最令程序员头疼的编程问题。

// 变量与数据交互的方式
// 变量与数据交互方式主要有移动（Move）和克隆（Clone）两种：

fn data() {
    // ================= 移动 ==================

    /*
     *
     * 这个程序将值 5 绑定到变量 x，然后将 x 的值复制并赋值给变量 y。
     * 现在栈中将有两个值 5。此情况中的数据是"基本数据"类型的数据，
     * 不需要存储到堆中，仅在栈中的数据的"移动"方式是直接复制，
     * 这不会花费更长的时间或更多的存储空间。"基本数据"类型有这些：

     所有整数类型，例如 i32 、 u32 、 i64 等。
     布尔类型 bool，值为 true 或 false 。
     所有浮点类型，f32 和 f64。
     字符类型 char。
     仅包含以上类型数据的元组（Tuples）。

    */
    // 交互数据在栈中
    let x = 5;
    let y = x;

    // 交互数据在堆中
    let s1 = String::from("hello");
    let s2 = s1;

    // String 的数据结构
    //  | name  | value |     | index | value |
    //  | ptr   |       |---->|  0    |  h    |
    //  | len   |  5    |     |  1    |  e    |
    //  |capacity |  5  |     |  2    |  l    |
    //                        |  3    |  l    |
    //                        |  4    |  o    |
    //
    // 第一步产生一个 String 对象，值为 "hello"。
    //      其中 "hello" 可以认为是类似于长度不确定的数据，需要在堆中存储。
    // 第二步的情况略有不同
    //
    // 两个 String 对象在栈中，每个 String 对象都有一个指针指向堆中的 "hello" 字符串。
    // 在给 s2 赋值时，只有栈中的数据被复制了，堆中的字符串依然还是原来的字符串。
    //
    // 前面我们说过，当变量超出范围时，Rust 自动调用释放资源函数并清理该变量的堆内存。
    // 但是 s1 和 s2 都被释放的话堆区中的 "hello" 被释放两次，这是不被系统允许的。
    // 为了确保安全，在给 s2 赋值时 s1 已经无效了。
    // 没错，在把 s1 的值赋给 s2 以后 s1 将不可以再被使用。下面这段程序是错的：
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // 错误！s1 已经失效

    // ================= 克隆 ==============
    // Rust会尽可能地降低程序的运行成本，所以默认情况下，长度较大的数据存放在堆中，
    // 且采用移动的方式进行数据交互。
    // 但如果需要将数据单纯的复制一份以供他用, 可以使用数据的第二种交互方式——克隆。
    let s3 = String::from("hello me");
    let s4 = s3.clone();
    println!("s3 = {}, s4={}", s3, s4);

    // 这里是真的将堆中的 "hello" 复制了一份，所以 s1 和 s2 都分别绑定了一个值，
    // 释放的时候也会被当作两个资源。
    // 当然，克隆仅在需要复制的情况下使用，毕竟复制数据会花费更多的时间。
}

/*
 * 涉及函数的所有权机制
 * 如果将变量当作参数传入函数，那么它和移动的效果是一样的。
 *
 */
fn function() {
    let s = String::from("hello");
    // s 被声明有效

    takes_ownership(s);
    // s 的值被当作参数传入函数
    // 所以可以当作 s 已经被移动，从这里开始已经无效
    // println!("{}", s);

    let x = 5;
    // x 被声明有效

    makes_copy(x);
    // x 的值被当作参数传入函数
    // 但 x 是基本类型，依然有效
    // 在这里依然可以使用 x 却不能使用 s
    println!("{}", x);
} // 函数结束, x 无效, 然后是 s. 但 s 已被移动, 所以不用被释放

fn takes_ownership(some_string: String) {
    // 一个 String 参数 some_string 传入，有效
    println!("{}", some_string);
} // 函数结束, 参数 some_string 在这里释放

fn makes_copy(some_integer: i32) {
    // 一个 i32 参数 some_integer 传入，有效
    println!("{}", some_integer);
} // 函数结束, 参数 some_integer 是基本类型, 无需释放

/*
 * 函数返回值的所有权机制
 * 被当作函数返回值的变量所有权将会被移动出函数并返回到调用函数的地方，
 * 而不会直接被无效释放。
 *
 */
fn func_return() {
    let s1 = gives_ownership();
    // gives_ownership 移动它的返回值到 s1

    let s2 = String::from("hello");
    // s2 被声明有效

    let s3 = takes_and_gives_back(s2);
    // s2 被当作参数移动, s3 获得返回值所有权

    let s5 = String::from("hello");
    let (s6, len) = calculate_length(s5);
    println!("The length of '{}' is {}.", s6, len);
} // s3 无效被释放, s2 被移动, s1 无效被释放.

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // some_string 被声明有效

    some_string
    // some_string 被当作返回值移动出函数
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string 被声明有效

    a_string // a_string 被当作返回值移出函数
}

// 如果我们只想使用值，而不取所有权，那就必须把参数返回，可以返回tuple
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// 但是以上的方式非常的繁琐，我们可是使用引用来解决这个问题

/*
 * 引用与租借
 *
 * 引用（Reference）是 C++ 开发者较为熟悉的概念。
 * 如果你熟悉指针的概念，你可以把它看作一种指针。
 * 实质上"引用"是变量的间接访问方式。
 *
 * & 运算符可以取变量的"引用"。
 * 当一个变量的值被引用时，变量本身不会被认定无效。
 * 因为"引用"并没有在栈中复制变量的值：
 *
 * 引用只是指向被引用的值，而没有获取变量的ownership
 *
 */
fn pointer() {
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("s1 is {}, s2 is {}", s1, s2);

    let mut s = String::from("hello");
    let r1 = &mut s;
    // 不能在同一个作用域中多次可变引用
    //let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    can_change(&mut s);
    println!("String changed {}", s);
}

// 借用： 使用应用最为函数参数为borrowing。
// 当s返回时，由于没有所有权，所以不会删除s引用的值，
// 也不需要返回它来归还所有权。
fn caculate_length(s: &String) -> usize {
    s.len()
}

// 不能对引用进行修改
fn cannot_change(s: &String) {
    // s.push_str(", world");
}

// 可变的引用可以修改值
// 但是可变引用有一个巨大的限制：
// 在一个指定的作用域中对一个指定的数据只能有一个可变引用
//
// 可变引用不能重复原因是rust在编译期间阻止了数据竞争，
// 数据竞争类似竞争条件，在以下三种情况出现时会发生：
//
// 1. 两个或者更多的指针同时访问同一个数据
// 2. 至少有一个指针被用于写数据
// 3. 没有任何机制来同步访问的数据
//
//
//
fn can_change(s: &mut String) {
    s.push_str(", world");
    multi_references();
}

// 引入其他域进行多重引用
fn multi_references() {
    let mut s = String::from("hello");
    // let s1 = &mut s; // wrong
    {
        let r1 = &mut s;
    } // r1的作用域结束了
    let s3 = &mut s;
}

// 可变和不可便混合引用
fn hybird_references() {
    let mut s = String::from("hello");
    let s1 = &s;
    let s2 = &s;
    let s3 = &mut s; // error
                     // 如何在s3后面s1和s2不再使用，则上面不会有问题
    println!("{}", s3);
    // println!("{}, {}, {}", s1, s2, s3);
}

// 引用参数传递和上面的道理一样
fn ref_parames() {
    let s1 = String::from("hello");
    let s2 = &s1;
    let len = caculate_length(s2);
    println!("the s2 = {}", s2);
    println!("the length of '{}' is {}", s1, len);
}

/*
 * 引用不会获得值的所有权。
 * 引用只能租借（Borrow）值的所有权。
 * 引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，
 * 但引用不具有所指值的所有权：
 *
 *
 */
fn borrow_error() {
    // let s1 = String::from("hello");
    // let s2 = &s1;
    // let s3 = s1;
    // println!("{}", s2);

    // 这段程序不正确：因为 s2 租借的 s1 已经将所有权移动到 s3，
    // 所以 s2 将无法继续租借使用 s1 的所有权。
    // 如果需要使用 s2 使用该值，必须重新租借：

    let s1 = String::from("hello");
    let mut s2 = &s1;
    let s3 = s1;
    s2 = &s3; // 重新租借
    println!("borrowed valule s2 = {}", s2);

    // 既然引用不具有所有权，即使它租借了所有权，
    // 它也只享有使用权（这跟租房子是一个道理）。
    // 如果尝试利用租借来的权利来修改数据会被阻止：

    // println!("{}", s2);
    // s2.push_str("oob"); // 错误，禁止修改租借的值
    // println!("{}", s2);

    // 可变租借
    // 当然，也存在一种可变的租借方式，就像你租一个房子，
    // 如果物业规定房主可以修改房子结构，房主在租借时也在合同中声明赋予你这种权利，
    // 你是可以重新装修房子的：
    let mut a = String::from("run");
    // a 是可变的

    let b = &mut a;
    // b 是可变的引用
    b.push_str("oob");
    println!("{}", b);

    // 这段程序就没有问题了。我们用 &mut 修饰可变的引用类型。

    // 可变引用与不可变引用相比除了权限不同以外，
    // 可变引用不允许多重引用，但不可变引用可以：

    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // 这段程序不正确，因为多重可变引用了 s。

    // Rust 对可变引用的这种设计主要出于对并发状态下发生数据访问竞争的考虑，
    // 在编译阶段就避免了这种事情的发生。

    // 由于发生数据访问碰撞的必要条件之一是数据被至少一个使用者写且同时
    // 被至少一个其他使用者读或写，所以在一个值被可变引用时不允许再次被任何引用。
}

/*
 * dangling 引用，悬垂引用
 * 这是一个换了个名字的概念，
 * 如果放在有指针概念的编程语言里它就指的是那种没有实际指向一个
 * 真正能访问的数据的指针（注意，不一定是空指针，还有可能是已经释放的资源）。
 * 它们就像失去悬挂物体的绳子，所以叫"垂悬引用"。
 * "垂悬引用"在 Rust 语言里不允许出现，如果有，编译器会发现它。
 *
 * 下面是一个垂悬的典型案例：
 */
// fn ref_dangle() {
//     let reference_to_nothin = dangle();
// }

// 很显然，伴随着 dangle 函数的结束，其局部变量的值本身没有被当作返回值，被释放了。
// 但它的引用却被返回，这个引用所指向的值已经不能确定的存在，故不允许其出现。
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 这里是引用而不是移动
// } // 这里s的作用域结束了，会被drop掉。

fn main() {
    data();
    function();
    func_return();
    pointer();
    ref_parames();
    borrow_error();
    hybird_references();
    // ref_dangle();
}


// 引用的规则
// 1. 在任何时间，可以有一个可变引用或者多个不可变引用
// 2. 引用必须是有效的

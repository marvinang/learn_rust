/*
 * 函数式编程： 迭代器与闭包
 *
 * 闭包(Closures): 一个可以存储在变量里的类似函数的结构
 * 迭代器(Iterators): 一种处理元素序列的方式
 *
 */

//====================== 闭包 ==========================
// rust的闭包是可以保存进变量或者作为参数传递给其他函数的
// 匿名函数。可以在一个地方创建闭包，然后在不同的上下文中
// 执行闭包运算。不同于函数，闭包允许捕获调用者作用域中的
// 值。
//
// 闭包类型推断和注解
// 闭包不需要求像fn函数那样在参数和返回值上注明类型。
// 闭包通常很短并只与对应相对任意的场景较小的上下文中。
// 在这些有限制的上下文中，编译器能可靠的推断参数和返回值的类型，
// 类似于它是如何能够推断大部分变量的类型一样。

// fn add_one_v1 (x: u32) -> u32 {x + 1}
// let add_one_v2 = |x: u32| ->u32 { x+1};
// let add_one_v3 = |x| { x+1};
// let add_one_v4 = |x| x +1;

use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn app() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 使用闭包代替
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // 使用Cacher
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            // simulated_expensive_calculation(intensity)
            // expensive_closure(intensity)
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            // simulated_expensive_calculation(intensity)
            // expensive_closure(intensity)
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                // simulated_expensive_calculation(intensity)
                expensive_result.value(intensity)
            );
        }
    }
}

//--------------------- 使用带有泛型和 Fn trait的闭包
// 可以创建一个存放闭包和调用闭包结果的结构体。该结构体只会在需要结果时执行闭包，
// 并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值。
// 你可能见过这种模式被称 memoization 或 lazy evaluation。
//
// 为了让结构体存放闭包，我们需要指定闭包的类型，因为结构体定义需要知道其每一个字段的类型。
// 每一个闭包实例有其自己独有的匿名类型：也就是说，即便两个闭包有着相同的签名，他们的类型仍然可以被认为是不同。
// 为了定义使用闭包的结构体、枚举或函数参数，需要像第十章讨论的那样使用泛型和 trait bound。

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

//---------------------- 闭包会捕获其环境 ------------------
// 闭包另一个重要的特点在：
// 他们可以捕获其环境并访问其被定义的作用域的变量。
// 当闭包从环境中捕获一个值，闭包会在闭包体中储存这个值以供使用。
// 这会使用内存并产生额外的开销，在更一般的场景中，当我们不需要闭包来捕获环境时，
// 我们不希望产生这些开销。因为函数从未允许捕获环境，定义和使用函数也就从不会有这些额外开销。

fn environ() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}

// 闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：
// 获取所有权，可变借用和不可变借用。
// 这三种捕获值的方式被编码为如下三个 Fn trait：
//
// 1. FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其环境environment。
//    为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。
//    其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
// 2. FnMut 获取可变的借用值所以可以改变其环境
// 3. Fn 从其环境获取不可变的借用值
//
// 当创建一个闭包时，Rust 根据其如何使用环境中变量来推断我们希望如何引用环境。
// 由于所有闭包都可以被调用至少一次，所以所有闭包都实现了 FnOnce 。
// 那些并没有移动被捕获变量的所有权到闭包内的闭包也实现了 FnMut ，
// 而不需要对被捕获的变量进行可变访问的闭包则也实现了 Fn 。
// 在以上示例中，equal_to_x 闭包不可变的借用了 x（所以 equal_to_x 具有 Fn trait），因为闭包体只需要读取 x 的值。
//
// 如果你希望强制闭包获取其使用的环境值的所有权，可以在参数列表前使用 move 关键字。
// 这个技巧在将闭包传递给新线程以便将数据移动到新线程中时最为实用
//
fn environ_move() {
    let x = vec![1, 2, 3];
    println!("{:?}", x);
    let equal_to_x = move |z| z == x;
    // println!("{:?}", x); // 这里x的所有权已经被移走了
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

//====================== 迭代器 ========================
// 迭代器模式允许你对一个项的序列进行某些处理。
// 迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑。
// 当使用迭代器时，我们无需重新实现这些逻辑。
//
// 在 Rust 中，迭代器是惰性的（lazy），这意味着在调用方法使用迭代器之前它都不会有效果。
//
fn iterator_1() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

//------------------- Iterator trait 和 next方法 ----------
// 迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait。这个 trait 的定义看起来像这样：
// 注意这里有一下我们还未讲到的新语法：type Item 和 Self::Item，
// 他们定义了 trait 的 关联类型（associated type）
//
// 换句话说，Item 类型将是迭代器返回元素的类型。

// next 是 Iterator 实现者被要求定义的唯一方法。
// next 一次返回迭代器中的一个项，封装在 Some 中，当迭代器结束时，它返回 None。
//
// trait Iterator {
//     type item;
//     fn next(&mut self) -> Option<Self::Item>;
//     ...
//
// }

// 可以直接调用next方法
// 注意 v1_iter 需要是可变的：
// 在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态。
// 换句话说，代码 消费（consume）了，或使用了迭代器。
// 每一个 next 调用都会从迭代器中消费一个项。
//
// 使用 for 循环时无需使 v1_iter 可变因为 for 循环会获取 v1_iter 的所有权并在后台使 v1_iter 可变。
//
// 另外需要注意到从 next 调用中得到的值是 vector 的不可变引用。
// iter 方法生成一个不可变引用的迭代器。
// 如果我们需要一个获取 v1 所有权并返回拥有所有权的迭代器，
// 则可以调用 into_iter 而不是 iter。
// 类似的，如果我们希望迭代可变引用，则可以调用 iter_mut 而不是 iter。

fn iterator_next() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
}

//------------------- 消费迭代器的方法 -------------------------
// 消费适配器(consuming adaptors)
// Iterator trait 有一系列不同的由标准库提供默认实现的方法；你可以在 Iterator trait 的标准库 API 文档中找到所有这些方法。
// 一些方法在其定义中调用了 next 方法，这也就是为什么在实现 Iterator trait 时要求实现 next 方法的原因。

// 这些调用 next 方法的方法被称为 消费适配器（consuming adaptors），因为调用他们会消耗迭代器。
// 一个消费适配器的例子是 sum 方法。这个方法获取迭代器的所有权并反复调用 next 来遍历迭代器，因而会消费迭代器
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("the length of v1 = {}", total);
    // 调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权。
    // v1_iter.next();
}

//--------------------- 产生其他迭代器的方法 ------------------------------
// Iterator trait 中定义了另一类方法，被称为 迭代器适配器（iterator adaptors），
// 他们允许我们将当前迭代器变为不同类型的迭代器。可以链式调用多个迭代器适配器。
// 不过因为所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果。
//
// 因为 map 获取一个闭包，可以指定任何希望在遍历的每个元素上执行的操作。
// 这是一个展示如何使用闭包来自定义行为同时又复用 Iterator trait 提供的迭代行为的绝佳例子。
fn iterator_map() {
    let v1 = vec![1, 2, 3];
    let v1_map = v1.iter().map(|x| x + 1);

    for v in v1_map {
        println!("map = {}", v);
    }
}

//----------------------- 使用闭包获取环境 ------------------------------
// 让我们展示一个通过使用 filter 迭代器适配器和捕获环境的闭包的常规用例。
// 迭代器的 filter 方法获取一个使用迭代器的每一个项并返回布尔值的闭包。
// 如果闭包返回 true，其值将会包含在 filter 提供的新迭代器中。如果闭包返回 false，其值不会包含在结果迭代器中。

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn filter_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    for v in in_my_size {
        println!("== {:?}", v);
    }
}

//--------------------- 实现Iterator trait来创建自定义迭代器 ------------------------
// 可以实现 Iterator trait 来创建任何我们希望的迭代器。
// 正如之前提到的，定义中唯一要求提供的方法就是 next 方法。
// 一旦定义了它，就可以使用所有其他由 Iterator trait 提供的拥有默认实现的方法来创建自定义迭代器了！
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn iterator_count() {
    let mut counter = Counter::new();
    println!("counter = {:?}", counter.next());
    println!("counter = {:?}", counter.next());
    println!("counter = {:?}", counter.next());
    println!("counter = {:?}", counter.next());
    println!("counter = {:?}", counter.next());
    println!("counter = {:?}", counter.next());
    println!("counter = {:?}", counter.next());
}

//---------------------- 使用自定义迭代器中其他 Iterator trait 方法------------------------
// 通过定义 next 方法实现 Iterator trait，
// 我们现在就可以使用任何标准库定义的拥有默认实现的 Iterator trait 方法了，因为他们都使用了 next 方法的功能。
fn Iterator_count_std() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("count sume = {}", sum);
}

//================================= 性能对比： 循环 vs 迭代器 ============================
// 这里所要表达的是：迭代器，作为一个高级的抽象，被编译成了与手写的底层代码大体一致性能代码。
// 迭代器是 Rust 的 零成本抽象（zero-cost abstractions）之一，它意味着抽象并不会引入运行时开销，
// 它与本贾尼・斯特劳斯特卢普（C++ 的设计和实现者）在 「Foundations of C++」（2012） 中所定义的 零开销（zero-overhead）如出一辙：
// 从整体来说，C++ 的实现遵循了零开销原则：你不需要的，无需为他们买单。更有甚者的是：你需要的时候，也不可能找到其他更好的代码了。
//
// 闭包和迭代器是 Rust 受函数式编程语言观念所启发的功能。他们对 Rust 以底层的性能来明确的表达高级概念的能力有很大贡献。
// 闭包和迭代器的实现达到了不影响运行时性能的程度。这正是 Rust 竭力提供零成本抽象的目标的一部分。

fn main() {
    app();
    environ();
    environ_move();
    iterator_1();
    iterator_next();
    iterator_sum();
    iterator_map();
    filter_by_size();
    iterator_count();
    Iterator_count_std();
}

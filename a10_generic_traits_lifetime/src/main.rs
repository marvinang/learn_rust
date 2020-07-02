
// 泛型与特性和生命周期
// Generic Types, Traits, Lifetimes




// 泛型是为了解决代码的重复问题，提高代码可用性的一种方法


// 函数中定义泛型
// 使用泛型参数时，需要在函数名和参数列表之间声明参数类型，用<>包住。
//
fn max<T>(array: &[T]) -> T {
     let mut max_index = 0;
     let mut i = 1;
     while i < array.len() {
         if array[i] > array[max_index] {
             max_index = i;
         }
         i += 1;
     }
     array[max_index]
 }


// 结构体与枚举类中的泛型
// 在之前我们学习的 Option 和 Result 枚举类就是泛型的。
// Rust 中的结构体和枚举类都可以实现泛型机制。
struct Point<T> {
    x: T,
    y: T
}

// 使用时并没有声明类型，这里使用的是自动类型机制，但不允许出现类型不匹配的情况如下：
// let p = Point {x: 1, y: 2.0};

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}




/*
特性

特性（trait）概念接近于 Java 中的接口（Interface），但两者不完全相同。
特性与接口相同的地方在于它们都是一种行为规范，可以用于标识哪些类有哪些方法。

特性在 Rust 中用 trait 表示：

默认特性
这是特性与接口的不同点：接口只能规范方法而不能定义方法，
但特性可以定义方法作为默认方法，因为是"默认"，
所以对象既可以重新定义方法，也可以不重新定义方法使用默认的方法：

*/
trait Descriptive {
   // fn describe(&self) -> String;
   fn describe(&self) -> String {
        String::from("[Object]")
    }
}

struct Person {
    name: String,
    age: u8
}

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}


/*
特性做参数
很多情况下我们需要传递一个函数做参数，例如回调函数、设置按钮事件等。
在 Java 中函数必须以接口实现的类实例来传递，
在 Rust 中可以通过传递特性参数来实现：

*/
fn output(object: impl Descriptive) {
    println!("{}", object.describe());
}

// 任何实现了 Descriptive 特性的对象都可以作为这个函数的参数，
// 这个函数没必要了解传入对象有没有其他属性或方法，
// 只需要了解它一定有 Descriptive 特性规范的方法就可以了。
// 当然，此函数内也无法使用其他的属性与方法。

// 特性参数还可以用这种等效语法实现：
fn output<T: Descriptive>(object: T) {
    println!("{}", object.describe());
}

// 特性作类型表示时如果涉及多个特性，可以用 + 符号表示，例如：
fn notify(item: impl Summary + Display)
fn notify<T: Summary + Display>(item: T)

// 复杂的实现关系可以使用 where 关键字简化，例如：
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U)
// 简化成：
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug



fn main() {
    let a = [2,4,5,3,8];
    // println!("max = {}", max(&a));

    let p = Point { x: 1, y: 2 };

    let cali = Person {
        name: String::from("Cali"),
        age: 24
    };
    println!("{}", cali.describe());
    println!("p.x= {}", p.x());
}

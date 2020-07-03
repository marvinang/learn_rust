pub mod news {
    // 定义trait
    pub trait Summary {
        // fn summarize(&self) -> String;
        fn summarize_author(&self) -> String;
        // 默认实现
        fn summarize(&self) -> String {
            String::from("Read more...");
            // 默认实现可以调用其他方法
            format!("(Read more from{}...)", self.summarize_author())
        }
        // 使用默认
        // imple Summary for Tweet {}

        // 可以只实现非默认方法
        // impl Summary from Tweet {
        //     fn summarize_author(&self) -> String {
        //         format!("@{}", self.username)
        //     }
        // }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    // 实现
    // 注意：只有trait或者type对于crate是本地的时候，我们才能在type上实现trait。
    // Vec<T>可以实现Summary, 不能实现Vec<T>实现Display
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
}

// 这里必须要把Summary引入，不然无法运行方法
use news::{Summary, Tweet};

// ========================= 方法参数 ================
// imple Trait语法更直观，但是对于多个参数来说Trait Bound语法更加简介
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 可以使用 Trait Bound语法简化上面的方法， 其实是impl trait的语法糖。
// Traits和Trait
// Bounds语法可以让我们的代码使用泛型来减少重复，也向编译器指定所需要的泛型必须满足一定的特性（方法）。
fn notify_1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 使用 + 指定多个trait
fn notify_2(item: &(impl Summary + Display)) {}
fn notify_3<T: Summary + Display>(item: &T) {}

use std::clone::Clone;
use std::fmt::{Debug, Display};
// 使用where子句使结构更加清晰
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    10
}
fn some_function_1<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    10
}

// 返回Trait
// 返回类型的功能在闭包和迭代器中特别有用
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
// 但是只能返回同一种具体类类型
// fn returns_summarizable_1(switch: bool) -> impl Summary {
//     if switch {
//         news::NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         news::Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

fn main() {
    returns_summarizable();
    // returns_summarizable_1();
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

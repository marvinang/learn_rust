/*
Rust 中有三和重要的组织概念：箱、包、模块。

箱（Crate）
"箱"是二进制程序文件或者库文件，存在于"包"中。
"箱"是树状结构的，它的树根是编译器开始运行时编译的源文件所编译的程序。
注意："二进制程序文件"不一定是"二进制可执行文件"，只能确定是是包含目标机器语言的文件，文件格式随编译环境的不同而不同。

包（Package）
当我们使用 Cargo 执行 new 命令创建 Rust 工程时，工程目录下会建立一个 Cargo.toml 文件。工程的实质就是一个包，包必须由一个 Cargo.toml 文件来管理，该文件描述了包的基本信息以及依赖项。
一个包最多包含一个库"箱"，可以包含任意数量的二进制"箱"，但是至少包含一个"箱"（不管是库还是二进制"箱"）。
当使用 cargo new 命令创建完包之后，src 目录下会生成一个 main.rs 源文件，Cargo 默认这个文件为二进制箱的根，编译之后的二进制箱将与包名相同。

模块（Module）
对于一个软件工程来说，我们往往按照所使用的编程语言的组织规范来进行组织，组织模块的主要结构往往是树。Java 组织功能模块的主要单位是类，而 JavaScript 组织模块的主要方式是 function。
这些先进的语言的组织单位可以层层包含，就像文件系统的目录结构一样。Rust 中的组织单位是模块（Module）。

*/



/*
访问权限
Rust 中有两种简单的访问权：公共（public）和私有（private）。

默认情况下，如果不加修饰符，模块中的成员访问权将是私有的。
如果想使用公共权限，需要使用 pub 关键字。
对于私有的模块，只有在与其平级的位置或下级的位置才能访问，不能从其外部访问。

*/

mod nation {
    pub mod government {
        pub fn govern() {}
    }

    mod congress {
        pub fn legislate() {}
    }

    mod court {
        fn judicial() {
            super::congress::legislate();
        }
    }
}


// 如果模块中定义了结构体，结构体除了其本身是私有的以外，其字段也默认是私有的。
// 所以如果想使用模块中的结构体以及其字段，需要 pub 声明：

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // private field
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

// 枚举类枚举项可以内含字段，但不具备类似的性质:
mod SomeModule {
    pub enum Person {
        King {
            name: String
        },
        Quene
    }
}

fn enum_mod() {
    let person = SomeModule::Person::King{
        name: String::from("Blue")
    };
    match person {
        SomeModule::Person::King {name} => {
            println!("{}", name);
        }
        _ => {}
    }
}

fn main() {
    nation::government::govern();
    eat_at_restaurant();
    enum_mod();
}

/*
Rust 中有三和重要的组织概念：箱、包、模块。

Packages: 包，一种cargo的特性，可以进行build, test，share crates
Crates: 库或者可执行文件的模块树
Modules and use: 控制paths的组织，作用域和隐私
Paths: 命名item的一种方式，比如struct, function 和 module


箱（Crate）
"箱"是二进制程序文件或者库文件，存在于"包"中。
"箱"是树状结构的，它的树根是一个源文件，也是箱的跟模块，也是编译器开始运行时编译的源文件。
注意："二进制程序文件"不一定是"二进制可执行文件"，只能确定是是包含目标机器语言的文件，文件格式随编译环境的不同而不同。

包（Package）
一个包最多包含一个库"箱"，可以包含任意数量的二进制"箱"，但是至少包含一个"箱"（不管是库还是二进制"箱"）。
一个包必须包含一个Cargo.toml文件，它描述如何编译包中的箱。
当我们使用 Cargo 执行 new 命令创建 Rust 工程时，工程目录下会建立一个 Cargo.toml 文件。
工程的实质就是一个包，包必须由一个 Cargo.toml 文件来管理，该文件描述了包的基本信息以及依赖项。

当使用 cargo new 命令创建完包之后，src 目录下会生成一个 main.rs 源文件，Cargo 默认这个文件为二进制箱的根，编译之后的二进制箱将与包名相同。
同样，如果package根目录包含src/lib.rs, 就会默认package包含一个和包同名的库箱。
cargo在编译的时候会把crate root files传递给rustc进行编译。

如果package根目录包含src/main.rs 和 src/lib.rs, 它就包含两个crates：一个库和一个二进制箱，这两个crate和package都是同名的。
只所以称其为crate roots，因为他们两个文件都会形成一个名为crate为根的模块树。

也可以在src/bin 目录放入文件添加更多的二进制crates, 每个文件都会被分割为一个binary crate.

模块（Module）
Modules使我们可以将一个箱中的代码组织成组，提高可读性和重用性，modules也控制items的访问控制。(public vs private)
对于一个软件工程来说，我们往往按照所使用的编程语言的组织规范来进行组织，组织模块的主要结构往往是树。Java 组织功能模块的主要单位是类，而 JavaScript 组织模块的主要方式是 function。
这些先进的语言的组织单位可以层层包含，就像文件系统的目录结构一样。Rust 中的组织单位是模块（Module）。


路径（Paths)
路径用来在模块树中定位items。
path有两种形式：
1. 绝对路径：从root crate开始， crate::father::child
2. 相对路径：从当前模块开始，使用 self, super，或者当前模块的一个标识开始

-----------------------------
use 关键字把path引入当前作用域，
pub 关键字声明items是公开的。
as 关键字为项目起别名
exrernal packages
glob operatoer


*/

/*
访问权限
Rust 中有两种简单的访问权：公共（public）和私有（private）。

默认情况下，如果不加修饰符，所有items的(functions, methods, structs, enums, modules, constants)访问权将是私有的。
父模块总的成员不能访问子模块中的private成员，但是子模块的成员可以使用任何祖先模块中的成员。

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
        King { name: String },
        Quene,
    }
}

fn enum_mod() {
    let person = SomeModule::Person::King {
        name: String::from("Blue"),
    };
    match person {
        SomeModule::Person::King { name } => {
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

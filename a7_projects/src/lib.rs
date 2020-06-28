mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// 需要注意的是，整个模块树的根是一个隐性的名为crate的模块
// 组成的模块数结构如下：
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment
//

// 添加一个公开方法
pub fn eat_at_waitlist() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

// 如果模块中定义了结构体，结构体除了其本身是私有的以外，其字段也默认是私有的。
// 所以如果想使用模块中的结构体以及其字段，需要 pub 声明：
mod back_of_house {
    // 使用super来相对路径
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        // toast字段是公开的
        pub toast: String,
        // 私有字段
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
// 如果枚举类是公开的，内涵的所有项目都是公开的，因为只是枚举的一个变体
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

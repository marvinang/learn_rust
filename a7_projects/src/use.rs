//
// use 关键字能够将路径(模块标识符)引入当前作用域：
//
// 如果一直使用paths来访问items会很不方便而且书写麻烦，所以引入了use关键字，
// 它的作用是把path引入一个作用域，然后可以向使用本地items一样使用引入的模块。
//
// 因为 use 关键字把 govern 标识符导入到了当前的模块下，可以直接使用。
// 这样就解决了局部模块路径过长的问题。
mod nation {
    pub mod government {
        pub fn govern() {}
    }
    pub fn govern() {}
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 当然，有些情况下存在两个相同的名称，且同样需要导入，
// 我们可以使用 as 关键字为标识符添加别名：
use crate::nation::govern as nation_govern;
use crate::nation::government::govern;

// 相对路径
use self::front_of_house::hosting;

// 习惯的用法一般是use到方法的父模块，而不是直接把方法引入进来，
// 这样就很清楚地区别哪些不是本地方法了。
//
// 但是对于structs,enums和其他items，一般引入完整的路径。
use std::collections::HashMap;

fn main() {
    govern();
    nation_govern();
    hosting::add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1, 2);
}

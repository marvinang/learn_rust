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

// self 相对路径
use self::front_of_house::hosting;

// use 习惯用法
// 1. 习惯的用法一般是use到方法的父模块，而不是直接把方法引入进来，这样就很清楚地区别哪些不是本地方法了。
//
// 2. 但是对于structs,enums和其他items，一般引入完整的路径。
use std::collections::HashMap;

// 2.1. 例外：对于相同名称的items需要使用不同的模块
use std::fmt;
use std::io;

fn func1() -> fmt::Result {}

fn fuc2() -> io::Result {}

// re-exporting
// 使用 pub use 重新导出名称
// 当使用use关键字引入名称时，引入的名称在新作用域是private的。
// 要想时它成为public的，使用 pub use 引入
//
// use 关键字可以与 pub 关键字配合使用：
mod nation {
    pub mod government {
        pub fn govern() {}
    }
    // 这是直接吧governmen::govern暴露出去了
    pub use government::govern;
}

// use 外部packages
// Rng是一个trait
use rand::Rng;

// 使用嵌套路径简写use列表
use std::cmp::Ordering;
use std::io;
// 可以这么简写
use std::{cmp::Ordering, io};

// 还可以这么引入
use std::io;
use std::io::Write;
// 简写
use std::io::{self, Write};

// the glob operator
// 使用通配符引入所有public items
// 使用这种方法需要谨慎，因为很难分辨引入了那些名称，也不清楚程序中使用的名称是哪里定义的。
// 一般有两种情况使用
// 1. 在tests模块中将所有要测试的内容引入
// 2. prelude pattern
use std::collections::*;

fn main() {
    nation::govern();
    let secret = rand::thread_rng().gen_range(1, 101);
    govern();
    nation_govern();
    hosting::add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1, 2);
}

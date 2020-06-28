//
// use 关键字能够将路径(模块标识符)引入当前作用域：
//
// 如果一直使用paths来访问items会很不方便而且书写麻烦，所以引入了use关键字，
// 它的作用是把path引入
// 因为 use 关键字把 govern 标识符导入到了当前的模块下，可以直接使用。
// 这样就解决了局部模块路径过长的问题。
mod nation {
    pub mod government {
        pub fn govern() {}
    }
    pub fn govern() {}
}

// 当然，有些情况下存在两个相同的名称，且同样需要导入，
// 我们可以使用 as 关键字为标识符添加别名：
use crate::nation::govern as nation_govern;
use crate::nation::government::govern;

fn main() {
    govern();
    nation_govern();
}

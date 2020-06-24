//
// use 关键字能够将模块标识符引入当前作用域：

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
use crate::nation::government::govern;
use crate::nation::govern as nation_govern;


fn main() {
    govern();
    nation_govern();
}

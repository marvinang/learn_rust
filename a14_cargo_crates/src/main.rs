/*
 * Cargo 和 Crates
 *
 *
 */

// ============================ Customizing builds with release profiles ==========================
// Cargo 有两个主要的配置：运行 cargo build 时采用的 dev 配置和运行 cargo build --release 的 release 配置。
// dev 配置被定义为开发时的好的默认配置，
// release 配置则有着良好的发布构建的默认配置。

// ============================ 将crate发布到 crates.io ==========================
// Rust 也有特定的用于文档的注释类型，通常被称为 文档注释（documentation comments），他们会生成 HTML 文档。这些 HTML 展示公有 API 文档注释的内容，他们意在让对库感兴趣的程序员理解如何 使用 这个 crate，而不是它是如何被 实现 的。

// 文档注释使用三斜杠 /// 而不是两斜杆并支持 Markdown 注解来格式化文本。文档注释就位于需要文档的项的之前。
// 示例展示了一个 my_crate crate 中 add_one 函数的文档注释：

//! # My Crate
//!
//! `my_crate` 是一个使得特定计算更方便的
//! 工具集合

/// 将给定的数字加一
///
/// # Examples
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

//------------------------ 常用文档注释部分 --------------------
// 1. Examples: 使用案例
// 2. Panics：这个函数可能会 panic! 的场景。并不希望程序崩溃的函数调用者应该确保他们不会在这些情况下调用此函数。
// 3. Errros： 如果这个函数返回 Result，此部分描述可能会出现何种错误以及什么情况会造成这些错误，这有助于调用者编写代码来采用不同的方式处理不同的错误。
// 4. Safety： 如果这个函数使用 unsafe 代码（这会在第十九章讨论），这一部分应该会涉及到期望函数调用者支持的确保 unsafe 块中代码正常工作的不变条件（invariants）。

// --------------------  文档注释作为测试 -----------------------
// cargo test 也会像测试那样运行文档中的示例代码！
// 没有什么比有例子的文档更好的了！也没有什么比不能正常工作的例子更糟的了，因为代码在编写文档时已经改变。

// --------------------- items 的顶部注释 --------------------------
// 还有另一种风格的文档注释，//!，将文档添加到包含注释的项目，而不是将文档添加到注释之后的项目。这通常用于 crate 根文件（通常是 src/lib.rs）或模块的根文件为 crate 或模块整体提供文档。
// 作为一个例子，如果我们希望增加描述包含 add_one 函数的 my_crate crate 目的的文档，可以在 src/lib.rs 开头增加以 //! 开头的注释，如示例 14-2 所示：

// --------------------------- 使用 pub use 到处合适的公共api -----------------
// 即使文件结构对于用户来说 不是 很方便，你也无需重新安排内部组织：
// 你可以选择使用 pub use 重导出（re-export）项来使公有结构不同于私有结构。
// 重导出获取位于一个位置的公有项并将其公开到另一个位置，好像它就定义在这个新位置一样。

/// # Art
///
/// 一个描述美术信息的库。
pub use kinds::{PrimaryColor, SecondaryColor};
pub use utils::mix;

pub mod kinds {
    /// 采用 RGB 色彩模式的主要颜色。
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// 采用 RGB 色彩模式的次要颜色。
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use super::kinds::*;

    /// 等量的混合两个主要颜色
    /// 来创建一个次要颜色。
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        SecondaryColor::Orange
    }
}

//===================== cargo 工作空间 ===============
// 工作空间 是一系列共享同样的 Cargo.lock 和输出目录的包。
// 让我们使用工作空间创建一个项目 —— 这里采用常见的代码以便可以关注工作空间的结构。

//======================= 使用 cargo install 从 Crates.io 安装二进制文件 ==========================
// cargo install 命令用于在本地安装和使用二进制 crate。
// 它并不打算替换系统中的包；它意在作为一个方便 Rust 开发者们安装其他人已经在 crates.io 上共享的工具的手段。
// 只有拥有二进制目标文件的包能够被安装。
// 二进制目标 文件是在 crate 有 src/main.rs 或者其他指定为二进制文件时所创建的可执行程序，
// 这不同于自身不能执行但适合包含在其他程序中的库目标文件。
// 通常 crate 的 README 文件中有该 crate 是库、二进制目标还是两者都是的信息。
//
// 所有来自 cargo install 的二进制文件都安装到 Rust 安装根目录的 bin 文件夹中。
// 如果你使用 rustup.rs 安装的 Rust 且没有自定义任何配置，这将是 $HOME/.cargo/bin。
// 确保将这个目录添加到 $PATH 环境变量中就能够运行通过 cargo install 安装的程序了。

fn main() {
    println!("Hello, world!");
}

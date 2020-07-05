/*

rust使用test属性标记函数成为一个测试案例。
属性是关于一段rust代码的元数据。

*/
#[cfg(test)]
mod tests {
    #[test]
    fn another() {
        // panic!("Make this test fail");
    }
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//=========== 使用 assert! 宏======
// assert!宏有标准库提供

struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

mod tests_1 {
    use super::*;
    #[test]
    fn large_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }
}

//================= assert_eq! and assert_ne! ====================
fn add_two(a: i32) -> i32 {
    a + 2
}
#[cfg(test)]
mod tests_2 {
    use super::*;
    #[test]
    fn it_adds_tow() {
        assert_eq!(4, add_two(2));
    }
}

//==================== 添加自定义错误信息 ==========
fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

mod tests_3 {
    use super::*;
    #[test]
    fn greeting_contains_names() {
        let result = greeting("Carol");
        assert!(
            result.contains("Tom"),
            "Greeting did not contains name, value was `{}`",
            result
        );
    }
}

//========================== should_panic =========
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }
        Guess { value }
    }
}

mod tests_4 {
    use super::*;
    #[test]
    #[should_panic]
    fn less_than_1() {
        Guess::new(0);
    }

    // use should_panic 参数 expected
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(0);
    }
}

// 使用 Result<T, E>
// return Ok(()) when the test passes and an Err with a String inside when the test fails.
#[cfg(test)]
mod tests_5 {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus tow does not equal four"))
        }
    }
}

//*******************************×××××*************************
//************* 控制tests如何运行 ******************************
//***********************************××××*********************
// 有些命令行选项用于 cargo test, 而另一些用于test二进制结果。
// 使用 -- 区分这两种选项。

// ============== 并行或连续的运行testing ========
// test默认是使用线程并行执行tests, 所有测试用例之间不能相互依赖，
// 或者依赖任何共享状态，包括工作目录或者环境变量。
// 禁用并行
// cargo test -- --test-threads=1

//================ 显示函数输出 ================
// 默认test通过的话不会输出方法中的println()内容
// test失败的话会输出。
// 如果想要所有的输出都显示的话，添加 --show-output
// cargo test -- --show-output
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests_6 {
    use super::*;
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}

//======================  根据名称运行子测试 ==============
// single test
// cargo test -p a11_tests one_hundred
// fitered multitest
// cargo test -p a11_tests add
// 仅仅执行被忽略的测试
// cargo test -p a11_tests -- --ignored
#[cfg(test)]
mod tests_8 {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[ignore] // 忽略测试
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}

//*******************************×××××*************************
//************* tests 的组织结构 ******************************
//***********************************××××*********************

// rust主要有两种测试 unit tests和integration tests
//
// 1. Unit Tests
// unit tests一般方式需要测试的方法的文件中，可以用tests mod单独包括，
// 并且使用#[cfg(test)]注释模块。
// #[cfg(test)] 告诉编译器只有在cargo test的时候运行模块，而不是cargo build也去运行
// integration tests不需要这个标签。
mod private {
    pub fn add_two(a: i32) -> i32 {
        internal_adder(a, 2)
    }

    fn internal_adder(a: i32, b: i32) -> i32 {
        a + b
    }
}

#[cfg(test)]
mod tests_9 {
    use super::private::*;

    #[test]
    fn internal() {
        assert_eq!(4, add_two(2));
        // test模块也遵循隐私规则
        // assert_eq!(4, internal_adder(2, 2));
    }
}

//=============================== 集成测试 ===============
// 集成测试只能测试public方法，就像别人使用你的库一样。
// 集成测试目录在根目录的 tests文件夹下
// 集成测试可以指定单独的crate
// cargo test --test integration_test
// 集成测试不需要标注#[cfg(test)]

//============================= 集成测试中的子模块 ==========
// 每个tests目录中的文件都被编译为单独的crate文件
// 可以将共享函数提取到 tests/common.rs中，但是common.rs会显示在测试结果中，
// 可以创建 tests/common/mod.rs, 而不是tests/common.rs, 这种rust的命名规范告诉
// rust不要将common看作一个集成测试文件。
// 结论: tests目录中的子目录不会被作为单独的crate编译或作为一个测试结果部分
// 出现在测试输出中。
//


//====================== 二进制crate的集成测试 =========
// 如果只有src/main.rs 而没有 src/lib.rs, 就不可能在tests目录中创建集成测试并使用
// use导入src/main.rs中定义的函数。只有库crate才会向其他crate暴露了可供
// 调用的函数; 二进制crate只在单独运行。



fn main() {
    println!("Hello, test!");
}

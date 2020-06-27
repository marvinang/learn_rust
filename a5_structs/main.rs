/*
 * Rust 中的结构体（Struct）与元组（Tuple）都可以将若干个类型不一定相同的数据
 * 捆绑在一起形成整体，
 * 但结构体的每个成员和其本身都有一个名字， 这样访问它成员的时候就不用记住下标了。
 * 元组常用于非定义的多值传递，
 * 而结构体用于规范常用的数据结构。结构体的每个成员叫做"字段"。
 *
 */


struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32,
}

fn bar() {
    let runoob = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013
    };


    let domain = String::from("www.runoob.com");
    let name = String::from("RUNOOB");
    let run = Site {
        domain,  // 等同于 domain : domain,
        name,    // 等同于 name : name,
        nation: String::from("China"),
        found: 2013
    };


    // 有这样一种情况：你想要新建一个结构体的实例，
    // 其中大部分属性需要被设置成与现存的一个结构体属性一样，
    // 仅需更改其中的一两个字段的值，可以使用结构体更新语法：
    let site = Site {
        domain: String::from("www.example.com"),
        name: String::from("hha"),
        ..runoob
    };

}

/*
 *
 *
 * 元组结构体
 * 有一种更简单的定义和使用结构体的方式：元组结构体。
 * 元组结构体是一种形式是元组的结构体。
 * 与元组的区别是它有名字和固定的类型格式。
 * 它存在的意义是为了处理那些需要定义类型（经常使用）又不想太复杂的简单数据：
 * 
 */
fn tuple_struct() {
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);
}


/*
 *
 * 结构体所有权
 *
结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段。
这就是为什么本章的案例中使用了 String 类型而不使用 &str 的原因。
但这不意味着结构体中不定义引用型字段，这需要通过"生命周期"机制来实现。
但现在还难以说明"生命周期"概念，所以只能在后面章节说明。
 *
 */

// 输出结构体
// 如第一行所示：一定要导入调试库 #[derive(Debug)] ，
// 之后在 println 和 print 宏中就可以用 {:?} 占位符输出一整个结构体：
// 如果属性较多的话可以使用另一个占位符 {:#?} 。

#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn print_struct() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 40, height: 20 };
    println!("rect1 is {:?}", rect1);
    println!("rect1's arean is {}", rect1.area());

    println!("rect1 > rect2 = {}", rect1.wider(&rect2));
}

// 结构体方法
// 方法（Method）和函数（Function）类似，只不过它是用来操作结构体实例的。
// Rust 语言不是面向对象的，从它所有权机制的创新可以看出这一点。
// 但是面向对象的珍贵思想可以在 Rust 实现。
// 结构体方法的第一个参数必须是 &self，不需声明类型，因为 self 不是一种风格而是关键字。
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }


}

// 结构体关联函数
// 之所以"结构体方法"不叫"结构体函数"是因为"函数"这个名字留给了这种函数：
// 它在 impl 块中却没有 &self 参数。
// 贴士：结构体 impl 块可以写几次，效果相当于它们内容的拼接！
impl Rectangle {
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

// 这种函数不依赖实例，但是使用它需要声明是在哪个 impl 块中的。
// 一直使用的 String::from 函数就是一个"关联函数"。
fn create_Rectangle() {
    let rect = Rectangle::create(30,80);
    println!("{:?}", rect);
    println!("{:#?}", rect);
}

// 单元结构体
// 结构体可以值作为一种象征而无需任何成员：
// 我们称这种没有身体的结构体为单元结构体（Unit Struct）。
struct UnitStruct;

fn main() {
    bar();
    tuple_struct();
    print_struct();
    create_Rectangle();
}



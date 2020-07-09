/*
 * Rust的面向对象
 *
 */

// =============================== 面向对象语言的特点 ======================
// 面向对象编程语言所共享的一些特性往往是对象、封装和继承。
// 让我们看一下这每一个概念的含义以及 Rust 是否支持他们。

// ------------------------ 对象包含数据和行为 -------------------
// The Gang of Four 定义对象：
// 面向对象的程序是由对象组成的。一个 对象 包含数据和操作这些数据的过程。这些过程通常被称为 方法 或 操作。
//
// Rust 是面向对象的：结构体和枚举包含数据而 impl 块提供了在结构体和枚举之上的方法。
// 虽然带有方法的结构体和枚举并不被 称为 对象，但是他们提供了与对象相同的功能。

// ------------------------ 封装隐藏了实现细节 -------------------
// 另一个通常与面向对象编程相关的方面是 封装（encapsulation）的思想：对象的实现细节不能被使用对象的代码获取到。
// 所以唯一与对象交互的方式是通过对象提供的公有API；使用对象的代码无法深入到对象内部并直接改变数据或者行为。
// 封装使得改变和重构对象的内部时无需改变使用对象的代码。

// 就像我们在第七章讨论的那样：可以使用 pub 关键字来决定模块、类型、函数和方法是公有的，而默认情况下其他一切都是私有的。
// 注意，结构体自身被标记为 pub，这样其他代码就可以使用这个结构体，但是在结构体内部的字段仍然是私有的。
//
// 如果封装是一个语言被认为是面向对象语言所必要的方面的话，那么 Rust 满足这个要求。
// 在代码中不同的部分使用 pub 与否可以封装其实现细节。
//

// ------------------------- 继承，作为类型系统与代码共享 -----------
// 继承（Inheritance）是一个很多编程语言都提供的机制，
// 一个对象可以定义为继承另一个对象的定义，这使其可以获得父对象的数据和行为，而无需重新定义。

// 如果一个语言必须有继承才能被称为面向对象语言的话，那么 Rust 就不是面向对象的。
// 无法定义一个结构体继承父结构体的成员和方法。
// 然而，如果你过去常常在你的编程工具箱使用继承，根据你最初考虑继承的原因，Rust 也提供了其他的解决方案。

// 选择继承有两个主要的原因。
// 第一个是为了重用代码：一旦为一个类型实现了特定行为，继承可以对一个不同的类型重用这个实现。
// 相反 Rust 代码可以使用默认 trait 方法实现来进行共享，在示例中我们见过在 Summary trait 上增加的 summarize 方法的默认实现。
// 任何实现了 Summary trait 的类型都可以使用 summarize 方法而无须进一步实现。
// 这类似于父类有一个方法的实现，而通过继承子类也拥有这个方法的实现。
// 当实现 Summary trait 时也可以选择覆盖 summarize 的默认实现，这类似于子类覆盖从父类继承的方法实现。

// 第二个使用继承的原因与类型系统有关：表现为子类型可以用于父类型被使用的地方。
// 这也被称为 多态（polymorphism），这意味着如果多种对象共享特定的属性，则可以相互替代使用。
//
// 多态（Polymorphism）
// 很多人将多态描述为继承的同义词。不过它是一个有关可以用于多种类型的代码的更广泛的概念。
// 对于继承来说，这些类型通常是子类。
// Rust 则通过泛型来对不同的可能类型进行抽象，并通过 trait bounds 对这些类型所必须提供的内容施加约束。
// 这有时被称为 bounded parametric polymorphism。
//
// 近来继承作为一种语言设计的解决方案在很多语言中失宠了，因为其时常带有共享多于所需的代码的风险。
// 子类不应总是共享其父类的所有特征，但是继承却始终如此。
// 如此会使程序设计更为不灵活，并引入无意义的子类方法调用，或由于方法实际并不适用于子类而造成错误的可能性。
// 某些语言还只允许子类继承一个父类，进一步限制了程序设计的灵活性。

// 因为这些原因，Rust 选择了一个不同的途径，使用trait对象而不是继承。
// 让我们看一下 Rust 中的 trait 对象是如何实现多态的。

// =============================== 为使用不同类型的值而设计的trait对象 ======================
// 在第八章中，我们谈到了 vector 只能存储同种类型元素的局限。
// 示例中提供了一个定义 SpreadsheetCell 枚举来储存整型，浮点型和文本成员的替代方案。
// 这意味着可以在每个单元中储存不同类型的数据，并仍能拥有一个代表一排单元的 vector。
// 这在当编译代码时就知道希望可以交替使用的类型为固定集合的情况下是完全可行的。
//
// 然而有时我们希望库用户在特定情况下能够扩展有效的类型集合。

// ----------------- 定义通用行为的trait ----------------------------
// 我们可以使用 trait 对象代替泛型或具体类型。
// 任何使用 trait 对象的位置，Rust 的类型系统会在编译时确保任何在此上下文中使用的值会实现其 trait 对象的 trait。
// 如此便无需在编译时就知晓所有可能的类型。
//
// 之前提到过，Rust 刻意不将结构体与枚举称为 “对象”，以便与其他语言中的对象相区别。
// 在结构体或枚举中，结构体字段中的数据和 impl 块中的行为是分开的，不同于其他语言中将数据和行为组合进一个称为对象的概念中。
// trait 对象将数据和行为两者相结合，从这种意义上说则其更类似其他语言中的对象。
// 不过 trait 对象不同于传统的对象，因为不能向 trait 对象增加数据。
// trait 对象并不像其他语言中的对象那么通用：其（trait 对象）具体的作用是允许对通用行为进行抽象。

mod demo {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        // 这个 vector 的类型是Box<dyn Draw>，
        // 此为一个 trait 对象：它是Box中任何实现了 Draw trait 的类型的替身。
        //
        // 这与定义使用了带有 trait bound 的泛型类型参数的结构体不同。
        // 泛型类型参数一次只能替代一个具体类型，而 trait 对象则允许在运行时替代多种具体类型。
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub struct Button {
        pub width: i32,
        pub height: i32,
    }
}

// 使用泛型实现
mod generic {
    pub trait Draw {
        fn draw(&self);
    }
    pub struct Screen<T: Draw> {
        pub components: Vec<T>,
    }

    impl<T> Screen<T>
    where
        T: Draw,
    {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
}

// =============================== 面向对象设计模式的实现 ======================

fn main() {
    println!("Hello, world!");
}

/*
* 智能指针
* 指针（pointer）是一个包含内存地址的变量的通用概念。
* 这个地址引用，或 「指向」（points at）一些其他数据。
* Rust 中最常见的指针是第四章介绍的 引用（reference）。
* 引用以 & 符号为标志并借用了他们所指向的值。
* 除了引用数据没有任何其他特殊功能。
* 它们也没有任何额外开销，所以应用的最多。
*
* 另一方面，智能指针（smart pointers）是一类数据结构，他们的表现类似指针，
* 但是也拥有额外的元数据和功能。
* 智能指针的概念并不为 Rust 所独有；其起源于 C++ 并存在于其他语言中。
* Rust 标准库中不同的智能指针提供了多于引用的额外功能。
*
* 本章将会探索的一个例子便是 引用计数（reference counting）智能指针类型，
* 其允许数据有多个所有者。
* 引用计数智能指针记录总共有多少个所有者，并当没有任何所有者时负责清理数据。
*
* 在 Rust 中，普通引用和智能指针的一个额外的区别是引用是一类只借用数据的指针；
* 相反大部分情况，智能指针 `拥有` 他们指向的数据。
*
* 实际上本书中已经出现过一些智能指针，比如第八章的 String 和 Vec，虽然当时我们并不这么称呼它们。
* 这些类型都属于智能指针因为它们拥有一些数据并允许你修改它们。
* 它们也带有元数据（比如他们的容量）和额外的功能或保证（String 的数据总是有效的 UTF-8 编码）。
*
* 智能指针通常使用结构体实现。
* 智能指针区别于常规结构体的显著特性在于其实现了 Deref 和 Drop trait。
* Deref trait 允许智能指针结构体实例表现的像引用一样，这样就可以编写既用于引用又用于智能指针的代码。
* Drop trait 允许我们自定义当智能指针离开作用域时运行的代码。
* 本章会讨论这些 trait 以及为什么对于智能指针来说他们很重要。
*
*
* 考虑到智能指针是一个在 Rust 经常被使用的通用设计模式，本章并不会覆盖所有现存的智能指针。
* 很多库都有自己的智能指针而你也可以编写属于你自己的智能指针。这里将会讲到的是来自标准库中最常用的一些：
*  1. Box<T>，用于在堆上分配值
*  2. Rc<T>，一个引用计数类型，其数据可以有多个所有者
*  3. Ref<T> 和 RefMut<T>，通过 RefCell<T> 访问，一个在运行时而不是在编译时执行借用规则的类型。
*
* 另外我们会涉及 内部可变性（interior mutability）模式，这是不可变类型暴露出改变其内部值的 API。
* 我们也会讨论 引用循环（reference cycles）会如何泄露内存，以及如何避免。
*
*/

//========================= Box<T> 指向堆上的数据 =============================
// 最简单直接的智能指针是 box，其类型是 Box<T>
// box 允许你将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针。
//
// 除了数据被储存在堆上而不是栈上之外，box 没有性能损失。不过也没有很多额外的功能。它们多用于如下场景：
//
//  1. 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
//  2. 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
//  3. 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
//

//------------------------ Box<T> 在堆上存储数据 --------------------
// 当像 b 这样的 box 在 main 的末尾离开作用域时，它将被释放。
// 这个释放过程作用于 box 本身（位于栈上）和它所指向的数据（位于堆上）。
fn box_heap() {
    let b = Box::new(5);
    println!("b={}", b);
}

//------------------------ Box<T> 允许创建递归类型 --------------------
// Rust 需要在编译时知道类型占用多少空间。
// 一种无法在编译时知道大小的类型是 递归类型（recursive type），其值的一部分可以是相同类型的另一个值。
// 这种值的嵌套理论上可以无限的进行下去，所以 Rust 不知道递归类型需要多少空间。
// 不过 box 有一个已知的大小，所以通过在循环类型定义中插入 box，就可以创建递归类型了。

//----------------------- cons list ------------------------------------
// 在 Lisp 中，cons 函数（「construct function" 的缩写）利用两个参数来构造一个新的列表，他们通常是一个单独的值和另一个列表。
// cons 函数的概念涉及到更通用的函数式编程术语；「将 x 与 y 连接」 通常意味着构建一个新的容器而将 x 的元素放在新容器的开头，其后则是容器 y 的元素。
// cons list 的每一项都包含两个元素：当前项的值和下一项。
// 其最后一项值包含一个叫做 Nil 的值并没有下一项。
// cons list 通过递归调用 cons 函数产生。代表递归的终止条件（base case）的规范名称是 Nil，它宣布列表的终止。
// 注意这不同于第六章中的 「null」 或 「nil」 的概念，他们代表无效或缺失的值。

// Rust 无法计算为了存放 List 值到底需要多少空间。
// 让我们一点一点来看：首先了解一下 Rust 如何决定需要多少空间来存放一个非递归类型。
// enum List {
//     Cons(i32, List),
//     Nil,
// }
//
// 使用 Box 给递归类型一个已知的大小。
// 因为 Box 是一个指针，我们总是知道它需要多少空间：指针的大小并不会根据其指向的数据量而改变。
// 这意味着可以将 Box 放入 Cons 成员中而不是直接存放另一个 List 值。
// Box 会指向另一个位于堆上的 List 值，而不是存放在 Cons 成员中。
// 从概念上讲，我们仍然有一个通过在其中 「存放」 其他列表创建的列表，
// 不过现在实现这个概念的方式更像是一个项挨着另一项，而不是一项包含另一项。

// Cons 成员将会需要一个 i32 的大小加上储存 box 指针数据的空间。
// Nil 成员不储存值，所以它比 Cons 成员需要更少的空间。
// 现在我们知道了任何 List 值最多需要一个 i32 加上 box 指针数据的大小。
// 通过使用 box ，打破了这无限递归的连锁，这样编译器就能够计算出储存 List 值需要的大小了

// box 只提供了间接存储和堆分配；他们并没有任何其他特殊的功能，比如我们将会见到的其他智能指针。
// 它们也没有这些特殊功能带来的性能损失，所以他们可以用于像 cons list 这样间接存储是唯一所需功能的场景。

mod cons {
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use self::List::{Cons, Nil};
    pub fn cons_list() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("{:?}", list);
    }
}

// Box 类型是一个智能指针，因为它实现了 Deref trait，它允许 Box 值被当作引用对待。
// 当 Box 值离开作用域时，由于 Box 类型 Drop trait 的实现，box 所指向的堆数据也会被清除。

//====================== 通过 Deref trait 将智能指针当作常规引用处理 ===========
// 实现 Deref trait 允许我们重载`解引用运算符`（dereference operator）*。
// 通过这种方式实现 Deref trait 的智能指针可以被当作常规引用来对待，可以编写操作引用的代码并用于智能指针。

// --------------------------- 通过解引用运算符追踪指针的值--------------------------------
// 常规引用是一个指针类型，一种理解指针的方式是将其看成指向储存在其他某处值的箭头
fn poiter() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // 然而，如果希望对 y 的值做出断言, 必须使用 *y 来追踪引用所指向的值（也就是 解引用）。
    // 一旦解引用了 y，就可以访问 y 所指向的整型值并可以与 5 做比较。
    assert_eq!(5, *y);
}

//------------------------ 像引用一样使用 Box ------------------------
fn poiter_box() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

//------------------------- 自定义智能指针 ----------------------------
// 为了体会默认智能指针的行为不同于引用，让我们创建一个类似于标准库提供的 Box 类型的智能指针
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
// 实现Deref
// 为了实现 trait，需要提供 trait 所需的方法实现。
// Deref trait，由标准库提供，要求实现名为 deref 的方法，其借用 self 并返回一个内部数据的引用
//
// Rust 将 * 运算符替换为先调用 deref 方法再进行直接引用的操作，
// 如此我们便不用担心是不是还需要手动调用 deref 方法了。
// Rust 的这个特性可以让我们写出行为一致的代码，无论是面对的是常规引用还是实现了 Deref 的类型。

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn poiter_mybox() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // 这里其实底层运行 *(y.deref())
    assert_eq!(5, *y);
}
//=================================== 函数和方法的隐式解引用 =============================
// 解引用强制多态（deref coercions）是 Rust 表现在函数或方法传参上的一种便利。
// Deref coercions仅对实现了Deref trait上的类型起作用。
// 将这种类型转为对另一种类型的引用。
//
// 比如， &String转为&str, 因为String实现了Deref trait，以便返回str。
// 当传给给函数或方法的引用不匹配定义中的参数类型时，deref coercions会自动执行。
//
// 解引用强制多态的加入使得 Rust 程序员编写函数和方法调用时无需增加过多显式使用 & 和 * 的引用和解引用。
// 这个功能也使得我们可以编写更多同时作用于引用或智能指针的代码。

fn hello(name: &str) {
    println!("Hello {}", name);
}

fn deref_coercions() {
    let m = MyBox::new(String::from("Rust"));
    // 因为deref coercions，这里会发生自动转换。
    // Rust 可以通过 deref 调用将 &MyBox<String> 变为 &String。
    // 标准库中提供了 String 上的 Deref 实现，其会返回字符串 slice，这可以在 Deref 的 API 文档中看到。
    // Rust 再次调用 deref 将 &String 变为 &str，这就符合 hello 函数的定义了
    //
    // 当所涉及到的类型定义了 Deref trait，Rust 会分析这些类型并使用任意多次 Deref::deref 调用以获得匹配参数的类型。
    // 这些解析都发生在编译时，所以利用解引用强制多态并没有运行时惩罚！
    hello(&m);

    // 如果没有使用deref coercions
    hello(&(*m)[..]);
}

//--------------------------- Deref coercions 和 mutability 交互 --------------------------
// 类似于如何使用 Deref trait 重载不可变引用的 * 运算符，Rust 提供了 DerefMut trait 用于重载可变引用的 * 运算符。
//
// Rust 在发现类型和 trait 实现满足三种情况时会进行解引用强制多态：
// 1. 当 T: Deref<Target=U> 时从 &T 到 &U。
// 2. 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
// 3. 当 T: Deref<Target=U> 时从 &mut T 到 &U。
//
// 头两个情况除了可变性之外是相同的：
// 第一种情况表明如果有一个&T，而T实现了返回U类型的Deref，则可以直接得到 &U。
// 第二种情况表明对于可变引用也有着相同的行为。
// 第三个情况有些微妙：Rust 也会将可变引用强转为不可变引用。
// 但是反之是不可能的：不可变引用永远也不能强转为可变引用。
// 因为根据借用规则，如果有一个可变引用，其必须是这些数据的唯一引用（否则程序将无法编译）。
// 将一个可变引用转换为不可变引用永远也不会打破借用规则。
// 将不可变引用转换为可变引用则需要数据只能有一个不可变引用，而借用规则无法保证这一点。
// 因此，Rust 无法假设将不可变引用转换为可变引用是可能的。

// =================================== Drop Trait 运行清理代码 =================================
// 对于智能指针模式来说第二个重要的 trait 是 Drop，其允许我们在值要离开作用域时执行一些代码。
// 可以为任何类型提供 Drop trait 的实现，同时所指定的代码被用于释放类似于文件或网络连接的资源。
//
// 我们在智能指针上下文中讨论 Drop 是因为其功能几乎总是用于实现智能指针。
//
// 指定在值离开作用域时应该执行的代码的方式是实现 Drop trait。
// Drop trait 要求实现一个叫做 drop 的方法，它获取一个 self 的可变引用。
// 为了能够看出 Rust 何时调用 drop，让我们暂时使用 println! 语句实现 drop。
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn drop_trait() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created");
}
// 当实例离开作用域 Rust 会自动调用 drop，并调用我们指定的代码。
// 变量以被创建时相反的顺序被丢弃，所以 d 在 c 之前被丢弃。

// ------------------------------- 通过 std::mem::drop 提早丢弃值 ---------------------
// 不幸的是，我们并不能直截了当的禁用 drop 这个功能。
// 通常也不需要禁用 drop ，整个 Drop trait 存在的意义在于其是自动处理的。
// 然而，有时你可能需要提早清理某个值。
// 一个例子是当使用智能指针管理锁时；你可能希望强制运行 drop 方法来释放锁以便作用域中的其他代码可以获取锁。
// Rust 并不允许我们主动调用 Drop trait 的 drop 方法；
// 当我们希望在作用域结束之前就强制释放变量的话，我们应该使用的是由标准库提供的 std::mem::drop。
//
// Rust 不允许我们显式调用 drop 因为 Rust 仍然会在 main 的结尾对值自动调用 drop，
// 这会导致一个 double free 错误，因为 Rust 会尝试清理相同的值两次。

// std::mem::drop 函数不同于 Drop trait 中的 drop 方法。可以通过传递希望提早强制丢弃的值作为参数
use std::mem;
fn drop_early() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("early: CustomSmartPointer created");
    mem::drop(c);
    println!("early: CustomSmartPointer dropped before the end of function");
}

// ==================================== Rc<T> 引用计数器智能指针 =============================
// 大部分情况下所有权是非常明确的：可以准确的知道哪个变量拥有某个值。
// 然而，有些情况单个值可能会有多个所有者。
// 例如，在图数据结构中，多个边可能指向相同的结点，而这个结点从概念上讲为所有指向它的边所拥有。
// 结点直到没有任何边指向它之前都不应该被清理。
//
// 为了启用多所有权，Rust 有一个叫做 Rc 的类型。其名称为 引用计数（reference counting）的缩写。
// 引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没有任何有效引用并可以被清理。
//
// Rc 用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的那一部分会最后结束使用它的时候。
// 如果确实知道哪部分是最后一个结束使用的话，就可以令其成为数据的所有者同时正常的所有权规则就可以在编译时生效。
//
// 注意 Rc 只能用于单线程场景

// ----------------------------------- 使用 Rc共享数据 -------------------------------------

use std::rc::Rc;
enum List_Rc {
    Cons(i32, Rc<List_Rc>),
    Nil,
}

fn rc_list() {
    let a = Rc::new(List_Rc::Cons(
        5,
        Rc::new(List_Rc::Cons(10, Rc::new(List_Rc::Nil))),
    ));
    // Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。
    // Rc::clone 只会增加引用计数，这并不会花费多少时间。
    // 深拷贝可能会花费很长时间。
    // 通过使用 Rc::clone 进行引用计数，可以明显的区别深拷贝类的克隆和增加引用计数类的克隆。
    // 当查找代码中的性能问题时，只需考虑深拷贝类的克隆而无需考虑 Rc::clone 调用。
    let b = List_Rc::Cons(3, Rc::clone(&a));
    let c = List_Rc::Cons(4, Rc::clone(&a));
}

// ------------------------------------- Clone 会增加引用计数 ---------------------------
fn rc_clone() {
    let a = Rc::new(List_Rc::Cons(
        5,
        Rc::new(List_Rc::Cons(10, Rc::new(List_Rc::Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = List_Rc::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = List_Rc::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c get out of scope = {}", Rc::strong_count(&a));
}

// ==================================== RefCell<T> 与内部可变模式 =============================
//  内部可变性（Interior mutability）是 Rust 中的一个设计模式，它允许你即使在有不可变引用时改变数据，
//  这通常是借用规则所不允许的。
//  为了改变数据，该模式在数据结构中使用 unsafe 代码来模糊 Rust 通常的可变性和借用规则。
// 当可以确保代码在运行时会遵守借用规则，即使编译器不能保证的情况，可以选择使用那些运用内部可变性模式的类型。
// 所涉及的 unsafe 代码将被封装进安全的 API 中，而外部类型仍然是不可变的。

// ----------------------------------- 通过RefCell<T> 在运行时检查借用规则 --------------------
// 不同于 Rc<T>，RefCell<T> 代表其数据的唯一的所有权。那么是什么让 RefCell<T> 不同于像 Box<T> 这样的类型呢？回忆一下第四章所学的借用规则：
//  1. 在任意给定时间，只能拥有一个可变引用或任意数量的不可变引用 之一（而不是全部）。
//  2. 引用必须总是有效的。
//
// 对于引用和 Box<T>，借用规则的不可变性作用于编译时。
// 对于 RefCell<T>，这些不可变性作用于 运行时。
// 对于引用，如果违反这些规则，会得到一个编译错误。
// 而对于 RefCell<T>，如果违反这些规则程序会 panic 并退出。
//
// 在编译时检查借用规则的优势是这些错误将在开发过程的早期被捕获，同时对运行时没有性能影响，因为所有的分析都提前完成了。
// 为此，在编译时检查借用规则是大部分情况的最佳选择，这也正是其为何是 Rust 的默认行为。
// 相反在运行时检查借用规则的好处则是允许出现特定内存安全的场景，而它们在编译时检查中是不允许的。
// 静态分析，正如 Rust 编译器，是天生保守的。
// 但代码的一些属性不可能通过分析代码发现：其中最著名的就是 停机问题（Halting Problem），这超出了本书的范畴，
// 不过如果你感兴趣的话这是一个值得研究的有趣主题。
//
// 因为一些分析是不可能的，如果 Rust 编译器不能通过所有权规则编译，它可能会拒绝一个正确的程序；从这种角度考虑它是保守的。
// 如果 Rust 接受不正确的程序，那么用户也就不会相信 Rust 所做的保证了。然而，如果 Rust 拒绝正确的程序，虽然会给程序员带来不便，但不会带来灾难。RefCell<T> 正是用于当你确信代码遵守借用规则，而编译器不能理解和确定的时候。
//
// `类似于 Rc<T>，RefCell<T> 只能用于单线程场景`。
// 如果尝试在多线程上下文中使用RefCell<T>，会得到一个编译误。
//
// 第十六章会介绍如何在多线程程序中使用 RefCell<T> 的功能。

// 如下为选择 Box<T>，Rc<T> 或 RefCell<T> 的理由：
//  1. Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
//  2. Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
//  3. 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。
//
// 在不可变值内部改变值就是 内部可变性 模式。让我们看看何时内部可变性是有用的，并讨论这是如何成为可能的。

// ---------------------------------- 内部可变性： 不可变值的可变借用-------------------------
// 借用规则的一个推论是当有一个不可变值时，不能可变地借用它。
fn immutable() {
    let x = 5;
    // let y = &mut x; //error
}
// 然而，特定情况下在值的方法内部能够修改自身是很有用的，而不是在其他代码中。
// 此时值仍然是不可变的，值方法外部的代码不能修改其值。
// RefCell<T> 是一个获得内部可变性的方法。
// RefCell<T> 并没有完全绕开借用规则，编译器中的借用检查器允许内部可变性并相应地在运行时检查借用规则。
// 如果违反了这些规则，会得到 panic! 而不是编译错误。

// ------------------------------------ 内部可变性的用力： mock对象----------------------
// 测试替身（test double）是一个通用编程概念，它代表一个在测试中替代某个类型的类型。
// mock 对象 是特定类型的测试替身，它们记录测试过程中发生了什么以便可以断言操作是正确的。
//
// 如下是一个我们想要测试的场景：我们在编写一个记录某个值与最大值的差距的库，并根据当前值与最大值的差距来发送消息。
// 例如，这个库可以用于记录用户所允许的 API 调用数量限额。
//
// 该库只提供记录与最大值的差距，以及何种情况发送什么消息的功能。
// 使用此库的程序则期望提供实际发送消息的机制：程序可以选择记录一条消息、发送 email、发送短信等等。
// 库本身无需知道这些细节；只需实现其提供的 Messenger trait 即可。示例展示了库代码：
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// 建立mock对象进行测试
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // sent_messages: vec![],
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // 这里send中的self是不可变的，需要让mock值可以记录就需要用到RefCell
            // self.sent_messages.push(String::from(message)); }
            //
            // 对于 send 方法的实现，第一个参数仍为 self 的不可变借用，这是符合方法定义的。
            // 我们调用 self.sent_messages 中 RefCell 的 borrow_mut 方法来获取 RefCell 中值的可变引用，这是一个 vector。
            // 接着可以对 vector 的可变引用调用 push 以便记录测试过程中看到的消息。
            self.sent_messages.borrow_mut().push(String::from(message));

            // 试着违反借用规则
            // 报错：  panicked at 'already borrowed: BorrowMutError'
            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut();
            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        // 最后必须做出的修改位于断言中：为了看到其内部 vector 中有多少个项，需要调用 RefCell 的 borrow 以获取 vector 的不可变引用。
        assert_eq!(mock_messenger.sent_messages.borrow_mut().len(), 1);
    }
}

// ------------------------------------ RefCell 在运行时跟踪记录 borrows -----------------------
// 当创建不可变和可变引用时，我们分别使用 & 和 &mut 语法。
// 对于 RefCell<T> 来说，则是 borrow 和 borrow_mut 方法，这属于 RefCell<T> 安全 API 的一部分。
// borrow 方法返回 Ref 类型的智能指针，borrow_mut 方法返回 RefMut 类型的智能指针。
// 这两个类型都实现了 Deref，所以可以当作常规引用对待。
//
// RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针。
// 每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一。当 Ref 值离开作用域时，不可变借用计数减一。
// 就像编译时借用规则一样，RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用。
//
// 在运行时捕获借用错误而不是编译时意味着将会在开发过程的后期才会发现错误，甚至有可能发布到生产环境才发现；
// 还会因为在运行时而不是编译时记录借用而导致少量的运行时性能惩罚。
// 然而，使用 RefCell 使得在只允许不可变值的上下文中编写修改自身以记录消息的 mock 对象成为可能。
// 虽然有取舍，但是我们可以选择使用 RefCell<T> 来获得比常规引用所能提供的更多的功能。

// ----------------------------------- 结合Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者 ----------------------
// RefCell<T> 的一个常见用法是与 Rc<T> 结合。
// 回忆一下 Rc<T> 允许对相同数据有多个所有者，不过只能提供数据的不可变访问。
// 如果有一个储存了 RefCell<T> 的 Rc<T> 的话，就可以得到有多个所有者，并且可以修改的值了！

mod mix {
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use self::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;

    pub fn mix() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        println!("mix a before = {:?}", a);

        *value.borrow_mut() += 10;
        // 这里创建了一个 Rc<RefCell<i32>> 实例并储存在变量 value 中以便之后直接访问。
        // 接着在 a 中用包含 value 的 Cons 成员创建了一个 List。
        // 需要克隆 value 以便 a 和 value 都能拥有其内部值 5 的所有权，而不是将所有权从 value 移动到 a 或者让 a 借用 value。
        //
        // 一旦创建了列表 a、b 和 c，我们将 value 的值加 10。
        // 为此对 value 调用了 borrow_mut，
        // 这里使用了第五章讨论的自动解引用功能（“-> 运算符到哪去了？” 部分）来解引用 Rc<T> 以获取其内部的 RefCell<T> 值。
        // borrow_mut 方法返回 RefMut<T> 智能指针，可以对其使用解引用运算符并修改其内部值。

        println!("mix a after = {:?}", a);
        println!("mix b after = {:?}", b);
        println!("mix c after = {:?}", c);
    }
}

// 这是非常巧妙的！通过使用 RefCell<T>，我们可以拥有一个表面上不可变的 List，不过可以使用 RefCell<T> 中提供内部可变性的方法来在需要时修改数据。
// RefCell<T> 的运行时借用规则检查也确实保护我们免于出现数据竞争——有时为了数据结构的灵活性而付出一些性能是值得的。
//
// 标准库中也有其他提供内部可变性的类型，比如 Cell<T>，它有些类似 RefCell<T>，但是它不是提供内部值的引用，而是拷贝值和进出 Cell<T>。
//
// 还有 Mutex<T>，其提供线程间安全的内部可变性，我们将在第 16 章中讨论其用法。
// 请查看标准库来获取更多细节和不同类型之间的区别。

// ==================================== 引用循环会泄露内存 =============================
// Rust 的内存安全性保证使其难以意外地制造永远也不会被清理的内存（被称为 内存泄露（memory leak）），但并不是不可能。
// 与在编译时拒绝数据竞争不同， Rust 并不保证完全地避免内存泄露，这意味着内存泄露在 Rust 被认为是内存安全的。
// 这一点可以通过 Rc<T> 和 RefCell<T> 看出：创建引用循环的可能性是存在的。
// 这会造成内存泄露，因为每一项的引用计数永远也到不了 0，其值也永远也不会被丢弃。

// ------------------------------------ 制造循环引用 ------------------------------------
mod cycle {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    use self::List::{Cons, Nil};
    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    pub fn cycle() {
        println!("============== cycle() ================");
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // 这里会造成栈溢出
        // println!("a next item = {:?}", a.tail());
    }
}

// 创建引用循环并不容易，但也不是不可能。
// 如果你有包含 Rc<T> 的 RefCell<T> 值或类似的嵌套结合了内部可变性和引用计数的类型，请务必小心确保你没有形成一个引用循环；
// 你无法指望 Rust 帮你捕获它们。创建引用循环是一个程序上的逻辑 bug，你应该使用自动化测试、代码评审和其他软件开发最佳实践来使其最小化。
//
// 另一个解决方案是重新组织数据结构，使得一部分引用拥有所有权而另一部分没有。
// 换句话说，循环将由一些拥有所有权的关系和一些无所有权的关系组成，只有所有权关系才能影响值是否可以被丢弃。
//
// 让我们看看一个由父结点和子结点构成的图的例子，观察何时是使用无所有权的关系来避免引用循环的合适时机。

// ----------------------------------- 避免循环引用： 将Rc<T>变为Weak<T> ---------------------------
// 到目前为止，我们已经展示了调用 Rc::clone 会增加 Rc<T> 实例的 strong_count，和只在其 strong_count 为 0 时才会被清理的 Rc<T> 实例。
// 你也可以通过调用 Rc::downgrade 并传递 Rc 实例的引用来创建其值的 弱引用（weak reference）。
// 调用 Rc::downgrade 时会得到 Weak<T> 类型的智能指针。
// 不同于将 Rc<T> 实例的 strong_count 加1，调用 Rc::downgrade 会将 weak_count 加1。
// Rc<T> 类型使用 weak_count 来记录其存在多少个 Weak<T> 引用，类似于 strong_count。
// 其区别在于 weak_count 无需计数为 0 就能使 Rc 实例被清理。
//
// 强引用代表如何共享 Rc<T> 实例的所有权，但弱引用并不属于所有权关系。
// 他们不会造成引用循环，因为任何弱引用的循环会在其相关的强引用计数为 0 时被打断。

// 因为 Weak<T> 引用的值可能已经被丢弃了，为了使用 Weak<T> 所指向的值，我们必须确保其值仍然有效。
// 为此可以调用 Weak<T> 实例的 upgrade 方法，这会返回 Option<Rc<T>>。
// 如果 Rc<T> 值还未被丢弃，则结果是 Some；如果 Rc<T> 已被丢弃，则结果是 None。
// 因为 upgrade 返回一个 Option<T>，我们确信 Rust 会处理 Some 和 None 的情况，所以它不会返回非法指针。

// 我们会创建一个某项知道其子项和父项的树形结构的例子，而不是只知道其下一项的列表。

// ---------------------------- 创建树形数据结构： 带有子结点的Node -----------------

mod tree {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
    }
    // 我们希望能够 Node 拥有其子结点，同时也希望通过变量来共享所有权，以便可以直接访问树中的每一个 Node，
    // 为此 Vec<T> 的项的类型被定义为 Rc<Node>。
    // 我们还希望能修改其他结点的子结点，所以 children 中 Vec<Rc<Node>> 被放进了 RefCell<T>。

    // 这里克隆了 leaf 中的 Rc<Node> 并储存在了 branch 中，这意味着 leaf 中的 Node 现在有两个所有者：leaf和branch。
    // 可以通过 branch.children 从 branch 中获得 leaf，不过无法从 leaf 到 branch。
    // leaf 没有到 branch 的引用且并不知道他们相互关联。
    // 我们希望 leaf 知道 branch 是其父结点。稍后我们会这么做。
    pub fn tree() {
        let leaf = Rc::new(Node {
            value: 3,
            children: RefCell::new(vec![]),
        });
        let branch = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
    }
}

// ---------------- 增加从子到父的引用 ----------------------
// 为了使子结点知道其父结点，需要在 Node 结构体定义中增加一个 parent 字段。
// 问题是 parent 的类型应该是什么。
// 我们知道其不能包含 Rc<T>，因为这样 leaf.parent 将会指向 branch 而 branch.children 会包含 leaf 的指针，
// 这会形成引用循环，会造成其 strong_count 永远也不会为 0.
//
// 现在换一种方式思考这个关系，父结点应该拥有其子结点：如果父结点被丢弃了，其子结点也应该被丢弃。
// 然而子结点不应该拥有其父结点：如果丢弃子结点，其父结点应该依然存在。这正是弱引用的例子！
// 所以 parent 使用 Weak<T> 类型而不是 Rc<T>，具体来说是 RefCell<Weak<Node>>。现在 Node 结构体定义看起来像这样：
mod tree1 {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    // 这样，一个结点就能够引用其父结点，但不拥有其父结点

    pub fn tree1() {
        println!("=============== tree1() ==========");
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }

    // ------------------------- 可视化 strong_count 和 weak_count 的改变 -------------------
    // 让我们通过创建了一个新的内部作用域并将 branch 的创建放入其中，来观察 Rc<Node> 实例的 strong_count 和 weak_count 值的变化。
    // 这会展示当 branch 创建和离开作用域被丢弃时会发生什么。
    pub fn count() {
        println!("============================= count =================");
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "1. leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "2. branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "3. leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("4. leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "5. leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}

fn main() {
    box_heap();
    cons::cons_list();
    poiter();
    poiter_box();
    poiter_mybox();
    deref_coercions();
    drop_trait();
    drop_early();
    rc_list();
    rc_clone();
    mix::mix();
    cycle::cycle();
    tree1::tree1();
    tree1::count();
}

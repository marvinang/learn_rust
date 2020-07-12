/*
 * 高级特性

1. 不安全 Rust：用于当需要舍弃 Rust 的某些保证并负责手动维持这些保证
2. 高级 trait：与 trait 相关的关联类型，默认类型参数，完全限定语法（fully qualified syntax），
   超（父）trait（supertraits）和 newtype 模式
3. 高级类型：关于 newtype 模式的更多内容，类型别名，never 类型和动态大小类型
4. 高级函数和闭包：函数指针和返回闭包
5. 宏：定义在编译时定义更多更多代码的方式
 *
 */

// ================ unsafe rust =================
// 目前为止讨论过的代码都有 Rust 在编译时会强制执行的内存安全保证。
// 然而，Rust 还隐藏有第二种语言，它不会强制执行这类内存安全保证：
// 这被称为 不安全 Rust（unsafe Rust）。
// 它与常规 Rust 代码无异，但是会提供额外的超级力量。

// 不安全 Rust 之所以存在，是因为静态分析本质上是保守的。
// 当编译器尝试确定一段代码是否支持某个保证时，拒绝一些有效的程序比接受无效程序要好一些。
// 这必然意味着有时代码可能是合法的，但是 Rust 不这么认为！
// 在这种情况下，可以使用不安全代码告诉编译器，“相信我，我知道我在干什么。”
// 这么做的缺点就是你只能靠自己了：如果不安全代码出错了，比如解引用空指针，可能会导致不安全的内存使用。

// 另一个 Rust 存在不安全一面的原因是：底层计算机硬件固有的不安全性。
// 如果 Rust 不允许进行不安全操作，那么有些任务则根本完成不了。
// Rust 需要能够进行像直接与操作系统交互，甚至于编写你自己的操作系统这样的底层系统编程！
// 这也是 Rust 语言的目标之一。
// 让我们看看不安全 Rust 能做什么，和怎么做。

// ------- 不安全的超级力量 -------------
// 可以通过 unsafe 关键字来切换到不安全 Rust，接着可以开启一个新的存放不安全代码的块。
// 这里有四类可以在不安全 Rust 中进行而不能用于安全 Rust 的操作，它们称之为 “不安全的超级力量。”
// 这些超级力量是：
//   1. 解引用裸指针
//   2. 调用不安全的函数或方法
//   3. 访问或修改可变静态变量
//   4. 实现不安全 trait
//   5. 访问 union 的字段
//   6. 有一点很重要，unsafe 并不会关闭借用检查器或禁用任何其他 Rust 安全检查：
//      如果在不安全代码中使用引用，它仍会被检查。
//      unsafe 关键字只是提供了那四个不会被编译器检查内存安全的功能。
//      你仍然能在不安全块中获得某种程度的安全。

// 再者，unsafe 不意味着块中的代码就一定是危险的或者必然导致内存安全问题：
// 其意图在于作为程序员你将会确保 unsafe 块中的代码以有效的方式访问内存。
//
// 人是会犯错误的，错误总会发生，不过通过要求这四类操作必须位于标记为 unsafe 的块中，
// 就能够知道任何与内存安全相关的错误必定位于 unsafe 块内。
// 保持 unsafe 块尽可能小，如此当之后调查内存 bug 时就会感谢你自己了。

// 为了尽可能隔离不安全代码，将不安全代码封装进一个安全的抽象并提供安全 API 是一个好主意，
// 当我们学习不安全函数和方法时会讨论到。
// 标准库的一部分被实现为在被评审过的不安全代码之上的安全抽象。
// 这个技术防止了 unsafe 泄露到所有你或者用户希望使用由 unsafe 代码实现的功能的地方，
// 因为使用其安全抽象是安全的。

// 让我们按顺序依次介绍上述四个超级力量，同时我们会看到一些提供不安全代码的安全接口的抽象。

// ------------------ 解引用裸指针 ---------
// 回到第四章的 “悬垂引用” 部分，那里提到了编译器会确保引用总是有效的。
// 不安全 Rust 有两个被称为 裸指针（raw pointers）的类似于引用的新类型。
// 和引用一样，裸指针是可变或不可变的，分别写作 *const T 和 *mut T。
// 这里的星号不是解引用运算符；它是类型名称的一部分。
// 在裸指针的上下文中，不可变 意味着指针解引用之后不能直接赋值。

// 与引用和智能指针的区别在于，记住裸指针：
//   1. 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
//   2. 不保证指向有效的内存
//   3. 允许为空
//   4. 不能实现任何自动清理功能
//
// 通过去掉 Rust 强加的保证，你可以放弃安全保证以换取性能或使用另一个语言或硬件接口的能力，此时 Rust 的保证并不适用。
//
// 注意这里没有引入 unsafe 关键字。
// 可以在安全代码中 创建 裸指针，只是不能在不安全块之外解引用裸指针，
// 稍后便会看到。
//
// 这里使用 as 将不可变和可变引用强转为对应的裸指针类型。
// 因为直接从保证安全的引用来创建他们，可以知道这些特定的裸指针是有效，
// 但是不能对任何裸指针做出如此假设。
#![allow(unused_variables)]
fn raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 接下来会创建一个不能确定其有效性的裸指针，
    // 示例展示了如何创建一个指向任意内存地址的裸指针。
    // 尝试使用任意内存是未定义行为：
    //  此地址可能有数据也可能没有，编译器可能会优化掉这个内存访问，
    // 或者程序可能会出现段错误（segmentation fault）。
    // 通常没有好的理由编写这样的代码，不过却是可行的：

    let address = 0x012345usize;
    let r = address as *const i32;

    // 记得我们说过可以在安全代码中创建裸指针，
    // 不过不能 解引用 裸指针和读取其指向的数据。
    // 现在我们要做的就是对裸指针使用解引用运算符 *，
    // 这需要一个 unsafe 块，
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // println!("r is: {}", *r);
    }
    // 还需注意示例中创建了同时指向相同内存位置 num 的裸指针 *const i32 和 *mut i32。
    // 相反如果尝试创建 num 的不可变和可变引用，
    // 这将无法编译因为 Rust 的所有权规则不允许拥有可变引用的同时拥有不可变引用。
    // 通过裸指针，就能够同时创建同一地址的可变指针和不可变指针，
    // 若通过可变指针修改数据，则可能潜在造成数据竞争。请多加小心！
}

// 创建一个指针不会造成任何危险；
// 只有当访问其指向的值时才有可能遇到无效的值。

// 既然存在这么多的危险，为何还要使用裸指针呢？
// 一个主要的应用场景便是调用 C 代码接口，这在下一部分 “调用不安全函数或方法” 中会讲到。
// 另一个场景是构建借用检查器无法理解的安全抽象。
// 让我们先介绍不安全函数，接着看一看使用不安全代码的安全抽象的例子。

// ------------------- 调用不安全函数或方法 ------------------
// 第二类要求使用不安全块的操作是调用不安全函数。
// 不安全函数和方法与常规函数方法十分类似，
// 除了其开头有一个额外的 unsafe。
// unsafe 表明我们作为程序需要满足其要求，因为 Rust 不会保证满足这些要求。
// 通过在 unsafe 块中调用不安全函数，我们表明已经阅读过此函数的文档并对其是否满足函数自身的契约负责。
unsafe fn dangerous() {
    println!("This is very dangerous!");
}
// 不安全函数体也是有效的 unsafe 块，
// 所以在不安全函数中进行另一个不安全操作时无需新增额外的 unsafe 块。

fn call_unsafe_function() {
    unsafe {
        dangerous();
    }
}

// ------------------- 创建不安全的安全抽象 ----------------
// 仅仅因为函数包含不安全代码并不意味着整个函数都需要标记为不安全的。
// 事实上，将不安全代码封装进安全函数是一个常见的抽象。
// 作为一个例子，标准库中的函数，split_at_mut，
// 它需要一些不安全代码，让我们探索如何可以实现它。
// 这个安全函数定义于可变 slice 之上：
// 它获取一个 slice 并从给定的索引参数开始将其分为两个 slice。
// split_at_mut 的用法如示例 19-4 所示：
fn split_slice() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let (a, b) = split_at_mut_demo(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// 这个函数无法只通过安全 Rust 实现。
// 一个尝试可能看起来像示例，它不能编译。
// 出于简单考虑，我们将 split_at_mut 实现为函数而不是方法，
// 并只处理 i32 值而非泛型 T 的 slice。
use std::slice;
fn split_at_mut_demo(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);

    // (&mut slice[..mid], &mut slice[mid..])

    // 不安全代码
    let ptr = slice.as_mut_ptr();
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}
// 以上代码， Rust 的借用检查器不能理解我们要借用这个 slice 的两个不同部分：
// 它只知道我们借用了同一个 slice 两次。
// 本质上借用 slice 的不同部分是可以的，因为结果两个 slice 不会重叠，
// 不过 Rust 还没有智能到能够理解这些。
// 当我们知道某些事是可以的而 Rust 不知道的时候，就是触及不安全代码的时候了

// 回忆第四章的 “Slice 类型” 部分，slice 是一个指向一些数据的指针，并带有该 slice 的长度。
// 可以使用 len 方法获取 slice 的长度，使用 as_mut_ptr 方法访问 slice 的裸指针。
// 在这个例子中，因为有一个 i32 值的可变 slice，as_mut_ptr 返回一个 *mut i32 类型的裸指针，储存在 ptr 变量中。

// 我们保持索引 mid 位于 slice 中的断言。
// 接着是不安全代码：slice::from_raw_parts_mut 函数获取一个裸指针和一个长度来创建一个 slice。
// 这里使用此函数从 ptr 中创建了一个有 mid 个项的 slice。
// 之后在 ptr 上调用 offset 方法并使用 mid 作为参数来获取一个从 mid 开始的裸指针，
// 使用这个裸指针并以 mid 之后项的数量为长度创建一个 slice。

// slice::from_raw_parts_mut 函数是不安全的因为它获取一个裸指针，并必须确信这个指针是有效的。
// 裸指针上的 offset 方法也是不安全的，因为其必须确信此地址偏移量也是有效的指针。
// 因此必须将 slice::from_raw_parts_mut 和 offset 放入 unsafe 块中以便能调用它们。
// 通过观察代码，和增加 mid 必然小于等于 len 的断言，我们可以说 unsafe 块中所有的裸指针将是有效的 slice 中数据的指针。
// 这是一个可以接受的 unsafe 的恰当用法。

// 注意无需将 split_at_mut 函数的结果标记为 unsafe，并可以在安全 Rust 中调用此函数。
// 我们创建了一个不安全代码的安全抽象，其代码以一种安全的方式使用了 unsafe 代码，
// 因为其只从这个函数访问的数据中创建了有效的指针。

// slice::from_raw_parts_mut 在使用 slice 时很有可能会崩溃。
// 这段代码获取任意内存地址并创建了一个长为一万的 slice：
fn broken_slice() {
    let address = 0x012345usize;
    let r = address as *mut i32;

    let s: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    // println!("broken end... {:?}", s);
}

// ===================== 使用extern函数调用外部代码 ==================
// 有时你的 Rust 代码可能需要与其他语言编写的代码交互。
// 为此 Rust 有一个关键字，extern，有助于创建和使用 外部函数接口（Foreign Function Interface， FFI）。
// 外部函数接口是一个编程语言用以定义函数的方式，其允许不同（外部）编程语言调用这些函数。

// 示例展示了如何集成 C 标准库中的 abs 函数。
// extern 块中声明的函数在 Rust 代码中总是不安全的。
// 因为其他语言不会强制执行 Rust 的规则且 Rust 无法检查它们，
// 所以确保其安全是程序员的责任：
extern "C" {
    fn abs(input: i32) -> i32;
}
fn call_c_abs() {
    unsafe {
        println!("c abs(-3) = {}", abs(-3));
    }
}

// 在 extern "C" 块中，列出了我们希望能够调用的另一个语言中的外部函数的签名和名称。
// "C" 部分定义了外部函数所使用的应用程序接口（application binary interface，ABI）
// —— ABI 定义了如何在汇编语言层面调用此函数。
// "C" ABI 是最常见的，并遵循 C 编程语言的 ABI。

// ------------------- 从其他语言调用 rust 函数 ------------
// 也可以使用 extern 来创建一个允许其他语言调用 Rust 函数的接口。
// 不同于 extern 块，就在 fn 关键字之前增加 extern 关键字并指定所用到的 ABI。
// 还需增加 #[no_mangle] 注解来告诉 Rust 编译器不要 mangle 此函数的名称。
// Mangling 发生于当编译器将我们指定的函数名修改为不同的名称时，这会增加用于其他编译过程的额外信息，
// 不过会使其名称更难以阅读。
// 每一个编程语言的编译器都会以稍微不同的方式 mangle 函数名，所以为了使 Rust 函数能在其他语言中指定，
// 必须禁用 Rust 编译器的 name mangling。
// 在如下的例子中，一旦其编译为动态库并从 C 语言中链接，call_from_c 函数就能够在 C 代码中访问：
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a rust function from C!");
}
// extern 的使用无需unsafe。

// ------------------- 访问或修改可变静态变量 --------- 
// 目前为止全书都尽量避免讨论 全局变量（global variables），Rust 确实支持他们，
// 不过这对于 Rust 的所有权规则来说是有问题的。
// 如果有两个线程访问相同的可变全局变量，则可能会造成数据竞争。

// 全局变量在 Rust 中被称为静态（static）变量。
// 示例展示了一个拥有字符串 slice 值的静态变量的声明和应用：
static HELLO_WORLD: &str = "Hello, world!";
fn static_word() {
    println!("name is {}", HELLO_WORLD);
}
// static 变量类似于第三章 “变量和常量的区别” 部分讨论的常量。
// 通常静态变量的名称采用 SCREAMING_SNAKE_CASE 写法，并 必须 标注变量的类型，
// 在这个例子中是 &'static str。静态变量只能储存拥有 'static 生命周期的引用，
// 这意味着 Rust 编译器可以自己计算出其生命周期而无需显式标注。
// 访问不可变静态变量是安全的。

// 常量与不可变静态变量可能看起来很类似，不过一个微妙的区别是静态变量中的值有一个固定的内存地址。
// 使用这个值总是会访问相同的地址。
// 另一方面，常量则允许在任何被用到的时候复制其数据。

// 常量与静态变量的另一个区别在于静态变量可以是可变的。
// 访问和修改可变静态变量都是 不安全 的。
// 示例展示了如何声明、访问和修改名为 COUNTER 的可变静态变量：
static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
// 就像常规变量一样，我们使用 mut 关键来指定可变性。
// 任何读写 COUNTER 的代码都必须位于 unsafe 块中。
// 这段代码可以编译并如期打印出 COUNTER: 3，因为这是单线程的。
// 拥有多个线程访问 COUNTER 则可能导致数据竞争。

// 拥有可以全局访问的可变数据，难以保证不存在数据竞争，
// 这就是为何 Rust 认为可变静态变量是不安全的。
// 任何可能的情况，请优先使用第十六章讨论的并发技术和线程安全智能指针，
// 这样编译器就能检测不同线程间的数据访问是否是安全的。


// -------------------- 实现不安全的trait  ------------------
// 最后一个只能用在 unsafe 中的操作是实现不安全 trait。
// 当至少有一个方法中包含编译器不能验证的不变量时 trait 是不安全的。
// 可以在 trait 之前增加 unsafe 关键字将 trait 声明为 unsafe，同时 trait 的实现也必须标记为 unsafe，
// 如示例所示：
unsafe trait Foo {

}

unsafe impl Foo for i32 {
}
// 通过 unsafe impl，我们承诺将保证编译器所不能验证的不变量。

// 作为一个例子，回忆第十六章 “使用 Sync 和 Send trait 的可扩展并发” 部分中的 Sync 和 Send 标记 trait，
// 编译器会自动为完全由 Send 和 Sync 类型组成的类型自动实现他们。
// 如果实现了一个包含一些不是 Send 或 Sync 的类型，比如裸指针，
// 并希望将此类型标记为 Send 或 Sync，则必须使用 unsafe。
// Rust 不能验证我们的类型保证可以安全的跨线程发送或在多线程键访问，
// 所以需要我们自己进行检查并通过 unsafe 表明。


// -------------------------------- 何时使用 unsafe ----------------
// 使用 unsafe 来进行这四个操作（超级力量）之一是没有问题的，甚至是不需要深思熟虑的，
// 不过使得 unsafe 代码正确也实属不易，因为编译器不能帮助保证内存安全。
// 当有理由使用 unsafe 代码时，是可以这么做的，通过使用显式的 unsafe 标注使得在出现错误时易于追踪问题的源头。

fn main() {
    raw_pointer();
    call_unsafe_function();
    split_slice();
    broken_slice();
    call_c_abs();
    static_word();
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

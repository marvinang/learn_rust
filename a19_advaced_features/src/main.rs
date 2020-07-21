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

// ------------ 使用extern函数调用外部代码 ------
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
unsafe trait Foo {}

unsafe impl Foo for i32 {}
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

// ===================== 高级生命周期 =========================
// 有三个未涉及到的生命周期高级特性：
// 1. 生命周期子类型(lifetime subtyping), 一个确保某个生命周期长于另一个生命周期的方式。
// 2. 生命周期bound(lifetime bounds), 用于指定泛型引用的生命周期。
// 3. trait对象生命周期（trait object lifetimes), 以及他们是如何推断的，以及何时需要指定匿名生命周期：使（生命周期）深略更为明显。

// --------------------- 生命周期子类型 --------------
// 生命周期子类型是一个指定某个生命周期应该长于另一个生命周期的方式。
// 为了探索生命周期子类型，想象一下我们想要编写一个解析器。
// 为此会有一个储存了需要解析的字符串的引用的结构体 Context。
// 解析器将会解析字符串并返回成功或失败。
// 解析器需要借用 Context 来进行解析。
// 其实现看起来像示例中的代码，除了缺少了必须的生命周期注解，
// 所以这还不能编译：
struct Context<'a>(&'a str);

// 并通过语法 's: 'c 声明一个不短于 'c 的生命周期 's
struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

// impl<'c, 's: 'c> Parser<'c, 's> {
//     fn parse(&self) -> Result<(), &'s str> {
//         Err(&self.context.0[1..])
//     }
// }

// Parser 和 context 需要比整个函数长寿（outlive）并在函数开始之前和结束之后都有效以确保代码中的所有引用始终是有效的。
// 虽然我们创建的两个 Parser 和 context 参数在函数的结尾就离开了作用域，因为 parse_context 获取了 context 的所有权。
//
// 需要一个方法来告诉 Rust Context 中的字符串 slice 与 Parser 中 Context 的引用有着不同的生命周期，
// 而且 parse_context 返回值与 Context 中字符串 slice 的生命周期相联系。

// fn parse_context(context: Context) -> Result<(), &str> {
//     Parser { context: &context }.parse()
// }

// ------------------ 生命周期bound ----------
// 在第十章 「trait bound」 部分，我们讨论了如何在泛型类型上使用 trait bound。
// 也可以像泛型那样为生命周期参数增加限制，这被称为 生命周期 bound（lifetime bounds）。
// 生命周期 bound 帮助 Rust 验证泛型的引用不会存在的比其引用的数据更久。

// T 增加生命周期 bound 来指定 T 中的任何引用需至少与 'a 存活的一样久

// 现在代码可以编译了，因为 T: 'a 语法指定了 T 可以为任意类型，不过如果它包含任何引用的话，
// 其生命周期必须至少与 'a 一样长。
struct Ref<'a, T: 'a>(&'a T);

// 在 T 上增加 'static 生命周期 bound，来限制T 为只拥有 'static 生命周期的引用或没有引用的类型
// 因为 'static 意味着引用必须同整个程序存活的一样长，一个不包含引用的类型满足所有引用都与整个程序存活的一样长的标准（因为他们没有引用）。对于借用检查器来说它关心的是引用是否存活的足够久，没有引用的类型与有永远存在的引用的类型并没有真正的区别；对于确定引用是否比其所引用的值存活得较短的目的来说两者是一样的。
struct StaticRef<T: 'static>(&'static T);

// ------------ trait 对象生命周期的推断 -------------
//
trait Red {}

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}

// 这段代码能没有任何错误的编译，即便并没有明确指出 obj 中涉及的任何生命周期。
// 这是因为有如下生命周期与 trait 对象必须遵守的规则：
//
//   1. trait 对象的默认生命周期是 'static。
//   2. 如果有 &'a X 或 &'a mut X，则默认生命周期是 'a。
//   3. 如果只有 T: 'a 从句， 则默认生命周期是 'a。
//   4. 如果有多个类似 T: 'a 的从句，则没有默认生命周期；必须明确指定。

// 当必须明确指定时，可以为像 Box<Red> 这样的 trait 对象增加生命周期 bound，
// 根据需要使用语法 Box<Foo + 'a> 或 Box<Foo + 'static>。
// 正如其他的 bound，这意味着任何 Red trait 的实现如果在内部包含有引用，
// 这些引用就必须拥有与 trait 对象 bound 中所指定的相同的生命周期。
fn new_Ball() {
    let num = 5;
    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
}

// --------------------- 匿名生命周期 -------------------------
// 比方说有一个封装了一个字符串 slice 的结构体，如下：
struct StrWrap<'a>(&'a str);

// 可以像这样编写一个返回它们的函数：
fn foo<'a>(string: &'a str) -> StrWrap<'a> {
    StrWrap(string)
}
// 不过这里有很多的 'a！为了消除这些噪音，可以使用匿名生命周期，'_，如下：
fn foo_new(string: &str) -> StrWrap<'_> {
    StrWrap(string)
}
// '_ 表明 「在此处使用省略的生命周期。」
// 这意味着我们仍然知道 StrWrap 包含一个引用，不过无需所有的生命周期注解来知道。
// 其也能用于 impl；例如：
trait Debug {}
// impl<'a> Debug for StrWrap<'a> {
// }

// 省略生命周期
impl Debug for StrWrap<'_> {}

// ================ 高级trait ============================

// ---------- 关联类型在trait定义中指定站位符类型 -----
// 关联类型（associated types）是一个将类型占位符与 trait 相关联的方式，
// 这样 trait 的方法签名中就可以使用这些占位符类型。
// trait 的实现者会针对特定的实现在这个类型的位置指定相应的具体类型。
// 如此可以定义一个使用多种类型的 trait，直到实现此 trait 时都无需知道这些类型具体是什么。
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(10)
    }
}
pub trait Iterator_1<T> {
    fn next(&mut self) -> Option<T>;
}

// 以上两种方式的区别是：
// 区别在于当如示例那样使用泛型时，则不得不在每一个实现中标注类型。
// 这是因为我们也可以实现为 Iterator<String> for Counter，或任何其他类型，
// 这样就可以有多个 Counter 的 Iterator 的实现。
// 换句话说，当 trait 有泛型参数时，可以多次实现这个trait，每次需改变泛型参数的具体类型。
// 接着当使用 Counter 的 next 方法时，必须提供类型注解来表明希望使用 Iterator 的哪一个实现。

// 通过关联类型，则无需标注类型因为不能多次实现这个 trait。
// 对于示例使用关联类型的定义，我们只能选择一次 Item 会是什么类型，因为只能有一个 impl Iterator for Counter。
// 当调用 Counter 的 next 时不必每次指定我们需要 u32 值的迭代器。

// ---------------- 默认泛型类型参数和运算符重载 --------------
// 当使用泛型类型参数时，可以为泛型指定一个默认的具体类型。
// 如果默认类型就足够的话，这消除了为具体类型实现 trait 的需要。
// 为泛型类型指定默认类型的语法是在声明泛型类型时使用 <PlaceholderType=ConcreteType>。

// 这种情况的一个非常好的例子是用于运算符重载。
// 运算符重载（Operator overloading）是指在特定情况下自定义运算符（比如 +）行为的操作。

// Rust 并不允许创建自定义运算符或重载任意运算符，
// 不过std::ops中所列出的运算符和相应的 trait 可以通过实现运算符相关 trait 来重载

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn add_point() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 0, y: 1 };
    println!("{:?}", p1 + p2);
}

// 查看Add trait
// trait Add<RHS=Self> {
//     type Output;
//     fn add(self, rhs: RHS) -> Self::Output;
// }
// 这看来应该很熟悉，这是一个带有一个方法和一个关联类型的 trait。
// 比较陌生的部分是尖括号中的 RHS=Self：这个语法叫做 默认类型参数（default type parameters）。
// RHS 是一个泛型类型参数（“right hand side” 的缩写），它用于定义 add 方法中的 rhs 参数。
// 如果实现 Add trait 时不指定 RHS 的具体类型，RHS 的类型将是默认的 Self 类型，也就是在其上实现 Add 的类型。

struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// 为了使 Millimeters 和 Meters 能够相加，我们指定 impl Add<Meters> 来设定 RHS 类型参数的值而不是使用默认的 Self。
//
// 默认参数类型主要用于如下两个方面：
//    1. 扩展类型而不破坏现有代码。
//    2. 在大部分用户都不需要的特定情况进行自定义。

// 标准库的 Add trait 就是一个第二个目的例子：大部分时候你会将两个相似的类型相加，不过它提供了自定义额外行为的能力。
// 在 Add trait 定义中使用默认类型参数意味着大部分时候无需指定额外的参数。
// 换句话说，一小部分实现的样板代码是不必要的，这样使用 trait 就更容易了。
//
// 第一个目的是相似的，但过程是反过来的：如果需要为现有 trait 增加类型参数，
// 为其提供一个默认类型将允许我们在不破坏现有实现代码的基础上扩展 trait 的功能。

// -------------- 完全限定语法与消歧义： 调用相同名称的方法 ----
// Rust 既不能避免一个 trait 与另一个 trait 拥有相同名称的方法，也不能阻止为同一类型同时实现这两个 trait。
// 甚至直接在类型上实现开始已经有的同名方法也是可能的！
// 不过，当调用这些同名方法时，需要告诉 Rust 我们希望使用哪一个。
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("pilot");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("wizard");
    }
}
impl Human {
    fn fly(&self) {
        println!("human");
    }
}

fn print_human() {
    let person = Human;
    person.fly();
    // 因为 fly 方法获取一个 self 参数，如果有两个 类型 都实现了同一 trait，
    // Rust 可以根据 self 的类型计算出应该使用哪一个 trait 实现。
    Pilot::fly(&person);
    Wizard::fly(&person);
}

// 然而，关联函数是 trait 的一部分，但没有 self 参数。
// 当同一作用域的两个类型实现了同一 trait，Rust 就不能计算出我们期望的是哪一个类型，
// 除非使用 完全限定语法（fully qualified syntax）。
trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("dog")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("animal")
    }
}

fn print_dog() {
    println!("=={}", Dog::baby_name());
    // 因为 Animal::baby_name 是关联函数而不是方法，因此它没有 self 参数，
    // Rust 无法计算出所需的是哪一个 Animal::baby_name 实现
    // println!("=={}", Animal::baby_name());

    // 为了消歧义并告诉 Rust 我们希望使用的是 Dog 的 Animal 实现，
    // 需要使用 完全限定语法，这是调用函数时最为明确的方式
    println!("=={}", <Dog as Animal>::baby_name());
}

// 通常，完全限定语法定义为：
// <Type as Trait>::function(receiver_if_method, next_arg, ...);
//
// 对于关联函数，其没有一个 receiver，故只会有其他参数的列表。
// 可以选择在任何函数或方法调用处使用完全限定语法。
// 然而，允许省略任何 Rust 能够从程序中的其他信息中计算出的部分。
// 只有当存在多个同名实现而 Rust 需要帮助以便知道我们希望调用哪个实现时，才需要使用这个较为冗长的语法。

// ------------- 父trait用于在另一个trait中使用某trait的功能 ----------
// 有时我们可能会需要某个 trait 使用另一个 trait 的功能。
// 在这种情况下，需要能够依赖相关的 trait 也被实现。
// 这个所需的 trait 是我们实现的 trait 的 父（超） trait（supertrait）。
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct People {
    name: String,
    age: u32,
}

impl fmt::Display for People {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.age)
    }
}
impl OutlinePrint for People {}

// -------------- newtype 模式用以在外部类型上实现外部trait ------------
// 在第十章的 “为类型实现 trait” “为类型实现 trait” 部分，我们提到了孤儿规则（orphan rule），
// 它说明只要 trait 或类型对于当前 crate 是本地的话就可以在此类型上实现该 trait。
// 一个绕开这个限制的方法是使用 newtype 模式（newtype pattern），
// 它涉及到在一个元组结构体（第五章 “用没有命名字段的元组结构体来创建不同的类型” 部分介绍了元组结构体）中创建一个新类型。
// 这个元组结构体带有一个字段作为希望实现 trait 的类型的简单封装。
// 接着这个封装类型对于 crate 是本地的，这样就可以在这个封装上实现 trait。
// Newtype 是一个源自（U.C.0079，逃）Haskell 编程语言的概念。
// 使用这个模式没有运行时性能惩罚，这个封装类型在编译时就被省略了。

// 如果想要在 Vec<T> 上实现 Display，而孤儿规则阻止我们直接这么做，
// 因为 Display trait 和 Vec<T> 都定义于我们的 crate 之外。
// 可以创建一个包含 Vec<T> 实例的 Wrapper 结构体.
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// 此方法的缺点是，因为 Wrapper 是一个新类型，它没有定义于其值之上的方法；
// 必须直接在 Wrapper 上实现 Vec<T> 的所有方法，
// 这样就可以代理到self.0 上 —— 这就允许我们完全像 Vec<T> 那样对待 Wrapper。
// 如果希望新类型拥有其内部类型的每一个方法，为封装类型实现 Deref trait（第十五章 “通过 Deref trait 将智能指针当作常规引用处理” 部分讨论过）并返回其内部类型是一种解决方案。
// 如果不希望封装类型拥有所有内部类型的方法 —— 比如为了限制封装类型的行为 —— 则必须只自行实现所需的方法。

fn set_wrapper() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

// ================ 高级类型 ===========

// ---------- 为了类型安全和抽象而使用 newtype 模式 --------
// newtype 模式可以用于一些其他我们还未讨论的功能，包括静态的确保某值不被混淆，和用来表示一个值的单元。
// 实际上示例 19-23 中已经有一个这样的例子：Millimeters 和 Meters 结构体都在 newtype 中封装了 u32 值。
// 如果编写了一个有 Millimeters 类型参数的函数，不小心使用 Meters 或普通的 u32 值来调用该函数的程序是不能编译的。

// 另一个 newtype 模式的应用在于抽象掉一些类型的实现细节：
// 例如，封装类型可以暴露出与直接使用其内部私有类型时所不同的公有 API，以便限制其功能。

// newtype 也可以隐藏其内部的泛型类型。
// 例如，可以提供一个封装了HashMap<i32, String> 的People 类型，用来储存人名以及相应的 ID。
// 使用 People 的代码只需与提供的公有 API 交互即可，比如向 People 集合增加名字字符串的方法，
// 这样这些代码就无需知道在内部我们将一个 i32 ID 赋予了这个名字了。
// newtype 模式是一种实现第十七章 “封装隐藏了实现细节” 部分所讨论的隐藏实现细节的封装的轻量级方法

// ----------- 类型别名用来创建类型同义词 ------------------
// 连同 newtype 模式，Rust 还提供了声明 类型别名（type alias）的能力，
// 使用 type 关键字来给予现有类型另一个名字。

type Kilometers = i32;
// 这意味着 Kilometers 是 i32 的 同义词（synonym）；
// 不同于示例 19-23 中创建的 Millimeters 和 Meters 类型。
// Kilometers 不是一个新的、单独的类型。
// Kilometers 类型的值将被完全当作 i32 类型值来对待：
fn print_kilometers() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);
}

// 类型别名的主要用途是减少重复。例如，可能会有这样很长的类型：
// Box<dyn Fn() + Send + 'static>
type Thunk = Box<dyn Fn() + Send + 'static>;
// let f: Thunk = Box::new(|| println!("hi"));

// 类型别名也经常与 Result<T, E> 结合使用来减少重复。
type Result<T> = std::result::Result<T, std::io::Error>;
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

// --------- 从不返回的never type ------------
// Rust 有一个叫做 ! 的特殊类型。
// 在类型理论术语中，它被称为 empty type，因为它没有值。
// 我们更倾向于称之为 never type。
// 这个名字描述了它的作用：在函数从不返回的时候充当返回值。例如：
fn bar() -> ! {
    panic!("hello")
}
// 这读 “函数 bar 从不返回”，而从不返回的函数被称为 发散函数（diverging functions）。
// 不能创建 ! 类型的值，所以 bar 也不可能返回值。
fn no_return(guess: String) {
    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}
// 正如你可能猜到的，continue 的值是 !。
// 也就是说，当 Rust 要计算 guess 的类型时，它查看这两个分支。
// 前者是 u32 值，而后者是 ! 值。
// 因为 ! 并没有一个值，Rust 决定 guess 的类型是 u32。
//
// 描述 ! 的行为的正式方式是 never type 可以强转为任何其他类型。
// 允许 match 的分支以 continue 结束是因为 continue 并不真正返回一个值；
// 相反它把控制权交回上层循环，所以在 Err 的情况，事实上并未对 guess 赋值。

// = never type 的另一个用途是 panic!。
// impl<T> Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Some(val) => val,
//             None => panic!("called `Option::unwrap()` on a `None` value"),
//         }
//     }
// }
// Rust 知道 val 是 T 类型，panic! 是 ! 类型，所以整个 match 表达式的结果是 T 类型。
// 这能工作是因为 panic! 并不产生一个值；它会终止程序。
// 对于 None 的情况，unwrap 并不返回一个值，所以这些代码是有效。

// 最后一个有着 ! 类型的表达式是 loop：
fn loop_value() {
    print!("forever ");
    loop {
        print!("and ever ");
    }
}
// 这里，循环永远也不结束，所以此表达式的值是 !。
// 但是如果引入 break 这就不为真了，因为循环在执行到 break 后就会终止。

// ----------- 动态大小类型和Sized trait --------------
// 因为 Rust 需要知道例如应该为特定类型的值分配多少空间这样的信息其类型系统的一个特定的角落可能令人迷惑：
// 这就是 动态大小类型（dynamically sized types）的概念。
// 这有时被称为 “DST” 或 “unsized types”，这些类型允许我们处理只有在运行时才知道大小的类型。

// 让我们深入研究一个贯穿本书都在使用的动态大小类型的细节：str。
// 没错，不是 &str，而是 str 本身。
// str 是一个 DST；直到运行时我们都不知道字符串有多长。
// 因为直到运行时都不能知道大其小，也就意味着不能创建 str 类型的变量，也不能获取 str 类型的参数。
// 考虑一下这些代码，他们不能工作：
fn dst() {
    // let s1: str = "Hello there";
    // let s2: str = "How's it going";
}

// Rust 需要知道应该为特定类型的值分配多少内存，同时所有同一类型的值必须使用相同数量的内存。
// 如果允许编写这样的代码，也就意味着这两个 str 需要占用完全相同大小的空间，
// 不过它们有着不同的长度。这也就是为什么不可能创建一个存放动态大小类型的变量的原因。

// 所以虽然 &T 是一个储存了 T 所在的内存位置的单个值，&str 则是 两个 值：str 的地址和其长度。
// 这样，&str 就有了一个在编译时可以知道的大小：它是 usize 长度的两倍。
// 也就是说，我们总是知道 &str 的大小，而无论其引用的字符串是多长。
// 这里是 Rust 中动态大小类型的常规用法：他们有一些额外的元信息来储存动态信息的大小。
//
// 这引出了动态大小类型的黄金规则：必须将动态大小类型的值置于某种指针之后。

// 可以将 str 与所有类型的指针结合：比如 Box<str> 或 Rc<str>。
// 事实上，之前我们已经见过了，不过是另一个动态大小类型：trait。
// 每一个 trait 都是一个可以通过 trait 名称来引用的动态大小类型。
// 在第十七章 “为使用不同类型的值而设计的 trait 对象” 部分，我们提到了为了将 trait 用于 trait 对象，
// 必须将他们放入指针之后，比如 &Trait 或 Box<Trait>（Rc<Trait> 也可以）。

// 为了处理 DST，Rust 有一个特定的 trait 来决定一个类型的大小是否在编译时可知：这就是 Sized trait。
// 这个 trait 自动为编译器在编译时就知道大小的类型实现。
// 另外，Rust 隐式的为每一个泛型函数增加了 Sized bound。
// 也就是说，对于如下泛型函数定义：
// fn generic<T>(t: T) {
//    println!("generic");
// }

fn generic<T: Sized>(t: T) {
    println!("generic");
}

// 泛型函数默认只能用于在编译时已知大小的类型。
// 然而可以使用如下特殊语法来放宽这个限制：
fn generic_1<T: ?Sized>(t: &T) {
    println!("generic");
}

// ?Sized trait bound 与 Sized 相对；
// 也就是说，它可以读作 “T 可能是也可能不是 Sized 的”。
// 这个语法只能用于 Sized ，而不能用于其他 trait。

// 另外注意我们将 t 参数的类型从 T 变为了 &T：因为其类型可能不是 Sized 的，所以需要将其置于某种指针之后。
// 在这个例子中选择了引用。

//
// ================ 高级函数和闭包 =================================

// ----------- 函数指针 ------------
// 我们讨论过了如何向函数传递闭包；
// 也可以向函数传递常规函数！
// 这在我们希望传递已经定义的函数而不是重新定义闭包作为参数是很有用。
// 通过函数指针允许我们使用函数作为另一个函数的参数。
// 函数的类型是 fn （使用小写的 “f” ）以免与 Fn 闭包 trait 相混淆。
// fn 被称为 函数指针（function pointer）。
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// 不同于闭包，fn 是一个类型而不是一个 trait，所以直接指定 fn 作为参数而不是声明一个带有 Fn 作为 trait bound 的泛型参数。

// 函数指针实现了所有三个闭包 trait（Fn、FnMut 和 FnOnce），
// 所以总是可以在调用期望闭包的函数时传递函数指针作为参数。
// 倾向于编写使用泛型和闭包 trait 的函数，这样它就能接受函数或闭包作为参数。

// 一个只期望接受 fn 而不接受闭包的情况的例子是与不存在闭包的外部代码交互时：
// C 语言的函数可以接受函数作为参数，但 C 语言没有闭包。

// 作为一个既可以使用内联定义的闭包又可以使用命名函数的例子，让我们看看一个 map 的应用。
// 使用 map 函数将一个数字 vector 转换为一个字符串 vector，就可以使用闭包，比如这样：
fn map_conv() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("=== list_of_strings ====== {:?}", list_of_strings);
}

// 或者可以将函数作为map的参数来代替闭包：
fn map_closure() {
    let list_of_numbers = vec![1, 2, 3];
    // 注意这里必须使用 “高级 trait” 部分讲到的完全限定语法，因为存在多个叫做 to_string 的函数；
    // 这里使用了定义于 ToString trait 的 to_string 函数，标准库为所有实现了 Display 的类型实现了这个 trait。
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("=== list_of_strings ====== {:?}", list_of_strings);
}

// 另一个实用的模式暴露了元组结构体和元组结构体枚举成员的实现细节。
// 这些项使用 () 作为初始化语法，这看起来就像函数调用，同时它们确实被实现为返回由参数构造的实例的函数。
// 它们也被称为实现了闭包 trait 的函数指针，并可以采用类似如下的方式调用：
#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn tuple_conv() {
    // 这里创建了 Status::Value 实例，它通过 map 用范围的每一个 u32 值调用 Status::Value 的初始化函数。
    // 一些人倾向于函数风格，一些人喜欢闭包。这两种形式最终都会产生同样的代码，所以请使用对你来说更明白的形式吧。
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("=== list_of_statuses ====== {:?}", list_of_statuses);
}

// ------------ 返回闭包 ----------
// 闭包表现为 trait，这意味着不能直接返回闭包。
// 对于大部分需要返回 trait 的情况，可以使用实现了期望返回的 trait 的具体类型来替代函数的返回值。
// 但是这不能用于闭包，因为他们没有一个可返回的具体类型；
// 例如不允许使用函数指针 fn 作为返回值类型。

// 这段代码尝试直接返回闭包，它并不能编译：
// fn return_closure() -> Fn(i32) -> i32 {
//     |x| x+1
// }

// 错误又一次指向了 Sized trait！Rust 并不知道需要多少空间来储存闭包。
// 不过我们在上一部分见过这种情况的解决办法：可以使用 trait 对象。
fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x|x+1)
}


// ========================= 宏 =====================



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
    //------
    let a = Context("nihao a");
    // println!("{:?}", parse_context(a));
    new_Ball();
    add_point();
    //-----
    print_human();
    print_dog();
    let p = People {
        name: String::from("tom"),
        age: 32,
    };
    p.outline_print();
    set_wrapper();
    print_kilometers();
    // ---------
    let answer = do_twice(add_one, 5);
    println!("the answer is: {}", answer);
    map_conv();
    map_closure();
    tuple_conv();
}

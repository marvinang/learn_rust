pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // 这里调用 Option 的 as_ref 方法是因为需要 Option 中值的引用而不是获取其所有权。
        // 因为 state 是一个 Option<Box<State>>，
        // 调用 as_ref 会返回一个 Option<&Box<State>>。
        // 如果不调用 as_ref，将会得到一个错误，
        // 因为不能将 state 移动出借用的 &self 函数参数。
        //
        // 接着调用 unwrap 方法，这里我们知道它永远也不会 panic，
        // 因为 Post 的所有方法都确保在他们返回时 state 会有一个 Some 值。这就是一个第十二章 “当我们比编译器知道更多的情况” 部分讨论过的我们知道 None 是不可能的而编译器却不能理解的情况。
        //
        // 接着我们就有了一个 &Box<State>，当调用其 content 时，
        // 解引用强制多态会作用于 & 和 Box ，
        // 这样最终会调用实现了 State trait 的类型的 content 方法。
        // 这意味着需要为 State trait 定义增加 content，
        // 这也是放置根据所处状态返回什么内容的逻辑的地方
        self.state.as_ref().unwrap().content(self)
    }

    // 请求审核博文
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    // 审核通过
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    // 这个语法意味着这个方法调用只对这个类型的 Box 有效。
    // 这个语法获取了 Box<Self> 的所有权，使老状态无效化以便 Post 的状态值可以将自身转换为新状态。
    //
    // 现在开始能够看出状态模式的优势了：Post 的 request_review 方法无论 state 是何值都是一样的。
    // 每个状态只负责它自己的规则。
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

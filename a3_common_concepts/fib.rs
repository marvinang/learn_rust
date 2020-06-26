fn main() {
    println!("1th fibonacci is {}", fib(1));
    println!("2th fibonacci is {}", fib(2));
    println!("3th fibonacci is {}", fib(3));
    println!("8th fibonacci is {}", fib(8));
    // println!("155th fibonacci is {}", fib(155));
}

fn fib(nth: i64) -> i64 {
    if nth <= 1 {
        return nth;
    } else {
        return fib(nth - 2) + fib(nth - 1);
    }
}

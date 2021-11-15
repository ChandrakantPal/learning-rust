// use std::collections::HashMap;  available only in main function
mod greet {
    // use std::collections::HashMap available only in greet module
    fn hello() {
        println!("hello");
    }

    fn goodbye() {
        println!("goodbye");
    }
}

mod math {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    use greet::hello;
    hello();
    greet::goodbye();
}

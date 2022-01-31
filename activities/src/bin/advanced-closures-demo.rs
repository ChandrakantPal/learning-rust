fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}
fn main() {
    let add = Box::new(|a, b| {
        println!("adding!");
        a + b
    });
    let sub = Box::new(|a, b| a - b);
    let mul = Box::new(|a, b| a * b);
    println!("{}", math(25, 45, add));
}

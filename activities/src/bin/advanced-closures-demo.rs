fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}
fn main() {
    let add = |a, b| a + b;
    let add: Box<_> = Box::new(add);
    println!("{}", math(25, 45, add))
}

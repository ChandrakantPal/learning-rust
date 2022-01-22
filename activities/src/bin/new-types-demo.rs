#[derive(Debug, Copy, Clone)]
struct NeverZero(i32);

impl NeverZero {
    fn new(i: i32) -> Result<Self, String> {}
}

fn divide(a: i32, b: NeverZero) -> i32 {}

fn main() {}

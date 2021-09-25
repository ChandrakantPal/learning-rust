enum Direction {
    Left,
    Right,
}
fn main() {
    let go = Direction::Left;
    match go {
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
    }
}

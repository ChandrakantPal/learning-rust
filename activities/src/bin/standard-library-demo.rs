fn main() {
    let numbers = vec![1, 2, 3];
    match numbers.is_empty() {
        true => println!("no numbers"),
        false => println!("has numbers"),
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // let mut plus_one = vec![];
    // for num in numbers {
    //     plus_one.push(num + 1);
    // }

    let plus_one: Vec<_> = numbers.iter().map(|num| num + 1).collect();

    let new_numbers: Vec<_> = numbers.iter().filter(|num| num <= 3).collect();

    let numbers = vec![1, 2, 3, 4, 5];
    let find_me: Option<i32> = numbers.iter().find(|num| num == 3);
}

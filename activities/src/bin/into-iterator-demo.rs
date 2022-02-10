use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
enum Fruit {
    Apple,
    Banana,
    Orange,
}

struct FruitStand {
    fruit: HashMap<Fruit, u32>,
}

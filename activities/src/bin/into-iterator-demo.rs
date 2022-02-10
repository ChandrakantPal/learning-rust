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

impl IntoIterator for FruitStand {
    type Item = (Fruit, u32);
    type IntoIter = std::collections::hash_map::IntoIter<Fruit, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.fruit.into_iter()
    }
}

impl<'a> IntoIterator for &'a FruitStand {
    type Item = (&'a Fruit, &'a u32);
    type IntoIter = std::collections::hash_map::Iter<'a, Fruit, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.fruit.iter()
    }
}

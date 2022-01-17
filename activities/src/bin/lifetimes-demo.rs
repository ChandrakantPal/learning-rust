#[derive(Debug)]
struct Cards {
    inner: Vec<IdCard>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum City {
    Barland,
    Bazopolis,
    Fooville,
}

#[derive(Debug)]
struct IdCard {
    name: String,
    age: u8,
    city: City,
}

impl IdCard {
    pub fn new(name: &str, age: u8, city: City) -> Self {
        Self {
            name: name.to_string(),
            age,
            city,
        }
    }
}

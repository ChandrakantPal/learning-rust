enum Species {
    Finch,
    Hawk,
    Parrot,
}

struct Bird {
    age: usize,
    species: Species,
}

fn main() {
    let hawk = Bird {
        age: 13,
        species: Species::Hawk,
    };

    match hawk {
        Bird { age: 4, .. } => println!("4 year old bird"),
        Bird {
            age: 4..=10 | 15..=20,
            ..
        } => println!("4-10 or 15-20 year old bird"),
        Bird {
            species: Species::Finch,
        } => println!("finch!"),
        Bird { .. } => println!("other bird"),
    }
}

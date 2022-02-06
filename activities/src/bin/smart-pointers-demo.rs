use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum MenuItem {
    Drink,
    Salad,
}

#[derive(Debug)]
struct ItemOrder {
    item: MenuItem,
    quantity: u32,
}

#[derive(Debug)]
struct TableOrder {
    items: Vec<ItemOrder>,
}

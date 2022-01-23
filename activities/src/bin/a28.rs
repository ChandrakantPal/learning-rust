// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

struct ShirtColor(Color);
impl ShirtColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}
struct ShoesColor(Color);
impl ShoesColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}
struct PantsColor(Color);
impl PantsColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

fn print_shirt_color(color: ShirtColor) {
    println!("shirt color = {:?}", color);
}

fn main() {}

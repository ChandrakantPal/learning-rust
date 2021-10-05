struct Temprature {
    degrees_f: f64,
}

impl Temprature {
    fn show_temp(&self) {
        println!("{:} degrees F", self.degrees_f);
    }
}

fn main() {
    let hot = Temprature { degrees_f: 99.9 };
    hot.show_temp();
}

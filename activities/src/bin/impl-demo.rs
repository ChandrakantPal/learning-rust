struct Temprature {
    degrees_f: f64,
}

impl Temprature {
    fn freezing() -> Self {
        Self { degrees_f: 32.0 }
    }
    fn boiling() -> Self {
        Self { degrees_f: 212.0 }
    }
    fn show_temp(&self) {
        println!("{:} degrees F", self.degrees_f);
    }
}

fn main() {
    let hot = Temprature { degrees_f: 99.9 };
    hot.show_temp();

    let cold = Temprature::freezing();
    cold.show_temp();

    let boiling = Temprature::boiling();
    boiling.show_temp();
}

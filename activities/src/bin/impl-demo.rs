struct Temprature {
    degrees_f: f64,
}

impl Temprature {
    fn show_temp(temp: Temprature) {
        println!("{:} degrees F", temp.degrees_f);
    }
}

fn main() {
    let hot = Temprature { degrees_f: 99.9 };
    Temprature::show_temp(hot);
}

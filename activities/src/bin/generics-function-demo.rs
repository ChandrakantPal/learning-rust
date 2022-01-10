trait CheckIn {
    fn check_in(&self);
    fn process(&self);
}

struct Pilot;
impl CheckIn for Pilot {
    fn check_in(&self) {
        println!("checked in as pilot");
    }
    fn process(&self) {
        println!("pilot enters the cockpit");
    }
}

fn main() {}

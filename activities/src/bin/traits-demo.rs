trait Fall {
    fn hit_ground(&self);
}

struct Vase;
impl Fall for Vase {
    fn hit_ground(&self) {
        println!("the vase broke!")
    }
}

fn main() {}

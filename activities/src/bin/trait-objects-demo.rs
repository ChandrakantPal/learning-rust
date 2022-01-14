trait Sale {
    fn amount(&self) -> f64;
}

struct FullSale(f64);
impl Sale for FullSale {
    fn amount(&self) -> f64 {
        self.0
    }
}

struct OneDollarOffCoupon(f64);
impl Sale for OneDollarOffCoupon {
    fn amount(&self) -> f64 {
        self.0 - 1.0
    }
}

fn main() {}

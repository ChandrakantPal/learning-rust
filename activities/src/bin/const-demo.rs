fn clamp_speed(speed: i32) -> i32 {
    if speed > 9000 {
        9000
    } else {
        speed
    }
}

fn main() {}

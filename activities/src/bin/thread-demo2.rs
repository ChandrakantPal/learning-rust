use std::thread;
use std::time::Duration;

fn main() {
    let value: JoinHandle<usize> = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        42
    });
}

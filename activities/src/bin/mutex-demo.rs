use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

type SharedSignData = Arc<Mutex<String>>;

struct DigitalSignBoard {
    display: SharedSignData,
}

impl DigitalSignBoard {
    fn update(&self) {
        let data = self.display().lock();
        println!("sign data='{}'", data);
        // place on digital sign
    }
}

fn spawn_display_thread(display_data: SharedSignData) {
    thread::spawn(|| {
        let board = DigitalSignBoard {
            display: display_data,
        };

        loop {
            board.update();
            thread::sleep(Duration::from_millis(200));
        }
    });
}

fn change_data(display_data: SharedSignData, new_data: &str) {
    let mut data = display_data.lock();
    *data = new_data.to_owned();
}

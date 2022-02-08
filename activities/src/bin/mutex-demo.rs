use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

type SharedSignData = Arc<Mutex<String>>;

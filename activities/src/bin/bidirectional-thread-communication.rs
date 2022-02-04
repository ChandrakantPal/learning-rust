use crossbeam_channel::unbounded;
use std::thread;

enum ThreadMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

enum MainMsg {
    SumResult(i64),
    WorkerQuit,
}

fn main() {
    let (worker_tx, worker_rx) = unbounded();
    let (main_tx, main_rx) = unbounded();

    let handle = thread::spawn(move || loop {
        match r.recv() {
            Ok(msg) => match msg {
                ThreadMsg::PrintData(d) => println!("{}", d),
                ThreadMsg::Sum(lhs, rhs) => println!("{}+{}={}", lhs, rhs, (lhs + rhs)),
                ThreadMsg::Quit => {
                    println!("thread terminating");
                    break;
                }
            },
            Err(e) => {
                println!("disconnected");
                break;
            }
        }
    });

    s.send(ThreadMsg::PrintData("hello from main".to_owned()));
    s.send(ThreadMsg::Sum(10, 10));
    s.send(ThreadMsg::Quit);
    // drop(s);
    handle.join();
}

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    for worker_id in 0..4 {
        let tx = tx.clone();
        thread::spawn(move || {
            for job in 0..3 {
                let msg = format!("worker {worker_id} processed job {job}");
                tx.send(msg).expect("send should succeed");
                thread::sleep(Duration::from_millis(50));
            }
        });
    }

    drop(tx);

    for received in rx {
        println!("{received}");
    }
}

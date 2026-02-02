use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = Vec::new();

    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10_000 {
                let mut guard = counter.lock().expect("lock should succeed");
                *guard += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("thread should finish");
    }

    println!("final counter = {}", *counter.lock().expect("lock should succeed"));
}

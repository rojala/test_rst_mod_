use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let chopsticks: Vec<_> = (0..5).map(|_| Arc::new(Mutex::new(()))).collect();
    let mut handles = Vec::new();

    for id in 0..5 {
        let left = Arc::clone(&chopsticks[id]);
        let right = Arc::clone(&chopsticks[(id + 1) % 5]);

        let handle = thread::spawn(move || {
            for _ in 0..3 {
                // Deadlock avoidance: lock in a consistent order by address.
                let (first, second) = if Arc::as_ptr(&left) < Arc::as_ptr(&right) {
                    (&left, &right)
                } else {
                    (&right, &left)
                };

                let _first_guard = first.lock().expect("lock should succeed");
                let _second_guard = second.lock().expect("lock should succeed");

                println!("philosopher {id} is eating");
                thread::sleep(Duration::from_millis(50));
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("thread should finish");
    }
}

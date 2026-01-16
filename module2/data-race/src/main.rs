// Synchronization Lab: Choose between Mutex, RwLock, and Condvar implementations
// Run with --help to see available options

use clap::Parser;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;

#[derive(Parser, Debug)]
#[command(name = "Data Race Lab")]
#[command(about = "Demonstrates thread-safe data access using Mutex, RwLock, or Condvar", long_about = None)]
struct Args {
    /// Use RwLock instead of Mutex
    #[arg(long)]
    rwlock: bool,

    /// Use Condvar for producer-consumer synchronization
    #[arg(long)]
    condvar: bool,
}

fn main() {
    let args = Args::parse();

    if args.condvar {
        run_with_condvar();
    } else if args.rwlock {
        run_with_rwlock();
    } else {
        run_with_mutex();
    }
}

fn run_with_mutex() {
    println!("Running with Mutex...");
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let handles: Vec<_> = (0..3)
        .map(|i| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let mut data = data.lock().unwrap();
                println!("Thread {} acquired Mutex lock", i);
                data[i] += 1;
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final data with Mutex: {:?}", data);
}

fn run_with_rwlock() {
    println!("Running with RwLock...");
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    let handles: Vec<_> = (0..3)
        .map(|i| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let mut data = data.write().unwrap();
                println!("Thread {} acquired RwLock write lock", i);
                data[i] += 1;
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final data with RwLock: {:?}", data);
}

fn run_with_condvar() {
    println!("Running with Condvar (Producer-Consumer Pattern)...\n");

    let queue = Arc::new(Mutex::new(Vec::new()));
    let condvar = Arc::new(Condvar::new());

    // Producer thread - adds items to queue
    let queue_clone = Arc::clone(&queue);
    let condvar_clone = Arc::clone(&condvar);
    let producer = thread::spawn(move || {
        for item in 0..5 {
            {
                let mut q = queue_clone.lock().unwrap();
                q.push(item);
                println!("[Producer] Added item: {}, queue size: {}", item, q.len());
                // Notify one waiting consumer
                condvar_clone.notify_one();
            }
            thread::sleep(std::time::Duration::from_millis(100));
        }
        println!("[Producer] Finished producing");
    });

    // Consumer thread 1 - consumes items from queue
    let queue_clone = Arc::clone(&queue);
    let condvar_clone = Arc::clone(&condvar);
    let _consumer1 = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(50));
        loop {
            let mut q = queue_clone.lock().unwrap();
            while q.is_empty() {
                println!("[Consumer 1] Waiting for items...");
                q = condvar_clone.wait(q).unwrap();
            }
            if let Some(item) = q.pop() {
                println!(
                    "[Consumer 1] Consumed item: {}, remaining: {}",
                    item,
                    q.len()
                );
            }
        }
    });

    // Consumer thread 2 - consumes items from queue
    let queue_clone = Arc::clone(&queue);
    let condvar_clone = Arc::clone(&condvar);
    let _consumer2 = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(150));
        loop {
            let mut q = queue_clone.lock().unwrap();
            while q.is_empty() {
                println!("[Consumer 2] Waiting for items...");
                q = condvar_clone.wait(q).unwrap();
            }
            if let Some(item) = q.pop() {
                println!(
                    "[Consumer 2] Consumed item: {}, remaining: {}",
                    item,
                    q.len()
                );
            }
        }
    });

    // Wait for producer to finish
    producer.join().unwrap();

    println!("\n[Main] Producer finished, allowing consumers 1 second to drain queue...");
    thread::sleep(std::time::Duration::from_secs(1));

    // Final state
    let final_queue = queue.lock().unwrap();
    println!("\nFinal queue state: {:?}", *final_queue);
    println!("Condvar demonstration complete!");

    // Note: consumer threads will keep running indefinitely waiting for more items
    // In a real application, you'd signal them to stop
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Condvar;
    use std::time::Duration;

    #[test]
    fn test_rwlock_multiple_readers() {
        let data = Arc::new(RwLock::new(vec![1, 2, 3]));
        let mut handles = vec![];

        // Spawn multiple reader threads
        for _i in 0..5 {
            let data_clone = Arc::clone(&data);
            let handle = thread::spawn(move || {
                let data_ref = data_clone.read().unwrap();
                assert_eq!(data_ref[0], 1);
                assert_eq!(data_ref[1], 2);
                assert_eq!(data_ref[2], 3);
            });
            handles.push(handle);
        }

        // Wait for all readers to complete
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_rwlock_exclusive_write() {
        let data = Arc::new(RwLock::new(vec![1, 2, 3]));

        // Acquire write lock
        let mut data_ref = data.write().unwrap();
        data_ref[0] = 100;

        // Trying to read in another thread should block until write lock is released
        let data_clone = Arc::clone(&data);
        let read_result = std::thread::spawn(move || {
            // This will block until the write lock above is released
            let data_ref = data_clone.read().unwrap();
            data_ref[0]
        });

        // Give the other thread time to try acquiring the read lock
        thread::sleep(Duration::from_millis(100));

        // Release the write lock
        drop(data_ref);

        // Now the read should succeed
        let value = read_result.join().unwrap();
        assert_eq!(value, 100);
    }

    #[test]
    fn test_rwlock_writer_modifications() {
        let data = Arc::new(RwLock::new(vec![1, 2, 3]));
        let mut handles = vec![];

        // Spawn writer threads
        for i in 0..3 {
            let data_clone = Arc::clone(&data);
            let handle = thread::spawn(move || {
                let mut data_ref = data_clone.write().unwrap();
                data_ref[i] += 10;
            });
            handles.push(handle);
        }

        // Wait for all writers to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all modifications were applied
        let final_data = data.read().unwrap();
        assert_eq!(final_data[0], 11);
        assert_eq!(final_data[1], 12);
        assert_eq!(final_data[2], 13);
    }

    #[test]
    fn test_rwlock_vs_mutex_behavior() {
        // Both RwLock and Mutex provide thread-safe access
        // The key difference is that RwLock allows multiple readers

        let rwlock_data = Arc::new(RwLock::new(vec![1, 2, 3]));

        // Multiple threads can acquire read locks simultaneously
        let mut read_handles = vec![];
        for _ in 0..3 {
            let data_clone = Arc::clone(&rwlock_data);
            let handle = thread::spawn(move || {
                let data_ref = data_clone.read().unwrap();
                assert_eq!(data_ref.len(), 3);
            });
            read_handles.push(handle);
        }

        for handle in read_handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_rwlock_poison_on_panic() {
        use std::panic;

        let data = Arc::new(RwLock::new(vec![1, 2, 3]));
        let data_clone = Arc::clone(&data);

        // Spawn a thread that panics while holding a write lock
        let handle = thread::spawn(move || {
            let _data_ref = data_clone.write().unwrap();
            panic!("Intentional panic");
        });

        // The thread will panic and we catch it
        let _result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            let _ = handle.join();
        }));

        // The lock is now poisoned
        let read_result = data.read();
        assert!(read_result.is_err()); // Will be poisoned
    }

    #[test]
    fn test_condvar_producer_consumer() {
        let data = Arc::new(Mutex::new(vec![]));
        let condvar = Arc::new(Condvar::new());

        let data_clone = Arc::clone(&data);
        let condvar_clone = Arc::clone(&condvar);

        // Producer thread
        let producer = thread::spawn(move || {
            for i in 0..5 {
                {
                    let mut vec = data_clone.lock().unwrap();
                    vec.push(i);
                    // Notify waiting consumers
                    condvar_clone.notify_one();
                }
                thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        let data_clone = Arc::clone(&data);
        let condvar_clone = Arc::clone(&condvar);

        // Consumer thread
        let consumer = thread::spawn(move || {
            let mut vec = data_clone.lock().unwrap();
            while vec.len() < 5 {
                // Wait for notification
                vec = condvar_clone.wait(vec).unwrap();
            }
            assert_eq!(vec.len(), 5);
            vec.clone()
        });

        producer.join().unwrap();
        let result = consumer.join().unwrap();
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_condvar_broadcast() {
        let barrier = Arc::new(Mutex::new(0));
        let condvar = Arc::new(Condvar::new());

        let mut handles = vec![];

        // Spawn 3 consumer threads waiting for the same condition
        for i in 0..3 {
            let barrier_clone = Arc::clone(&barrier);
            let condvar_clone = Arc::clone(&condvar);

            let handle = thread::spawn(move || {
                let mut count = barrier_clone.lock().unwrap();
                // Wait until count reaches 3
                while *count < 3 {
                    count = condvar_clone.wait(count).unwrap();
                }
                println!("Thread {} woke up, count: {}", i, *count);
                i
            });
            handles.push(handle);
        }

        // Give threads time to start waiting
        thread::sleep(std::time::Duration::from_millis(100));

        // Producer thread increments counter and notifies all
        {
            let mut count = barrier.lock().unwrap();
            *count = 3;
            // Wake up all waiting threads
            condvar.notify_all();
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_condvar_timeout() {
        use std::time::Duration;

        let data = Arc::new(Mutex::new(false));
        let condvar = Arc::new(Condvar::new());

        let data_clone = Arc::clone(&data);
        let condvar_clone = Arc::clone(&condvar);

        // Thread waiting with timeout
        let waiter = thread::spawn(move || {
            let flag = data_clone.lock().unwrap();
            let result = condvar_clone
                .wait_timeout(flag, Duration::from_millis(100))
                .unwrap();
            // Timeout should occur since no one notifies
            result.1.timed_out()
        });

        let timed_out = waiter.join().unwrap();
        assert!(timed_out);
    }

    #[test]
    fn test_condvar_notification() {
        let data = Arc::new(Mutex::new(0));
        let condvar = Arc::new(Condvar::new());

        let data_clone = Arc::clone(&data);
        let condvar_clone = Arc::clone(&condvar);

        // Waiter thread
        let waiter = thread::spawn(move || {
            let mut value = data_clone.lock().unwrap();
            println!("Waiter: waiting for condition...");
            while *value != 42 {
                value = condvar_clone.wait(value).unwrap();
            }
            println!("Waiter: condition met! value = {}", *value);
            *value
        });

        // Notifier thread
        thread::sleep(std::time::Duration::from_millis(50));
        {
            let mut value = data.lock().unwrap();
            *value = 42;
            println!("Notifier: setting value to 42 and notifying");
            condvar.notify_one();
        }

        let result = waiter.join().unwrap();
        assert_eq!(result, 42);
    }
}

/*

use std::thread;

fn main() {
    let mut data = vec![1, 2, 3];

    for i in 0..3 {
        // Try to capture a mutable reference in multiple threads
        // This will fail to compile!
        thread::spawn(move || {
            data[i] += 1;
        });
    }

    // No data race can occur, this will not compile.
}

*/

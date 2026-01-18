use std::thread;
use std::sync::mpsc;  // Multi-Producer, Single-Consumer channel
use std::sync::Arc;   // Atomic Reference Counting for safe shared ownership

// Example 1: Basic thread spawning with ownership transfer
fn example_1_basic_spawning() {
    let data = vec![1, 2, 3, 4, 5];
    
    // Spawn a new thread using move closure to transfer ownership
    let handle = thread::spawn(move || {
        // Now the thread owns 'data'
        for item in &data {
            println!("Thread 1 processing: {}", item);
        }
    });
    
    // Wait for the thread to finish
    handle.join().unwrap();
    
    // 'data' is no longer accessible here - it was moved into the thread
    // This prevents data races!
}

// Example 2: Using channels to pass data between threads
fn example_2_channels() {
    // Create a channel for communication
    let (sender, receiver) = mpsc::channel();
    
    // Spawn a producer thread
    let producer = thread::spawn(move || {
        let numbers = vec![10, 20, 30, 40, 50];
        for num in numbers {
            // Send data through the channel
            sender.send(num).unwrap();
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    
    // In the main thread, receive data from the channel
    println!("\n=== Channel Communication ===");
    for received in receiver {
        println!("Main thread received: {}", received);
    }
    
    producer.join().unwrap();
}

// Example 3: Multiple threads sending through one channel
fn example_3_multiple_producers() {
    let (sender, receiver) = mpsc::channel();
    
    // Clone the sender for multiple threads
    let sender1 = sender.clone();
    let sender2 = sender.clone();
    drop(sender);  // Drop the original sender
    
    let thread1 = thread::spawn(move || {
        sender1.send("Message from thread 1").unwrap();
    });
    
    let thread2 = thread::spawn(move || {
        sender2.send("Message from thread 2").unwrap();
    });
    
    println!("\n=== Multiple Producers ===");
    for message in receiver {
        println!("Received: {}", message);
    }
    
    thread1.join().unwrap();
    thread2.join().unwrap();
}

// Example 4: Using Arc for shared read-only access
fn example_4_shared_immutable() {
    
    let shared_data = Arc::new(vec!["apple", "banana", "orange"]);
    
    let mut handles = vec![];
    
    // Create 3 threads that all read the same data
    for i in 1..=3 {
        let data_clone = Arc::clone(&shared_data);
        
        let handle = thread::spawn(move || {
            println!("\n=== Thread {} ===", i);
            for item in data_clone.iter() {
                println!("Thread {} sees: {}", i, item);
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
}

// Example 5: Full example combining channels and threading
fn example_5_complete() {
    println!("\n=== Complete Threading Example ===");
    
    let (sender, receiver) = mpsc::channel();
    
    // Worker threads that process data
    let worker1 = thread::spawn(move || {
        for i in 1..=3 {
            sender.send(format!("Worker1: Processing item {}", i)).unwrap();
            thread::sleep(std::time::Duration::from_millis(50));
        }
    });
    
    // Main thread receives and processes results
    for message in receiver {
        println!("{}", message);
    }
    
    worker1.join().unwrap();
}

fn main() {
    example_1_basic_spawning();
    example_2_channels();
    example_3_multiple_producers();
    example_4_shared_immutable();
    example_5_complete();
}
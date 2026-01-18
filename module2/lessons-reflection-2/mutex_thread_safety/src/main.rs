
use std::thread;
use std::sync::{Mutex, Arc};
use std::sync::atomic::{AtomicUsize, Ordering};

// Example 1: Basic Mutex usage - Counter shared between threads
fn example_1_basic_mutex() {
    println!("=== Example 1: Basic Mutex Counter ===");
    
    // Arc (Atomic Reference Counting) allows multiple owners
    // Mutex ensures only one thread can modify the data at a time
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    // Spawn 5 threads
    for i in 1..=5 {
        // Clone the Arc so each thread gets a reference to the same Mutex
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            // Acquire the lock (automatically releases when 'guard' goes out of scope)
            let mut num = counter_clone.lock().unwrap();
            
            // Now we have exclusive access to modify the value
            *num += 1;
            println!("Thread {} incremented counter to: {}", i, *num);
            
            // Lock is automatically released here
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Get the final value
    let final_value = *counter.lock().unwrap();
    println!("Final counter value: {}", final_value);
}

// Example 2: Mutex protecting a data structure
#[derive(Clone)]
struct BankAccount {
    balance: i32,
}

fn example_2_mutex_struct() {
    println!("\n=== Example 2: Mutex with Data Structure ===");
    
    let account = Arc::new(Mutex::new(BankAccount { balance: 1000 }));
    let mut handles = vec![];
    
    // Thread 1: Withdrawals
    for i in 1..=3 {
        let account_clone = Arc::clone(&account);
        
        let handle = thread::spawn(move || {
            let mut acc = account_clone.lock().unwrap();
            acc.balance -= 100;
            println!("Withdrawal {}: Balance is now {}", i, acc.balance);
            thread::sleep(std::time::Duration::from_millis(50));
        });
        
        handles.push(handle);
    }
    
    // Thread 2: Deposits
    for i in 1..=3 {
        let account_clone = Arc::clone(&account);
        
        let handle = thread::spawn(move || {
            let mut acc = account_clone.lock().unwrap();
            acc.balance += 50;
            println!("Deposit {}: Balance is now {}", i, acc.balance);
            thread::sleep(std::time::Duration::from_millis(50));
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_account = account.lock().unwrap();
    println!("Final balance: {}", final_account.balance);
}

// Example 3: Multiple operations on shared data
fn example_3_complex_operations() {
    println!("\n=== Example 3: Complex Operations with Mutex ===");
    
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut handles = vec![];
    
    // Thread that adds values
    for i in 1..=3 {
        let data_clone = Arc::clone(&data);
        
        let handle = thread::spawn(move || {
            let mut vec = data_clone.lock().unwrap();
            vec.push(i * 10);
            println!("Added {}: {:?}", i * 10, *vec);
        });
        
        handles.push(handle);
    }
    
    // Thread that reads values
    let data_clone = Arc::clone(&data);
    let read_handle = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(100));
        let vec = data_clone.lock().unwrap();
        println!("Reader sees: {:?}", *vec);
    });
    handles.push(read_handle);
    
    for handle in handles {
        handle.join().unwrap();
    }
}

// Example 4: Proper error handling with Mutex
fn example_4_error_handling() {
    println!("\n=== Example 4: Error Handling ===");
    
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data_clone = Arc::clone(&data);
    
    // Spawn a thread that panics while holding the lock
    let thread_handle = thread::spawn(move || {
        let mut vec = data_clone.lock().unwrap();
        vec.push(4);
        // If this thread panicked, the lock would be poisoned
        // panic!("Oops!");
    });
    
    thread_handle.join().unwrap();
    
    // We can still access the data because no panic occurred
    // Use if-let to ensure the lock guard is dropped properly
    if let Ok(vec) = data.lock() {
        println!("Successfully locked: {:?}", *vec);
    } else {
        println!("Lock was poisoned by a panicked thread");
    }
}

// Example 5: Using AtomicUsize for simple counters (more efficient than Mutex)
fn example_5_atomic_alternative() {
    println!("\n=== Example 5: Atomic Alternative for Counters ===");
    
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for i in 1..=10 {
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            // No lock needed - atomic operations are thread-safe
            let new_value = counter_clone.fetch_add(1, Ordering::SeqCst) + 1;
            println!("Thread {} incremented to: {}", i, new_value);
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final atomic counter: {}", counter.load(Ordering::SeqCst));
}

fn main() {
    example_1_basic_mutex();
    example_2_mutex_struct();
    example_3_complex_operations();
    example_4_error_handling();
    example_5_atomic_alternative();
}
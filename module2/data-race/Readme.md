# Data Race
## Objective
This lab aims to provide you with hands-on experience in using Rust's Mutex for thread-safe concurrent programming. By the end of this lab, you will understand the use of Mutex to protect shared data from data races in a multi-threaded environment.

https://www.coursera.org/learn/data-engineering-rust/ungradedLab/oKG9D/data-race/lab

Note: assuming that this github is the corrent one: https://github.com/nogibjj/rust-data-engineering/blob/main/data-race

## Instructions
**Step 1: Open your Rust programming environment.**
Open your preferred Rust IDE or text editor.

**Step 2: Create a new Rust project.**
Open your terminal and run cargo new mutex_lab to initiate a new Rust project.
**Note:** using data-race instead of mutex_lab like in github repo.

**Step 3: Navigate to the src/main.rs file.**
This file will contain the skeleton code for your project.

**Step 4: Implement Mutex and Thread Logic.**
Copy the given code snippet into your src/main.rs file.

Uncomment the first block of code that uses Mutex to protect the data vector.

Comment out the second block of code that tries to capture a mutable reference in multiple threads.

**Step 5: Save your project.**
Make sure all your changes are saved.

**Step 6: Compile and run the project.**
Open your terminal, navigate to your project directory, and run `cargo run --bin main`.

**Step 7: Verify the output.**
The output should show the modified data vector, incremented by each thread.
```bash
    Running with Mutex...
    Thread 0 acquired Mutex lock
    Final data with Mutex: Mutex { data: [2, 3, 4], poisoned: false, .. }
```

## Reflection Questions
### What is the purpose of using Mutex in this lab?
The purpose of using Mutex is to protect shared data from data races in a multi-threaded environment. Mutex (Mutual Exclusion) ensures that only one thread can access the protected data at a time by requiring a thread to acquire a lock before reading or modifying the data. In this lab, the Mutex protects a shared vector that is modified by multiple threads concurrently, guaranteeing thread-safe access and preventing race conditions where multiple threads might read and write the same data simultaneously, leading to corrupted or unpredictable results.

### How does Rust prevent data races when using Mutex?
Rust prevents data races when using Mutex through several mechanisms: (1) **Ownership System**: Mutex takes ownership of the data, and you must wrap shared data in Arc to share ownership across threads. (2) **Compile-time Borrow Checking**: Rust's borrow checker enforces that you cannot have multiple mutable references to the same data, even across threads. (3) **Runtime Mutual Exclusion**: The Mutex lock enforces that only one thread can access the protected data at a time—threads must call `.lock().unwrap()` to acquire exclusive access. (4) **Type Safety**: Rust's type system ensures that the data inside a Mutex can only be accessed through the proper synchronization mechanism, preventing accidental unsynchronized access. Together, these mechanisms guarantee that data races are impossible at compile-time and runtime.

### What happens when you try to compile the second block of code, and why?
When you try to compile the second block of code (which attempts to capture a mutable reference without using Mutex or Arc), the Rust compiler will throw a **borrow checker error**. Specifically, you will get an error like "cannot move value into closure more than once" or "value used after move". This happens because: (1) **Ownership Violation**: Each thread closure tries to take ownership of the mutable reference to the data, but Rust's ownership rules only allow one owner at a time. (2) **Multiple Mutable References**: The code attempts to create multiple mutable references to the same data across different threads, which violates Rust's fundamental rule: you cannot have multiple mutable references to the same data simultaneously. (3) **Move Semantics**: When you move a mutable reference into the first closure, it's no longer available to move into the second closure. This compile-time error is exactly what prevents data races—Rust catches unsafe code before it can run, making it impossible to accidentally create race conditions.

## Challenge Questions
### Can you modify the code to use read-write locks (RwLock) instead of a Mutex? What are the advantages and disadvantages?

**Yes, you can modify the code to use `RwLock` instead of `Mutex`.** RwLock (Read-Write Lock) allows multiple readers to access the data simultaneously, while only one writer can access it exclusively.

**Advantages of RwLock:**
- **Better Concurrency**: Multiple threads can read the data at the same time without blocking each other
- **Performance**: When you have many read-heavy workloads, RwLock is more efficient than Mutex
- **Fine-grained Access Control**: Distinguishes between read and write operations

**Disadvantages of RwLock:**
- **Overhead**: RwLock has higher overhead than Mutex due to complexity of managing multiple readers and a writer
- **Writer Starvation**: If many readers keep acquiring read locks, writers may be starved and unable to acquire a write lock
- **Complexity**: More complex to reason about compared to Mutex
- **Performance on Write-Heavy Workloads**: If most operations are writes, Mutex may be faster since RwLock adds unnecessary overhead

**Test Cases:**
The implementation includes 5 comprehensive test cases in [src/main.rs](src/main.rs) that verify RwLock behavior:
1. `test_rwlock_multiple_readers` - Verifies multiple threads can read simultaneously
2. `test_rwlock_exclusive_write` - Confirms write access blocks readers
3. `test_rwlock_writer_modifications` - Validates concurrent writes work correctly
4. `test_rwlock_vs_mutex_behavior` - Compares RwLock with Mutex expectations
5. `test_rwlock_poison_on_panic` - Demonstrates lock poisoning behavior

Run all tests with: `cargo test`

**Test Cases**
[See](RWLOCK_TESTS.md) for a test case details.


## Command Line Options

The program now supports command line arguments to switch between synchronization mechanisms:

**Run with Mutex (default):**
```bash
cargo run
```

**Run with RwLock:**
```bash
cargo run -- --rwlock
```

**View all available options:**
```bash
cargo run -- --help
```

**Run all tests:**
```bash
cargo test
```

### Example Output Comparison

**Mutex Implementation:**
```bash
$ cargo run --bin main
Running with Mutex...
Thread 2 acquired Mutex lock
Thread 0 acquired Mutex lock
Thread 1 acquired Mutex lock
Final data with Mutex: Mutex { data: [2, 3, 4], poisoned: false, .. }
```

**RwLock Implementation:**
```bash
$ cargo run --bin main -- --rwlock
Running with RwLock...
Thread 0 acquired RwLock write lock
Thread 2 acquired RwLock write lock
Thread 1 acquired RwLock write lock
Final data with RwLock: RwLock { data: [2, 3, 4], poisoned: false, .. }
```

### How would you handle potential deadlocks in a more complex application that uses multiple Mutexes?

**Deadlocks** occur when two or more threads are blocked indefinitely, waiting for each other to release locks. Here are several strategies to prevent deadlocks when using multiple Mutexes:

#### 1. **Lock Ordering (Most Common)**
Acquire locks in a consistent order across all threads. If all threads always acquire locks in the same order (e.g., lock_A then lock_B), circular wait conditions cannot occur.

```rust
// ❌ BAD: Different threads acquire locks in different orders
// Thread 1: Lock A -> Lock B
// Thread 2: Lock B -> Lock A  // DEADLOCK RISK!

// ✅ GOOD: All threads acquire locks in the same order
// Thread 1: Lock A -> Lock B
// Thread 2: Lock A -> Lock B  // No deadlock possible
fn safe_access(mutex_a: &Arc<Mutex<i32>>, mutex_b: &Arc<Mutex<i32>>) {
    let _guard_a = mutex_a.lock().unwrap();  // Always acquire A first
    let _guard_b = mutex_b.lock().unwrap();  // Then B
}
```

#### 2. **Use Try-Lock with Timeout**
Instead of blocking indefinitely with `.lock()`, use `.try_lock()` with a timeout to detect potential deadlocks:

```rust
use std::sync::TryLockError;

fn acquire_locks_safely(
    mutex_a: &Arc<Mutex<i32>>,
    mutex_b: &Arc<Mutex<i32>>,
) -> Result<(), TryLockError<()>> {
    loop {
        let lock_a = mutex_a.try_lock();
        if let Ok(mut guard_a) = lock_a {
            match mutex_b.try_lock() {
                Ok(guard_b) => {
                    // Both locks acquired safely
                    return Ok(());
                }
                Err(_) => {
                    // Failed to acquire B, release A and retry
                    drop(guard_a);
                    std::thread::sleep(std::time::Duration::from_millis(1));
                    continue;
                }
            }
        }
    }
}
```

#### 3. **Minimize Lock Duration**
Keep the time a thread holds a lock as short as possible. This reduces the probability of lock conflicts:

```rust
fn minimal_lock_duration(data: &Arc<Mutex<Vec<i32>>>) {
    // Good: Only lock when necessary
    let mut vec = data.lock().unwrap();
    vec.push(42);
    // Lock automatically released here
} // No lock held outside this scope
```

#### 4. **Avoid Nested Locks**
Prevent acquiring multiple locks in nested contexts when possible. Refactor code to flatten lock acquisition:

```rust
// ❌ BAD: Nested locks can cause deadlock
fn nested_locks(a: &Arc<Mutex<i32>>, b: &Arc<Mutex<i32>>) {
    let guard_a = a.lock().unwrap();
    // ... some code ...
    let guard_b = b.lock().unwrap();  // If another thread has B and wants A, deadlock!
}

// ✅ GOOD: Flatten the structure, acquire all locks upfront
fn safe_locks(a: &Arc<Mutex<i32>>, b: &Arc<Mutex<i32>>) {
    let guard_a = a.lock().unwrap();
    let guard_b = b.lock().unwrap();
    // Use both locks in the same scope
}
```

#### 5. **Use Channels Instead of Multiple Locks**
For producer-consumer patterns, channels are often safer than managing multiple Mutexes:

```rust
use std::sync::mpsc;

fn channel_based_approach() {
    let (sender, receiver) = mpsc::channel();
    
    // Producer thread
    std::thread::spawn(move || {
        for i in 0..10 {
            sender.send(i).unwrap();
        }
    });
    
    // Consumer thread
    std::thread::spawn(move || {
        for received in receiver {
            println!("Received: {}", received);
        }
    });
}
```

#### 6. **Use parking_lot Crate (Advanced)**
The `parking_lot` crate provides more efficient Mutex and RwLock implementations with deadlock detection:

```rust
// Add to Cargo.toml: parking_lot = "0.12"
use parking_lot::Mutex;

fn with_parking_lot(data: &Arc<Mutex<i32>>) {
    let mut guard = data.lock();
    *guard += 1;
    // No unwrap() needed - parking_lot never poisons locks
}
```

#### 7. **Design with Single Responsibility**
Limit each thread to owning and modifying data it's responsible for, reducing the need for multiple locks:

```rust
struct Account {
    balance: Mutex<i32>,
}

struct Bank {
    accounts: Vec<Arc<Account>>,
}

// Each thread handles one account, avoiding complex multi-lock scenarios
fn transfer_safe(from: &Arc<Account>, to: &Arc<Account>, amount: i32) {
    // Acquire in a consistent order based on account ID
    if from as *const _ < to as *const _ {
        let _from_guard = from.balance.lock().unwrap();
        let _to_guard = to.balance.lock().unwrap();
    } else {
        let _to_guard = to.balance.lock().unwrap();
        let _from_guard = from.balance.lock().unwrap();
    }
}
```

#### **Summary: Prevention Strategies**
1. ✅ Use consistent **lock ordering** across all threads
2. ✅ **Minimize lock duration** - hold locks as briefly as possible
3. ✅ Use **try_lock()** with timeout instead of blocking
4. ✅ Avoid **nested locks** when possible
5. ✅ Consider **channels** for communication patterns
6. ✅ Use libraries like **parking_lot** for better tools
7. ✅ **Design for simplicity** - avoid complex multi-lock scenarios

### Can you extend the code to use conditional variables to synchronize thread execution?

**Yes!** Condition Variables (Condvar) are powerful synchronization primitives that allow threads to wait until a specific condition is met, then be notified to wake up. In Rust, `std::sync::Condvar` works with `Mutex` to provide efficient thread synchronization.

#### **What Are Condition Variables?**

A Condvar allows a thread to:
1. **Wait** - Release the lock and sleep until notified
2. **Notify** - Wake up one waiting thread
3. **Broadcast** - Wake up all waiting threads

#### **Basic Pattern**

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn main() {
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
            thread::sleep(Duration::from_millis(10));
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
        println!("Got all data: {:?}", vec);
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
```

#### **Key Methods**

**1. `wait(guard)` - Block Until Notified**
```rust
let mut data = mutex.lock().unwrap();
// Automatically releases lock while waiting
data = condvar.wait(data).unwrap();
// Lock is reacquired after notification
```

**2. `notify_one()` - Wake One Thread**
```rust
condvar.notify_one();  // Wakes the first waiting thread
```

**3. `notify_all()` - Wake All Threads**
```rust
condvar.notify_all();  // Wakes all waiting threads
```

**4. `wait_timeout(guard, duration)` - Wait With Timeout**
```rust
let result = condvar.wait_timeout(data, Duration::from_secs(1)).unwrap();
if result.1.timed_out() {
    println!("Timeout occurred!");
}
```

#### **Real-World Example: Producer-Consumer Queue**

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

struct Queue {
    data: Arc<Mutex<Vec<i32>>>,
    condvar: Arc<Condvar>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            data: Arc::new(Mutex::new(Vec::new())),
            condvar: Arc::new(Condvar::new()),
        }
    }

    fn push(&self, item: i32) {
        let mut vec = self.data.lock().unwrap();
        vec.push(item);
        self.condvar.notify_one();  // Wake one consumer
    }

    fn pop(&self) -> i32 {
        let mut vec = self.data.lock().unwrap();
        while vec.is_empty() {
            vec = self.condvar.wait(vec).unwrap();
        }
        vec.pop().unwrap()
    }
}
```

#### **Example: Multi-Consumer Barrier**

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn main() {
    let barrier = Arc::new(Mutex::new(0));
    let condvar = Arc::new(Condvar::new());

    let mut handles = vec![];

    // Spawn 3 threads waiting for the same condition
    for i in 0..3 {
        let barrier_clone = Arc::clone(&barrier);
        let condvar_clone = Arc::clone(&condvar);

        let handle = thread::spawn(move || {
            let mut count = barrier_clone.lock().unwrap();
            println!("Thread {} waiting...", i);
            
            // Wait until count reaches 3
            while *count < 3 {
                count = condvar_clone.wait(count).unwrap();
            }
            println!("Thread {} released!", i);
        });
        handles.push(handle);
    }

    thread::sleep(Duration::from_millis(100));

    // Signal all waiting threads
    {
        let mut count = barrier.lock().unwrap();
        *count = 3;
        condvar.notify_all();  // Wake ALL threads at once
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

#### **Test Cases in Implementation**

The codebase includes 4 comprehensive Condvar tests in [src/main.rs](src/main.rs):

1. **`test_condvar_producer_consumer`** - Producer pushes data, consumer waits and wakes when data arrives
2. **`test_condvar_broadcast`** - Multiple threads wait on same condition, all wake with `notify_all()`
3. **`test_condvar_timeout`** - Thread waits with timeout that expires
4. **`test_condvar_notification`** - Simple notification between two threads

Run tests with: `cargo test`

#### **Advantages of Condition Variables**

✅ **Efficient Waiting** - Threads sleep without busy-waiting  
✅ **Event-Driven** - Threads wake only when condition is met  
✅ **Producer-Consumer** - Perfect for queues and buffer management  
✅ **Synchronization Barriers** - Multiple threads wait for single event  
✅ **Timeout Support** - Can fail gracefully if event doesn't happen  

#### **Disadvantages**

❌ **Spurious Wakeups** - Always check condition in a `while` loop, not `if`  
❌ **More Complex** - Requires careful understanding of lock/wait semantics  
❌ **Easy to Misuse** - Incorrect conditions can cause deadlocks  

#### **Common Pitfall: Spurious Wakeups**

```rust
// ❌ BAD: Can wake up without condition being true
if !condition {
    condvar.wait(...)?;
}

// ✅ GOOD: Always check in a loop
while !condition {
    condition = condvar.wait(condition)?;
}
```

#### **Summary: When to Use Each Synchronization Primitive**

| Primitive | Use Case |
|-----------|----------|
| **Mutex** | Simple mutual exclusion, protecting shared data |
| **RwLock** | Read-heavy workloads, many readers few writers |
| **Condvar** | Event signaling, producer-consumer patterns, barriers |
| **Channels** | Thread communication, message passing |



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
Open your terminal, navigate to your project directory, and run cargo run.

**Step 7: Verify the output.**
The output should show the modified data vector, incremented by each thread.
```bash
    Mutex { data: [2, 3, 4], poisoned: false, .. }
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

### How would you handle potential deadlocks in a more complex application that uses multiple Mutexes?

### Can you extend the code to use conditional variables to synchronize thread execution?


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

### How does Rust prevent data races when using Mutex?

### What happens when you try to compile the second block of code, and why?

## Challenge Questions
### Can you modify the code to use read-write locks (RwLock) instead of a Mutex? What are the advantages and disadvantages?

### How would you handle potential deadlocks in a more complex application that uses multiple Mutexes?

### Can you extend the code to use conditional variables to synchronize thread execution?


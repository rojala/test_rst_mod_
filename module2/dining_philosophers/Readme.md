# Lab: Dining Philosophers

## Objective
In this lab, you'll explore the classic concurrency problem known as the "Dining Philosophers." You'll learn how to use Rust's primitives like Arc and Mutex for thread-safe programming. The code simulates 15 philosophers sharing 4 forks, highlighting the complexity of avoiding deadlocks.

## Instructions
### Step 1: Repository Access
Head over to the repository at https://github.com/nogibjj/rust-data-engineering.

### Step 2: Launch GitHub Codespace
* Click on the green "Code" button and then select "Open with Codespaces."
* Click on "+ New codespace." This will open up a new GitHub Codespace, a cloud-based VS Code environment.
Note: Make sure you have a GitHub account and permissions to create Codespaces.

###Step 3: Navigate to Project
In the Codespace, navigate to rust-data-engineering/main/dining-philosophers in the file explorer.

### Step 4: Review Code
Open the src/main.rs file.

Take a moment to understand the code, paying attention to how Mutex and Arc are used.

#### Arc (Atomic Reference Counting)
Arc allows multiple threads to share ownership of the same data:

Line 77-83: Creates 4 forks, each wrapped in Arc<Fork>. This allows multiple philosophers to hold references to the same fork.
Line 107-109: Arc::clone(&forks[left]) and Arc::clone(&forks[right]) create new references to the same underlying fork without copying the data. The reference count is atomically incremented.
Line 121: When philosophers move into threads (thread::spawn(move || ...)), they take ownership of their Arc references, keeping the forks alive across thread boundaries.

#### Mutex (Mutual Exclusion)
Mutex ensures only one thread can access the fork at a time:

Line 30: Each fork contains Mutex<()> - the () represents the lock itself (no data needs to be protected, just exclusive access).
Line 56-59: first_fork.mutex.lock().unwrap() blocks the thread until it can acquire exclusive access to the fork. The guard (_first_guard) holds the lock until it goes out of scope.
Line 67: When the guard is dropped at the end of the eat() function, the mutex is automatically unlocked, allowing other philosophers to acquire it.

#### Deadlock Prevention
The code avoids deadlock by having even/odd numbered philosophers pick up forks in different orders (lines 53-56), preventing circular wait conditions.

### Step 5: Compile
Open the integrated terminal (View -> Terminal).

Run cargo build to compile the program.

### Step 6: Execute
Run cargo run to execute the program. Observe the output and timing metrics.
``` bash
    Dining Philosophers Problem:  15 Philosophers, 4 Forks...Yikes!!
    Jürgen Habermas picked up fork 0.
    Jürgen Habermas picked up fork 1.
    Jürgen Habermas is eating.
    Karl Marx picked up fork 2.
    Karl Marx picked up fork 3.
    Karl Marx is eating.
    Jürgen Habermas finished eating.
    Jürgen Habermas put down fork 0.
    Jürgen Habermas put down fork 1.
    Karl Marx finished eating.
    Karl Marx put down fork 2.
    Karl Marx put down fork 3.
    Friedrich Engels picked up fork 2.
    Friedrich Engels picked up fork 1.
    Friedrich Engels is eating.
    Thomas Piketty picked up fork 0.
    Thomas Piketty picked up fork 3.
    Thomas Piketty is eating.
    Friedrich Engels finished eating.
    Friedrich Engels put down fork 2.
    Friedrich Engels put down fork 1.
    Thomas Piketty finished eating.
    Thomas Piketty put down fork 0.
    Thomas Piketty put down fork 3.
    Michel Foucault picked up fork 0.
    Michel Foucault picked up fork 1.
    Michel Foucault is eating.
    Socrates picked up fork 2.
    Michel Foucault finished eating.
    Michel Foucault put down fork 0.
    Michel Foucault put down fork 1.
    Socrates picked up fork 1.
    Socrates is eating.
    Aristotle picked up fork 0.
    Aristotle picked up fork 3.
    Aristotle is eating.
    Socrates finished eating.
    Socrates put down fork 2.
    Socrates put down fork 1.
    Aristotle finished eating.
    Aristotle put down fork 0.
    Aristotle put down fork 3.
    Heraclitus picked up fork 2.
    Heraclitus picked up fork 1.
    Heraclitus is eating.
    Diogenes picked up fork 0.
    Diogenes picked up fork 3.
    Diogenes is eating.
    Heraclitus finished eating.
    Heraclitus put down fork 2.
    Heraclitus put down fork 1.
    Diogenes finished eating.
    Plato picked up fork 2.
    Diogenes put down fork 0.
    Diogenes put down fork 3.
    Pythagoras picked up fork 0.
    Pythagoras picked up fork 1.
    Pythagoras is eating.
    Plato picked up fork 3.
    Plato is eating.
    Pythagoras finished eating.
    Pythagoras put down fork 0.
    Pythagoras put down fork 1.
    Plato finished eating.
    Plato put down fork 2.
    Plato put down fork 3.
    Democritus picked up fork 2.
    Democritus picked up fork 3.
    Democritus is eating.
    Epicurus picked up fork 0.
    Epicurus picked up fork 1.
    Epicurus is eating.
    Democritus finished eating.
    Democritus put down fork 2.
    Democritus put down fork 3.
    Zeno of Citium picked up fork 2.
    Epicurus finished eating.
    Epicurus put down fork 0.
    Epicurus put down fork 1.
    Zeno of Citium picked up fork 1.
    Zeno of Citium is eating.
    Zeno of Citium finished eating.
    Zeno of Citium put down fork 2.
    Zeno of Citium put down fork 1.
    Thales of Miletus picked up fork 2.
    Thales of Miletus picked up fork 3.
    Thales of Miletus is eating.
    Thales of Miletus finished eating.
    Thales of Miletus put down fork 2.
    Thales of Miletus put down fork 3.
    Total time: 9.002582782s
``` 

## Reflection Questions
### How does Mutex help in safely sharing the fork between philosophers?

### What role does Arc play in this program?

### How is deadlock avoided in this example?

## Challenge Questions

### How would you modify the program to include more forks or philosophers?

### Can you implement a feature to dynamically add philosophers or forks during runtime?

### How would you measure the "hunger level" of each philosopher and implement a priority system?
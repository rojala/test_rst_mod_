# External Lab: Fruit Salad Creation with VecDeque in GitHub Codespaces

Objective: The objective of this lab is to explore the functionality of Rust's VecDeque, a double-ended queue data structure. By the end of the lab, you should be familiar with creating, manipulating, and printing VecDeque data structures.

Instructions:

* Step 1: Navigate to the lab repository in GitHub Codespaces using the following URL: 
https://github.com/nogibjj/rust-data-engineering/tree/main/vecdeque-fruit-salad
.

* Step 2: Open the project in your Codespaces environment. You'll find the code for this lab in the file named src/main.rs in the project directory.

* Step 3: A Makefile should already be in your project directory, which will be used to build and run your project.

* Step 4: Open a terminal in Codespaces and navigate to the project directory.

* Step 5: Run the command make all to execute the program. This command will format your code, check for any linter warnings, run any tests, and finally execute your program.

* Step 6: Observe the output. You should see a randomized list of fruits printed in the console with additional fruits added to both ends - this is your fruit salad!

* To understand how the environment is setup, you can inspect the .devcontainer files 
here
.

# Reflection Questions:

## What is a VecDeque in Rust and how is it different from a Vector or a LinkedList?

A VecDeque (short for vector double-ended queue) is a growable ring buffer provided by Rust's standard library. It allows efficient insertion and removal of elements from both the front and the back of the queue.

### Key characteristics:
* Efficient front and back operations: Unlike Vec, which is fast at the back but slow at the front (due to shifting elements), VecDeque is optimized for both ends.
* Backed by a Vec internally, but uses a circular buffer to avoid shifting elements.
* Useful for queue-like behavior, especially when you need to push/pop from both ends.

### How is VecDeque different from Vec and LinkedList?

| Feature | Vec | VecDeque | LinkedList|
|------|---------|-------|-------|
| Memory layout | Contiguous | Ring buffer | Node-based |
|Front insert/remove|Slow (O(n))|Fast (O(1))|Fast (O(1))|
|Back insert/remove|Fast (O(1))|Fast (O(1))|Fast (O(1))|
|Random access|Fast (O(1))|Fast (O(1))|Slow (O(n))
|Use case|Stack, array|Queue, deque|Rarely needed in Rust (often replaced by VecDeque)|

## What is the significance of converting VecDeque to a Vector and then back to VecDeque in the program?
Why convert VecDeque to Vec and back?
This is typically done when you want to shuffle the elements. The standard shuffle function from rand works on slices (&mut [T]), which VecDeque does not expose directly due to its ring buffer layout.

So the conversion looks like:
```
let mut vec: Vec<_> = deque.into_iter().collect();
vec.shuffle(&mut rng);
let deque: VecDeque<_> = vec.into_iter().collect();
```
This allows you to:
* Convert to Vec for easy shuffling.
* Convert back to VecDeque to retain efficient front/back operations.

## Why do we push "Pomegranate" to the front of the queue and "Fig" and "Cherry" to the back of the queue after shuffling?
This is likely part of a queue manipulation pattern. Here's a possible interpretation:

* "Pomegranate" to the front: Maybe it's a high-priority item that should be processed first.
* "Fig" and "Cherry" to the back: These could be lower-priority or simply added in normal FIFO order.

This kind of logic is common in task queues, where:
* Urgent tasks go to the front.
* Regular tasks go to the back.

# Challenge Questions:

## Can you modify the program to allow the user to add fruits to either end of the queue after shuffling?

## The SliceRandom trait provides a method choose(&self, rng: &R) -> Option<&T>. Can you use this to select a random fruit from the salad?

## Can you adjust the program to remove a fruit from either end of the queue, displaying the name of the removed fruit and the state of the queue afterwards?
# Objective
This lab is designed to help you understand Rust's Vec (vector) data structure and its practical applications. By the end of this lab, you should be able to use Vectors to manage a dynamic list of fruits.

# Instructions
* Step 1: Open your Rust programming environment. Ensure that you have your Rust IDE or text editor open and ready for development.
    * Use github codespace version

* Step 2: Create a new Rust project.
    * Open your terminal and run cargo new fruit_vector_lab to create a new Rust project.
    * See module2/fruit_vector_lab

* Step 3: Navigate to the src/main.rs file.
    * This file should contain the skeleton code for your project.

* Step 4: Implement Vector operations.
    * Create a Vector called fruit_salad that contains a list of fruits like "apple", "banana", "cherry", etc.
    * Add a new fruit to the Vector using the push method.
        * Mansikka and mustikka added  

* Remove the last fruit from the Vector using the pop method.

* Iterate through the Vector and print each fruit.

* Step 5: Save your project.
    * Make sure all your changes are saved.

* Step 6: Compile and run the project.
    * Open your terminal and navigate to your project directory.
    * Run the cargo run command to compile and execute your program.

* Step 7: Verify the output.
    * The output in the console should show the initial list of fruits, the list after adding a new fruit, and finally the list after removing the last fruit.
    * Example output:
    ``` text
    Original fruit salad: ["apple", "banana", "cherry", "dates", "elderberries"]
        apple
        banana
        cherry
        dates
        elderberries
        figs
        mansikka
    ```

# Reflection Questions
## 1. How does the push method work in a Vector in Rust?
The push method on a Rust Vec<T> appends a new element to the end of the vector, taking ownership of the value and growing the vector’s length by one. It runs in amortized constant time and may trigger a reallocation if the vector’s capacity is full.

https://doc.rust-lang.org/std/vec/struct.Vec.html


## 2. What does the pop method do, and what does it return?
Vec::pop removes the last element from a vector and returns it as an Option<T>. If the vector has at least one element, you get Some(value); if it’s empty, you get None.

https://www.slingacademy.com/article/adding-and-removing-elements-from-a-vec-t-with-push-pop-insert-and-remove/?utm_source=copilot.com#google_vignette

## 3. Why is it necessary to declare the Vector as mutable for these operations?
Because push, pop, and similar methods change the contents or structure of the vector, Rust requires the vector to be declared as mut so the compiler knows you intend to modify it. This follows Rust’s core rule: you cannot mutate data through an immutable binding.

https://doc.rust-lang.org/std/vec/struct.Vec.html


# Challenge Questions
## 1. Can you implement a function that takes a Vector and a fruit name as parameters and removes that specific fruit from the Vector?

**Solution Implemented:**

The `remove_fruit()` function takes a mutable Vector reference and a fruit name, then removes the first occurrence of that fruit:

```rust
fn remove_fruit(vector: &mut Vec<&str>, fruit_name: &str) -> bool {
    if let Some(pos) = vector.iter().position(|&f| f == fruit_name) {
        vector.remove(pos);
        true
    } else {
        false
    }
}
```

**Usage with Clap:**
The program uses Clap for CLI argument handling:
```bash
cargo run -- --fruit banana
cargo run -- -f apple
```

**How it works:**
- Uses `iter().position()` to find the fruit in the vector
- Returns `true` if the fruit was found and removed, `false` otherwise
- Clap parses the fruit name from command-line arguments
- The function is called from main with the user-provided fruit name

### Unit Tests

Comprehensive unit tests have been implemented to verify the `remove_fruit()` function:

```bash
cargo test
```

**Test Coverage:**

1. **test_remove_existing_fruit** - Verifies removing a fruit that exists in the vector
2. **test_remove_first_occurrence_of_duplicate** - Ensures only the first occurrence is removed when duplicates exist
3. **test_remove_nonexistent_fruit** - Confirms the function returns false when fruit is not found
4. **test_remove_from_empty_vector** - Tests behavior with an empty vector
5. **test_remove_from_single_element_vector** - Tests removing the only element in the vector
6. **test_remove_first_fruit** - Verifies removing the first element
7. **test_remove_last_fruit** - Verifies removing the last element

## 2. How would you modify the program to sort the fruits alphabetically?

## 3. Can you extend the program to count the occurrences of each fruit in the Vector?
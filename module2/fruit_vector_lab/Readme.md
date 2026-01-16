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

Reflection Questions
How does the push method work in a Vector in Rust?

What does the pop method do, and what does it return?

Why is it necessary to declare the Vector as mutable for these operations?

Challenge Questions
Can you implement a function that takes a Vector and a fruit name as parameters and removes that specific fruit from the Vector?

How would you modify the program to sort the fruits alphabetically?

Can you extend the program to count the occurrences of each fruit in the Vector?
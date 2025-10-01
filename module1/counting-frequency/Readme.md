# Counting Frequency with Rust HashMaps

Objective: The goal of this lab is to provide an understanding of Rust's HashMap data structure and its application. By the end of this lab, you will be able to use HashMaps to count the frequency of each element in a vector.

## Instructions:

* Step 1: Open your Rust programming environment.
* Step 2: You will see a src/main.rs file which contains the starter code for this lab.
* Step 3: Review the existing code. The logic function takes a vector of integers as input and calculates the frequency of each integer using a HashMap. The frequency of each number is then returned as a vector of tuples.
* Step 4: Save the file in your project directory.
* Step 5: A Makefile should be present in your project directory that is used to build and run your project.
* Step 6: Open a terminal/command prompt and navigate to your project directory.
* Step 7: Run the command make all to build and run the program. This command will format your code, check for any linter warnings, run any tests, and finally execute your program.
* Step 8: You should see the frequency of each number printed in the console.

## Reflection Questions:

* How is HashMap used in this program and what is its function?
* Why is or_insert(0) used in frequencies.entry(num).or_insert(0)?
* How does the program ensure that each number and its frequency are correctly paired in the final result?

## Challenge Questions:

* Can you modify the program to accept input from the user and then calculate the frequency of each integer?
* Can you extend this concept to count the frequency of words in a sentence?
* How would you modify the program to sort the final result by frequency?

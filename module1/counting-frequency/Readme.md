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

(Lab id fopgeyplnzcb)

## Reflection Questions:

* How is HashMap used in this program and what is its function?

    HashMap is used to store the frequency of each number in the input vector. It maps each unique number (i32) to the number of times it appears (u32).

* Why is or_insert(0) used in frequencies.entry(num).or_insert(0)?

    frequencies.entry(num):

    * This accesses the entry for the key num in the HashMap.
    * If num is already a key, it returns a mutable reference to its value.
    * If num is not present, it prepares to insert it.

    .or_insert(0):
    * If num is not already in the map, insert it with the value 0.
    * Returns a mutable reference to the value (either existing or newly inserted).

    (Bonus) *frequency += 1;:
    * Dereferences the mutable reference and increments the value by 1.

* How does the program ensure that each number and its frequency are correctly paired in the final result?
    
    After counting, the program builds the result vector:

        for (num, frequency) in frequencies {
            result.push((num, frequency));
        }
    
    This loop:
    * Iterates over each (key, value) pair in the HashMap.
    * Pushes a tuple (num, frequency) into the result vector.

    Since HashMap maintains the correct mapping of each number to its frequency, this guarantees that each tuple in the result vector is accurate.

## Challenge Questions:

* Can you modify the program to accept input from the user and then calculate the frequency of each integer?
* Can you extend this concept to count the frequency of words in a sentence?
* How would you modify the program to sort the final result by frequency?

# Lab weighted rating of programming languages

Objective: This lab is designed to help you get a better understanding of Rust's HashMap data structure and its practical applications. By the end of this lab, you should be able to use HashMaps to create a weighted rating for programming languages based on their age.

[Lab link](https://www.coursera.org/learn/data-engineering-rust/ungradedLab/fQq6m/weighting-programming-languages-with-rust-hashmaps/lab)

## Instructions:

* Step 1: Open your Rust programming environment.
* Step 2: The src/main.rs file should be open, containing the code for this lab.
* Step 3: Review the existing code. The init_languages function initializes a HashMap containing programming languages and the years they were created. The calculate_weights function then calculates a weight for each language based on how many years it's been active.
* Step 4: Save your project.
* Step 5: Your project directory should have a Makefile for building and running your project.
* Step 6: Open a terminal/command prompt and navigate to your project directory.
* Step 7: Run the make all command to compile and run your program. This command will format your code, check for linter warnings, run any tests, and finally run your program.
* Step 8: The output in the console should show the weight of each language from 1-100, where 1 is the newest and 100 is the oldest.

## Reflection Questions:

* How does the calculate_weights function determine the weight of each language?

    The function calculates a **weight between 1 and 100** for each language based on how many years it has been active. Here's the process:

    - It first converts the creation year into **years active** by subtracting the creation year from the current year.
    - Then it finds the **minimum and maximum** values among all years active.
    - Each language's years active is **normalized** to a value between 0.0 and 1.0 using:

    $$
    \text{normalized} = \frac{\text{years\_active} - \text{min}}{\text{max} - \text{min}}
    $$

    - This normalized value is scaled to a range of **1 to 100** using:

    $$
    \text{weight} = (\text{normalized} \times 99) + 1
    $$

    So, the **longer a language has been active**, the **higher its weight**.


* How does the code ensure that the weights are normalized between 1 and 100?

    It uses this line:

    ```rust
    let weight = (normalized_year * 99.0) as i32 + 1;
    ```

    - `normalized_year` is a float between 0.0 and 1.0.
    - Multiplying by 99 gives a range from 0.0 to 99.0.
    - Adding 1 shifts the range to **1.0 to 100.0**.
    - Casting to `i32` gives an integer weight between **1 and 100**.

    This ensures all weights fall within the desired range.

* Why does the code use values_mut when updating the years in the calculate_weights function?

    ```rust
    for year in years_active.values_mut() {
        *year = current_year - *year;
    }
    ```

    - `values_mut()` gives **mutable references** to the values in the `HashMap`.
    - This allows the function to **modify the values in-place**.
    - In this case, it replaces each creation year with the number of years active.

    Without `values_mut()`, the code would only have read access and couldn’t update the values.

## Challenge Questions:

* Can you modify the program to accept new languages from the user, along with the year they were created, and then include them in the weight calculation?
* How would you modify the program to sort the languages by their weights?
* Can you extend the weight calculation to take into account other factors, such as the number of users or the growth rate of each language?

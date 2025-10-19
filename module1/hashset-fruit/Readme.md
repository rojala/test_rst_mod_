# External GitHub Lab: Generating Unique Fruits with Rust and HashSet

    Objective: In this lab, you'll use Rust's HashSet data structure to generate a unique set of fruits. By the end of this lab, you should understand how to use HashSet and why it's useful for ensuring uniqueness.

## Instructions:

1. Step 1: Go to the following GitHub repository:

https://github.com/nogibjj/rust-data-engineering

2. Step 2: Click the green "Code" button and then select "Open with GitHub Codespaces". Follow the instructions to create a new Codespace.

3. Step 3: Once your Codespace is ready, navigate to the hashset-fruit directory.

4. Step 4: Open the src/main.rs file and examine the provided code. The generate_fruit() function generates a random fruit string, and the main() function uses a HashSet to store unique instances of these fruits.

5. Step 5: In the Codespaces terminal, compile the program by running cargo build.

6. Step 6: Run the program by using cargo run in the terminal. The program will generate 100 random fruits and add them to a HashSet, which automatically removes any duplicates. The program then prints the number of unique fruits that were generated.

## Reflection Questions:

1. How does the generate_fruit() function ensure a random fruit is chosen every time it's called?

2. What is the purpose of using a HashSet in this program?

3. What would happen if you changed the HashSet to a different collection type, such as a Vec or LinkedList?

## Challenge Questions:

1. Modify the program to generate a user-specified number of random fruits. Hint: you'll need to use command-line arguments to get input from the user.

2. How would you modify the program to keep track of how many times each fruit is generated? Hint: you might need to use a different collection type.

3. Can you adjust the program to print out the list of unique fruits at the end?

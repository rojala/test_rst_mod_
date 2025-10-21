# External GitHub Lab: Generating Fruit Salad with Rust and BinaryHeap

Objective: In this lab, you will use the BinaryHeap data structure in Rust to generate a fruit salad with a priority for "Fig". By the end of this lab, you should understand how to use BinaryHeap and how the priority queue operates.

## Instructions:

1. Step 1: Go to the following GitHub repository: 
https://github.com/nogibjj/rust-data-engineering

2. Step 2: Click the green "Code" button and then select "Open with GitHub Codespaces". Follow the instructions to create a new Codespace.

3. Step 3: Once your Codespace is ready, navigate to the binaryheap-fruit directory.

4. Step 4: Open the src/main.rs file and examine the provided code. The main() function creates a BinaryHeap and fills it with a random selection of fruits, with a priority for "Fig".

5. Step 5: In the Codespaces terminal, compile the program by running cargo build.

6. Step 6: Run the program by using cargo run in the terminal. The program will generate a random selection of fruits and add them to a BinaryHeap, which operates as a priority queue. The program then prints the BinaryHeap of fruits.

## Reflection Questions:

1. How does the use of BinaryHeap affect the output of the program compared to if a HashSet or BTreeSet was used?

2. What are the benefits of using a BinaryHeap over other collection types?

3. What would happen if you changed the BinaryHeap to a different collection type, such as a Vec or LinkedList?

## Challenge Questions:

1. Modify the program to include the capability to remove a fruit from the BinaryHeap. Hint: you'll need to use command-line arguments to get input from the user.

2. Can you adjust the program to print out the list of unique fruits at the end in reverse order? Hint: look up how to iterate over a BinaryHeap in reverse order in Rust.

3.How would you modify the program to keep track of how many times each fruit is generated? Hint: you might need to use a different collection type.

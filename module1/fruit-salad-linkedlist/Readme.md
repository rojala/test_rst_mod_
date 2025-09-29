# External Lab: Fruit Salad Creation with LinkedList in GitHub Codespaces
Objective: The objective of this lab is to familiarize you with Rust's LinkedList data structure. By the end of the lab, you should be able to create a LinkedList, manipulate its contents, and print them out.

## Instructions:

* Step 1: Navigate to the lab repository in GitHub Codespaces using the following URL: 
https://github.com/nogibjj/rust-data-engineering/tree/main/linked-list-fruit-salad
.

* Step 2: Open the project in your Codespaces environment. You'll find the code for this lab in the file named src/main.rs in the project directory.

* Step 3: A Makefile should already be in your project directory, which will be used to build and run your project.

* Step 4: Open a terminal in Codespaces and navigate to the project directory.

* Step 5: Run the command make all to execute the program. This command will format your code, check for any linter warnings, run any tests, and finally execute your program.

* Step 6: Observe the output. You should see a randomized list of fruits printed in the console with additional fruits added to both ends - this is your fruit salad!

To understand how the environment is set up, you can inspect the .devcontainer files here.

### Reflection Questions:

* What is a LinkedList in Rust and how is it different from a Vector or a VecDeque?

* In what situations might you prefer to use a LinkedList over other data structures?
 
* Why is there a need to convert the LinkedList to a Vec and then back to LinkedList in this program?

### Challenge Questions:

* Can you modify the program to allow the user to add fruits at any position in the LinkedList after shuffling?

* The SliceRandom trait provides a method choose(&self, rng: &R) -> Option<&T>. Can you use this to select a random fruit from the salad?

* Can you adjust the program to remove a fruit from any position in the LinkedList, displaying the name of the removed fruit and the state of the list afterwards?


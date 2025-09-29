# Command Line Fruit Salad Creator in GitHub Codespaces

Objective: The objective of this lab is to familiarize you with creating a command line interface (CLI) application in Rust that creates a fruit salad with a specified number of fruits. By the end of the lab, you should be able to understand how to parse command line arguments and utilize them in your Rust programs.

## Instructions:
* Step 1: Navigate to the lab repository in GitHub Codespaces using the following URL: 
https://github.com/nogibjj/rust-data-engineering/tree/main/cli-salad.
* Step 2: Open the project in your Codespaces environment. You'll find the main program in the file named src/main.rs and the fruit salad creation logic in src/lib.rs in the project directory.
* Step 3: A Makefile should already be in your project directory, which will be used to build and run your project.
* Step 4: Open a terminal in Codespaces and navigate to the project directory.
* Step 5: Run the command make all to execute the program. This command will format your code, check for any linter warnings, run any tests, and finally execute your program. Don't forget to provide the required number of fruits as a command-line argument while running the program.
* Step 6: Observe the output. You should see a fruit salad created with the number of fruits you specified in your command line arguments.

To understand how the environment is set up, you can inspect the .devcontainer files 
here.

## Reflection Questions:

### How does the clap library help in creating a command-line interface (CLI) in Rust?

### How is the Vec fruits shuffled in the create_fruit_salad function?

### Why is there a need to convert the fruits Vec into an iterator and then take only a specific number of fruits?

## Challenge Questions:

### Can you modify the program to allow the user to specify which fruits they want in their salad as command-line arguments?

### Can you modify the program to print the fruits in alphabetical order after creating the salad?

### How might you adjust the program to handle invalid input from the user, such as a request for more fruits than are available?
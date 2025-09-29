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
    It streamlines argument parsing, provides built-in help and error messages, and supports complex CLI structures—all with minimal boilerplate.

### How is the Vec fruits shuffled in the create_fruit_salad function?
    It is done by using SliceRandom traits suffle.

### Why is there a need to convert the fruits Vec into an iterator and then take only a specific number of fruits?

    After shuffling the fruits vector, the goal is to return a new Vec<String> containing only the first num_fruits items.
    
    fruits.into_iter().take(num_fruits).collect()

    1. .into_iter()
    Converts the Vec<String> into an iterator that takes ownership of its elements.

    This is necessary because we want to consume the original vector and build a new one from selected items.

    2. .take(num_fruits)
    Limits the iterator to yield only the first num_fruits items.

    If num_fruits is greater than the length of the vector, it just yields as many as available.

    3. .collect()
    Collects the items from the iterator into a new Vec<String>.

## Challenge Questions:

* Can you modify the program to allow the user to specify which fruits they want in their salad as command-line arguments?

* Can you modify the program to print the fruits in alphabetical order after creating the salad?

* How might you adjust the program to handle invalid input from the user, such as a request for more fruits than are available?

    Dynamically/runtime in code or statically e.g.

        #[arg(short, long, value_parser = clap::value_parser!(usize).range(1..=10))]
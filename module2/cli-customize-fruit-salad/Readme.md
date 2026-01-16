# Customize fruit salad with a CLI
## Objective
In this lab, you'll work with the cli-customize-fruit-salad project to understand Rust CLI (Command Line Interface) applications and data manipulation. By the end of this lab, you'll be able to read input from either a CSV file or command-line arguments and then generate a randomized fruit salad.

## Instructions
**Step 1: Access the Repository**
Visit the following GitHub repository: [rust-data-engineering](https://github.com/nogibjj/rust-data-engineering).

**Step 2: Setup GitHub Codespaces**
Click on the green "Code" button on the GitHub repository page.
Select "Open with Codespaces."
Click on "+ New codespace."
Note: Ensure you have a GitHub account and permissions to create Codespaces.
* **Note:** using my present Codespace. Code can be found from https://github.com/rojala/test_rst_mod_/tree/main/module2/cli-customize-fruit-salad.

**Step 3: Navigate to the Project Folder**
Within the Codespace environment, navigate to the rust-data-engineering/main/cli-customize-fruit-salad/src folder using the file explorer on the left-hand side.

**Step 4: Review Code**
Open the lib.rs and main.rs files and review the existing code. The lib.rs file contains a function to shuffle a list of fruits, and main.rs uses that function to create a fruit salad based on the inputs from a CSV file or command-line arguments.

**Step 5: Build the Program**
Open the integrated terminal by going to View -> Terminal and run the following command to compile the program:

cargo build

**Step 6: Execute the Program**
Run the program using either a CSV file with a list of fruits or by providing the list directly as a command-line argument. You can run the program using one of the following commands:
``` bash
    cargo run -- --csvfile fruits.csv
    # Well valid command is
    cargo run -- fruits.csv
```
    or
``` bash
    cargo run -- --fruits "apple, pear"
```
Observe the output, which will display a shuffled list of fruits in your fruit salad.
``` bash
    cargo run -- --fruits "apple, pear"
        Your fruit salad contains:
        apple
        pear
```

``` bash
    cargo run -- fruits.csv
        Your fruit salad contains:
        morango
        uva
        laranja
        banana
        limão
        abacaxi
        melancia
        pêssego
        manga
        maçã
```

## Reflection Questions
### What is the role of the create_fruit_salad function in the lib.rs file?
The create_fruit_salad function takes a mutable vector of strings (fruit names) as input and returns a new vector with the same fruits but in random order. It uses the rand crate's shuffle method to randomly rearrange the fruits before returning them. This function essentially randomizes the order of fruits in a salad.

### How does the program read input from either a CSV file or command-line arguments?
The program reads input using the `clap` crate for command-line argument parsing:

1. **Argument Parsing**: The `Opts` struct defines two optional input sources:
   - `fruits`: A string of comma-separated fruit names (passed via `--fruits` flag)
   - `csvfile`: A path to a CSV file (positional argument)

2. **Input Processing**: In the `main()` function, a `match` statement checks which input source is available:
   - **CSV File**: If `csvfile` is provided, the program reads the file using `std::fs::read_to_string()` and parses it with `csv_to_vec()`
   - **Command-line Arguments**: If no CSV file is provided, it uses the `--fruits` argument and splits it by commas
   - **Default**: If neither is provided, it defaults to an empty string

3. **Parsing Helper**: The `csv_to_vec()` function splits the input string by commas, trims whitespace, and collects the results into a vector of strings.

4. **Usage Examples**:
   - `cargo run -- fruits.csv` - reads from CSV file
   - `cargo run -- --fruits "apple, pear"` - reads from command-line arguments

### What is the purpose of the Opts struct in main.rs?
1. **Argument Parsing**: The `Opts` struct defines two optional input sources:
   - `fruits`: A string of comma-separated fruit names (passed via `--fruits` flag)
   - `csvfile`: A path to a CSV file (positional argument)

## Challenge Questions
### How would you modify the program to allow for customizable "salad dressing," such as adding a random syrup or spice to the fruit salad?

### Can you extend the application to output the resulting fruit salad to a new CSV file?

### Can you adapt this code for different kinds of data, such as a list of books or movies?

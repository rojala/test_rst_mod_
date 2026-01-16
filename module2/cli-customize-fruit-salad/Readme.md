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

The program has been modified to support customizable dressing with a command-line argument. Here's what was implemented:

1. **Added `--dressing` flag to CLI**: The `Opts` struct now includes a `dressing: bool` field with a `--dressing` or `-d` flag.

2. **Created `get_random_dressing()` function** in lib.rs that randomly selects from 8 dressing options:
   - Honey Drizzle
   - Maple Syrup
   - Cinnamon Spice
   - Vanilla Yogurt
   - Mint Lime
   - Balsamic Reduction
   - Brown Sugar Glaze
   - Coconut Cream

3. **Updated display function**: The `display_fruit_salad()` function now accepts an optional dressing parameter and displays it below the fruit list.

4. **Usage examples**:
   ```bash
   cargo run -- --fruits "apple, pear" --dressing
   # Output:
   # Your fruit salad contains:
   # pear
   # apple
   # 
   # Dressing: Mint Lime
   ```
   
   ```bash
   cargo run -- fruits.csv --dressing
   # Displays fruit salad with randomly selected dressing
   ```

#### Testing

Comprehensive test cases have been created to ensure the program functions correctly. The test suite includes both unit tests and integration tests.

#### Running Tests

To run all tests:
```bash
cargo test
```

To run tests with output:
```bash
cargo test -- --nocapture
```

To run a specific test:
```bash
cargo test test_name
```

##### Unit Tests (src/lib.rs)

The unit tests verify the core functionality of the library functions:

1. **test_create_fruit_salad_returns_same_fruits** - Verifies the salad contains the same number of fruits as the input
2. **test_create_fruit_salad_contains_all_fruits** - Ensures all original fruits are present in the shuffled result
3. **test_create_fruit_salad_with_empty_vector** - Edge case: tests with empty fruit list
4. **test_create_fruit_salad_with_single_fruit** - Edge case: tests with only one fruit
5. **test_create_fruit_salad_with_duplicate_fruits** - Verifies duplicate fruits are preserved during shuffling
6. **test_create_fruit_salad_with_many_fruits** - Tests with 8 different fruits to ensure scalability
7. **test_get_random_dressing_returns_valid_dressing** - Validates that the returned dressing is from the predefined list
8. **test_get_random_dressing_returns_string** - Ensures dressing is a non-empty string
9. **test_write_salad_to_csv_success** - Verifies CSV file creation succeeds
10. **test_write_salad_to_csv_content** - Validates the CSV content format (comma-separated)
11. **test_write_salad_to_csv_with_special_characters** - Tests UTF-8 character support in CSV output
12. **test_write_salad_to_csv_empty** - Tests writing an empty salad to CSV

##### Integration Tests (tests/integration_tests.rs)

The integration tests verify the program works correctly as a complete system:

1. **test_integration_basic_salad** - Tests basic workflow with 3 fruits
2. **test_integration_salad_with_dressing** - Tests salad creation combined with dressing selection
3. **test_integration_multiple_dressing_calls** - Tests 10 consecutive random dressing selections
4. **test_integration_large_fruit_salad** - Tests with 10 different fruits to verify scalability
5. **test_integration_special_characters_in_fruit_names** - Tests with special characters (Portuguese fruit names like "açaí", "limão")
6. **test_integration_repeated_salad_creation** - Tests consistency across multiple salad creation calls
7. **test_integration_salad_to_csv** - Tests CSV output functionality with salad data
8. **test_integration_full_workflow_with_csv_output** - Tests complete workflow including salad creation, dressing, and CSV output

### Can you extend the application to output the resulting fruit salad to a new CSV file?

The application has been extended to support CSV output with an `--output` or `-o` command-line argument. Here's what was implemented:

1. **Added `--output` flag to CLI**: The `Opts` struct now includes an `output: Option<String>` field that accepts a filename.

2. **Created `write_salad_to_csv()` function** in lib.rs that:
   - Takes a slice of fruit strings and a filename as parameters
   - Joins the fruits with commas (CSV format)
   - Writes the content to the specified file
   - Returns a Result type for proper error handling

3. **Updated main.rs** to:
   - Call `write_salad_to_csv()` when an output file is specified
   - Display a confirmation message when the file is saved
   - Handle errors gracefully with descriptive error messages

4. **Usage examples**:
   ```bash
   cargo run -- --fruits "apple, banana, orange" --output output_salad.csv
   # Creates output_salad.csv containing: apple,banana,orange
   ```
   
   ```bash
   cargo run -- fruits.csv --dressing --output result.csv
   # Reads from fruits.csv, shuffles, and saves to result.csv
   # (Note: dressing is displayed but not saved in CSV)
   ```
   
   ```bash
   cargo run -- --fruits "mango, pineapple" -o output.csv
   # Short form: -o instead of --output
   ```

5. **Test coverage**: Added 4 new unit tests and 2 new integration tests:
   - `test_write_salad_to_csv_success` - Verifies file creation
   - `test_write_salad_to_csv_content` - Validates CSV content format
   - `test_write_salad_to_csv_with_special_characters` - Tests Unicode support
   - `test_write_salad_to_csv_empty` - Tests empty salad handling
   - `test_integration_salad_to_csv` - Full integration test
   - `test_integration_full_workflow_with_csv_output` - Complete workflow test

### Can you adapt this code for different kinds of data, such as a list of books or movies?

Yes, this code is highly adaptable and can be used for any type of data that needs to be shuffled and processed. Here's how to adapt it for books or movies:

#### **Step 1: Update Function Names (Optional but Recommended)**
Rename functions to reflect the new data type:
- `create_fruit_salad()` → `shuffle_books()` or `shuffle_movies()`
- `get_random_dressing()` → `get_random_genre()` or `get_random_rating()`
- `write_salad_to_csv()` → `write_books_to_csv()` or `write_movies_to_csv()`

#### **Step 2: Modify the Opts Struct in main.rs**
Change the field names and descriptions:
```rust
struct Opts {
    /// Books/movies as a string of comma-separated values
    #[clap(short, long)]
    items: Option<String>,
    csvfile: Option<String>,
    /// Add random metadata (genre, rating, etc.)
    #[clap(short, long)]
    metadata: bool,
    #[clap(short, long)]
    output: Option<String>,
}
```

#### **Step 3: Update the Core Logic in lib.rs**
The shuffling function remains the same - it just shuffles any vector of strings:
```rust
// This function works for any data type!
pub fn shuffle_items(mut items: Vec<String>) -> Vec<String> {
    let mut rng = thread_rng();
    items.shuffle(&mut rng);
    items
}
```

For metadata, replace dressing with relevant options:
```rust
// For books
pub fn get_random_genre() -> String {
    let genres = vec!["Fiction", "Mystery", "Sci-Fi", "Romance", "Thriller"];
    // ... implementation
}

// For movies
pub fn get_random_rating() -> String {
    let ratings = vec!["G", "PG", "PG-13", "R"];
    // ... implementation
}
```

#### **Step 4: Update Display Function**
Modify the display text:
```rust
fn display_items(items: Vec<String>, metadata: Option<String>) {
    println!("Your book collection contains:");  // or "Your movie watchlist:"
    for item in items {
        println!("{}", item);
    }
    if let Some(m) = metadata {
        println!("\nGenre: {}", m);  // or Rating, etc.
    }
}
```

#### **Step 5: Update File Format and Parsing**
The CSV functions work universally:
- Books: `The Great Gatsby,1984,To Kill a Mockingbird`
- Movies: `Inception,The Matrix,Interstellar`
- Any data: Same comma-separated format

#### **Example Use Cases**

**Books:**
```bash
cargo run -- --items "The Great Gatsby, 1984, Dune" --output books.csv
```

**Movies:**
```bash
cargo run -- movies.csv --metadata --output shuffled_movies.csv
```

**Music Playlist:**
```bash
cargo run -- --items "Song A, Song B, Song C" --output playlist.csv
```

**Key Advantages:**
- ✅ Core shuffling logic is **data-agnostic** - works with any strings
- ✅ CSV reading/writing is **universal** - no changes needed
- ✅ Test cases remain **mostly the same** - just different data
- ✅ Minimal code changes needed - just rename variables and update display messages

The architecture is flexible enough to handle any type of data that can be represented as a comma-separated list!


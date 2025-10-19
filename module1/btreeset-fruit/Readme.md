# External GitHub Lab: Generating Unique Fruits with Rust and BTreeSet

Objective: In this lab, you will use the BTreeSet data structure in Rust to generate a unique and ordered set of fruits. By the end of this lab, you should understand how to use BTreeSet and why it is beneficial for ensuring both uniqueness and order.

## Instructions:

1.Step 1: Go to the following GitHub repository:

    https://github.com/nogibjj/rust-data-engineering

2. Step 2: Click the green "Code" button and then select "Open with GitHub Codespaces". Follow the instructions to create a new Codespace.

3. Step 3: Once your Codespace is ready, navigate to the btreeset-fruit directory.

4. Step 4: Open the src/main.rs file and examine the provided code. The main() function creates a BTreeSet and fills it with a random selection of fruits.

5. Step 5: In the Codespaces terminal, compile the program by running cargo build.

6. Step 6: Run the program by using cargo run in the terminal. The program will generate a random selection of fruits and add them to a BTreeSet, which removes any duplicates and maintains the items in sorted order. The program then prints the BTreeSet of fruits for varying amounts of fruits.

## Reflection Questions:

1. How does the use of BTreeSet affect the output of the program compared to if a HashSet was used?

    Both `BTreeSet` and `HashSet` are **sets**, meaning they store **unique elements only**. However:

    - **`BTreeSet` maintains elements in sorted order** (lexicographically for strings).
    - **`HashSet` does not guarantee any order** — the output order will be arbitrary and may vary between runs.

    **Example difference in output:**

    ```rust
    // Using BTreeSet
    3: {"apple", "banana", "cherry"}

    // Using HashSet
    3: {"cherry", "banana", "apple"} // or any other order
    ```

    So, if you care about **consistent, sorted output**, `BTreeSet` is the right choice.


2. What are the benefits of using a BTreeSet over other collection types?

    | Feature            | `BTreeSet`                        | `HashSet`                        | `Vec` / `LinkedList`            |
    |--------------------|-----------------------------------|----------------------------------|---------------------------------|
    | Uniqueness         | ✅ Yes                            | ✅ Yes                           | ❌ No (duplicates allowed)      |
    | Sorted order       | ✅ Yes (in-order traversal)       | ❌ No                            | ❌ No                           |
    | Range queries      | ✅ Efficient (e.g., range search) | ❌ Not supported                 | ❌ Not efficient                |
    | Memory usage       | Moderate                          | Lower (usually)                 | Lower                          |
    | Performance        | Logarithmic insert/search         | Constant insert/search (avg)    | Linear search                  |

    **Why choose `BTreeSet`:**
    - You need **sorted output**.
    - You want to perform **range queries** (e.g., get all elements between "banana" and "fig").
    - You prefer **predictable iteration order**.


3. What would happen if you changed the BTreeSet to a different collection type, such as a Vec or LinkedList?

    By replacing `BTreeSet` with a `Vec` or `LinkedList`:

    - **Duplicates may appear** unless you manually check for them.
    - **No automatic sorting** — you'd need to sort manually if needed.
    - **Insertion and lookup become less efficient** (linear time).

    **Modified behavior:**
    ```rust
    let mut fruit_list = Vec::new();
    for fruit in shuffled_fruits {
        if !fruit_list.contains(&fruit) {
            fruit_list.push(fruit);
        }
        if fruit_list.len() >= *amount {
            break;
        }
    }
    ```
    This mimics set behavior but is less efficient and lacks automatic sorting.

## Challenge Questions:

1. Modify the program to include the capability to remove a fruit from the BTreeSet. Hint: you'll need to use command-line arguments to get input from the user.

2. Can you adjust the program to print out the list of unique fruits at the end in reverse order? Hint: look up how to iterate over a BTreeSet in reverse order in Rust.

3. How would you modify the program to keep track of how many times each fruit is generated? Hint: you might need to use a different collection type.

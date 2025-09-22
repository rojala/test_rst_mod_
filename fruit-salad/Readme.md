# External Lab: Creating a Fruit Salad with Rust Vectors in GitHub Codespaces

## Reflection Questions:
### What is a Vector in Rust and how is it different from arrays?
A Vector in Rust is a growable array type, represented by the `Vec<T>` type. Unlike arrays, which have a fixed size determined at compile time, vectors can change in size at runtime. This makes vectors more flexible for scenarios where the number of elements is not known in advance or can change over time.

### What is the use of the SliceRandom trait from the rand crate in the program?
The `SliceRandom` trait from the `rand` crate provides methods for randomly shuffling or selecting elements from slices (like arrays and vectors). In the program, it is used to shuffle the fruit vector, creating a randomized fruit salad.

### Why is enumerate() method used while printing the fruits? What functionality does it provide?
The `enumerate()` method is used to iterate over the elements of the vector while keeping track of their indices. This is useful for printing the fruits in a formatted way, as it allows the program to know when it is at the last element and print it without a trailing comma.

## Challenge Questions:

### Can you modify the program to accept fruits from the user and then add them to the fruit salad?

### The SliceRandom trait provides a method choose(&self, rng: &R) -> Option<&T>. Can you use this to select a random fruit from the salad?

### Can you create a feature in the program to add a specific number of random fruits (selected from a predefined list) to the salad?


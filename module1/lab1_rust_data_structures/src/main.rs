/* Lab1 Exploring Rust Data Structures

Objective: The objective of this lab is to introduce students to the common collections and data
structures available in the Rust programming language. By the end of the lab, you should have a
basic understanding of the variety of collections and their uses.

Instructions:

Step 1: Open a terminal in your lab environment.

Step 2: Navigate to the directory containing the project for this lab. The code for the lab is
already there in a file named main.rs.

Step 3: In the project directory, you will find a Makefile with the provided content. This will
be used to build and run your project.

Step 4: In the terminal, run the command make all to execute the program. This command will format
your code, check for any linter warnings, run any tests, and finally execute your program.

Step 5: Observe the output. You should see a list of common Rust collections printed in the console
along with links to their documentation.

See Readme.md for other than last program example.



Reflection Questions:

Why does Rust have different types of sequence collections like Vec, VecDeque, and LinkedList? What
are the different use cases for these collections?

Similarly, Rust provides HashMap and BTreeMap for maps, and HashSet and BTreeSet for sets. How do
these differ and when might you choose one over the other?

The 'Misc' section includes BinaryHeap. Where might this collection be used? Can you think of any
real-world examples?

Challenge Questions:

Look up the documentation for each of these collections. Can you identify the key methods for each
of these and understand what they do?

Can you modify the program to create an instance of each of these collections and perform some basic
operations on them (like inserting and removing elements)?

Can you create a program that takes user input to interactively manipulate these collections?
For example, the user could choose which type of collection to use, then add or remove elements
while the program prints out the state of the collection after each operation.

 */

/*
 // Original main function just printing the list of collections and their links

 fn main() {
    println!("Common Rust Collections:");

    // Sequences
    println!("\n\tSequences:");
    println!("\t\tVec: https://doc.rust-lang.org/std/vec/struct.Vec.html");
    println!("\t\tVecDeque: https://doc.rust-lang.org/std/collections/struct.VecDeque.html");
    println!("\t\tLinkedList: https://doc.rust-lang.org/std/collections/struct.LinkedList.html");

    // Maps
    println!("\n\tMaps:");
    println!("\t\tHashMap: https://doc.rust-lang.org/std/collections/struct.HashMap.html");
    println!("\t\tBTreeMap: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html");

    // Sets
    println!("\n\tSets:");
    println!("\t\tHashSet: https://doc.rust-lang.org/std/collections/struct.HashSet.html");
    println!("\t\tBTreeSet: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html");

    // Misc
    println!("\n\tMisc:");
    println!("\t\tBinaryHeap: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html");
}
*/

// create an instance of each of these collections and perform some basic operations on them (like inserting and removing elements)
// to demonstrate their usage.

// For example, you can create a Vec, add some elements to it, and then print its contents.
// Similarly, you can create a HashMap, insert some key-value pairs, and then retrieve a value using its key.
// Feel free to expand on this by adding more collections and operations as needed.
// Remember to include the necessary `use` statements at the top of your file to bring the collections into scope.
// For example:
// use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet
// , VecDeque, LinkedList, BinaryHeap};
// This will allow you to use these collections without needing to prefix them with `std::collections
// ::` each time.
// Happy coding!
// For more information on Rust's standard library collections, you can refer to the official documentation:
// https://doc.rust-lang.org/std/collections/
// https://doc.rust-lang.org/std/vec/struct.Vec.html
// https://doc.rust-lang.org/std/collections/struct.HashMap.html
// https://doc.rust-lang.org/std/collections/struct.HashSet.html
// https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
// https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
// https://doc.rust-lang.org/std/collections/struct.VecDeque.html
// https://doc.rust-lang.org/std/collections/struct.LinkedList.html
// https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
// Feel free to explore and experiment with these collections to get a better understanding of their capabilities and use cases.

fn test_vector() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("Vector: {:?}", vec);
    vec.pop();
    println!("Vector after pop: {:?}", vec);
}

fn test_vect_deque() {
    use std::collections::VecDeque;
    let mut vec_deque = VecDeque::new();
    vec_deque.push_back(1);
    vec_deque.push_back(2);
    vec_deque.push_front(0);
    println!("VecDeque: {:?}", vec_deque);
    vec_deque.pop_front();
    println!("VecDeque after pop_front: {:?}", vec_deque);
}

fn test_linked_list() {
    use std::collections::LinkedList;
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_front(0);
    println!("LinkedList: {:?}", list);
    list.pop_front();
    println!("LinkedList after pop_front: {:?}", list);
}

fn test_hash_map() {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    println!("HashMap: {:?}", map);
    if let Some(value) = map.get("one") {
        println!("Value for 'one': {}", value);
    }
    map.remove("two");
    println!("HashMap after removing 'two': {:?}", map);
}

fn test_btree_map() {
    use std::collections::BTreeMap;
    let mut btree_map = BTreeMap::new();
    btree_map.insert("one", 1);
    btree_map.insert("two", 2);
    println!("BTreeMap: {:?}", btree_map);
    if let Some(value) = btree_map.get("one") {
        println!("Value for 'one': {}", value);
    }
    btree_map.remove("two");
    println!("BTreeMap after removing 'two': {:?}", btree_map);
}

fn test_hash_set() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    println!("HashSet: {:?}", set);
    set.remove(&2);
    println!("HashSet after removing 2: {:?}", set);
}

fn test_btree_set() {
    use std::collections::BTreeSet;
    let mut btree_set = BTreeSet::new();
    btree_set.insert(1);
    btree_set.insert(2);
    btree_set.insert(3);
    println!("BTreeSet: {:?}", btree_set);
    btree_set.remove(&2);
    println!("BTreeSet after removing 2: {:?}", btree_set);
}

fn test_binary_heap() {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    heap.push(1);
    heap.push(3);
    heap.push(2);
    println!("BinaryHeap: {:?}", heap);
    if let Some(max) = heap.pop() {
        println!("Popped max from BinaryHeap: {}", max);
    }
    println!("BinaryHeap after pop: {:?}", heap);
}

fn main() {
    println!("Common Rust Collections:");

    // Sequences
    println!("\n\tSequences:");
    test_vector();
    test_vect_deque();
    test_linked_list();

    // Maps
    println!("\n\tMaps:");
    test_btree_map();
    test_hash_map();

    // Sets
    println!("\n\tSets:");
    test_hash_set();
    test_btree_set();

    // Misc
    println!("\n\tMisc:");
    test_binary_heap();
}

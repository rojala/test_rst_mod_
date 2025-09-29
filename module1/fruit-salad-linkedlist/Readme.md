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

    LinkedList<T>
    A doubly-linked list where each element points to the next and previous elements.

    Pros:
    * Efficient insertion and removal at both ends and in the middle (O(1) if you have the pointer).
    * Good for scenarios where you frequently insert/remove elements in the middle of the list.

    Cons:
    * Poor cache locality: each element is stored separately on the heap, so iteration is slower.
    * Slower random access: accessing the nth element takes O(n) time.
    * Generally less efficient than Vec or VecDeque for most use cases.

    Use case:
    * When you need frequent insertions/removals in the middle of a list and don’t care about random access speed

* In what situations might you prefer to use a LinkedList over other data structures?

    You might prefer to use a LinkedList in Rust in very specific scenarios, especially when the characteristics of linked lists align well with your performance needs. Here are the main situations where LinkedList can be a better choice than Vec or VecDeque:
    * When frequent insertions/removals in the middle of the list are needed
        
        If your algorithm involves frequent non-end insertions or deletions, and you already have a reference (or cursor) to the location, a LinkedList can do this in O(1) time. In contrast, Vec and VecDeque would require shifting elements, which is O(n).

        Example use case:
        * Implementing a LRU cache with a list of recently used items.
        * Maintaining a sorted list where elements are inserted in order.

    * When you need stable pointers to elements
        
        In a LinkedList, the memory location of each node is stable (unless removed), which means you can safely hold references to elements even as the list changes.
        
        Example use case:
        * Building a graph structure where nodes are stored in a list and edges reference them.

    * When you need bidirectional iteration

        LinkedList is doubly-linked, so you can iterate forward and backward efficiently.
        
        Example use case:
        * Implementing a text editor buffer where you move a cursor back and forth.


    ⚠️ Caveats and Considerations
    
    Despite these advantages, in most real-world Rust applications, Vec or VecDeque are preferred because:
    * They are faster due to better cache locality.
    * They are simpler to use and reason about.
    * Rust’s ownership model makes safe manipulation of linked lists more complex.

    Even the Rust documentation notes that LinkedList is rarely the optimal choice.

* Why is there a need to convert the LinkedList to a Vec and then back to LinkedList in this program?

    Rust’s LinkedList does not support built-in shuffling but Vec does.

### Challenge Questions:

* Can you modify the program to allow the user to add fruits at any position in the LinkedList after shuffling?

* The SliceRandom trait provides a method choose(&self, rng: &R) -> Option<&T>. Can you use this to select a random fruit from the salad?

* Can you adjust the program to remove a fruit from any position in the LinkedList, displaying the name of the removed fruit and the state of the list afterwards?


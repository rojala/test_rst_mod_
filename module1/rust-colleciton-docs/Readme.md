# Rust Collections Docs

https://doc.rust-lang.org/std/collections/index.html

## Reflection Questions:

* What are the main collection types provided in Rust's standard library? When might you use each one?

* How does the documentation recommend deciding between a Vec or HashMap? When would other collections be more optimal?

* According to the docs, what are some key factors to consider when choosing between HashMap and BTreeMap?

* What are some differences in performance characteristics between Vec, VecDeque, and LinkedList? When does this matter?

* How could using entry API methods like or_insert help optimize accumulators and other use cases?

## Challenges:

* Implement a simple word counter in Rust using a HashMap and the entry API.

* Build a priority queue data structure in Rust using a BinaryHeap to sort items by priority.

* Benchmark different collections in Rust on insert and lookup operations to compare performance.

* Create a small Rust command line app that parses arguments using Vec, HashMap, and BTreeSet.

* Write tests for a custom Rust struct that implements Ord and other traits to be usable in BTreeMap and BTreeSet.

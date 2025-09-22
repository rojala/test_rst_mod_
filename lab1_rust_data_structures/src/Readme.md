# Compiler output after 1st run
## Common Rust Collections:

        Sequences:
                Vec: https://doc.rust-lang.org/std/vec/struct.Vec.html
                VecDeque: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
                LinkedList: https://doc.rust-lang.org/std/collections/struct.LinkedList.html

        Maps:
                HashMap: https://doc.rust-lang.org/std/collections/struct.HashMap.html
                BTreeMap: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html

        Sets:
                HashSet: https://doc.rust-lang.org/std/collections/struct.HashSet.html
                BTreeSet: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html

        Misc:
                BinaryHeap: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html

## Rust sequence collections
 Why does Rust have different types of sequence collections like Vec, VecDeque, and LinkedList? What are the different use cases for these collections?

 Each is optimized for different use cases and performance characteristics.
 
### Vec (Vector)
#### Structure
    Contiguous growable array on the heap
#### Best for
    Fast random access (O(1) indexing)
    Efficient push/pop at the end (O(1))
#### Avoid if
    You need frequent insertions/removals at the front or middle (O(n))
#### Use cases
    Storing large datasets with frequent reads
    Stack-like behavior
    Buffering data for processing

### VecDeque (Double-ended queue)
#### Structure
    Ring buffer
#### Best for
    Fast push/pop at both ends (O(1))
    Queue-like behavior
#### Avoid if
    You need frequent insertions/removals in the middle
#### Use cases
    Implementing queues or deques
    Sliding window algorithms
    Event buffering

### LinkedList
#### Structure
    Doubly-linked list
#### Best for
    Frequent insertions/removals in the middle (O(1) if you have the node)
#### Avoid if
    You need fast indexing or cache-friendly performance (O(n) access)
#### Use cases
    When you need to splice or rearrange elements frequently
    Low-level systems programming where pointer manipulation is needed

## Rust maps
Rust provides HashMap and BTreeMap for maps, and HashSet and BTreeSet for sets. How do these differ and when might you choose one over the other?

Rust provides both HashMap/HashSet and BTreeMap/BTreeSet to give developers a choice between performance and ordering, depending on the use case. Here's a breakdown of the differences and when to use each:

### Key Differences
| Feature | HashMap / HashSet | BTreeMap / BTreeSet |
|---------|-------------------|---------------------|
|Ordering | No guaranteed order|Sorted by key (ascending)|
|Performance | Faster for most operations | Slower, especially for large sets|
|Underlying structure|Hash table|Balanced binary tree (B-tree)|
|Memory usage|Slightly more memory|More compact memory usage|
|Key requirements|Requires Hash + Eq traits*|Requires Ord trait*|
|Iteration|Arbitrary order|Ordered iteration|

*Rust uses traits to define what operations a type supports. For collections like HashMap, HashSet, BTreeMap, and BTreeSet, the key types must implement certain traits so the collection can work properly.

### Use HashMap / HashSet when:
You need fast lookups, insertions, and deletions.
You don’t care about order.

#### Examples:
Caching
Counting frequencies
Storing configuration options

### Use BTreeMap / BTreeSet when:
You need sorted keys or range queries.
You want predictable iteration order.
Your keys implement Ord.
#### Examples:

Keeping leaderboard scores sorted
Implementing interval trees or range filters
Generating ordered reports

### Summary
Choose HashMap/HashSet for speed.
Choose BTreeMap/BTreeSet for order and range queries.

## What is BinaryHeap in Rust?
BinaryHeap is a priority queue implemented as a binary max-heap. It always keeps the largest element at the top, making it efficient for scenarios where you repeatedly need to access or remove the highest-priority item.

# Documentation
Look up the documentation for each of these collections. Can you identify the key methods for each of these and understand what they do?

## Sequence Collections
### Vec<T>
#### Key Methods:
* new()
* with_capacity()
* push()
* pop()
* insert()
* remove()
* get()
* get_mut()
* len()
* capacity()
* resize()
* shrink_to_fit()
#### Use Cases:
* Dynamic arrays
* Stack-like behavior
* Fast random access
* Best for most general-purpose storage

### VecDeque<T>
#### Key Methods:
* push_back()
* push_front()
* pop_back()
* pop_front()
* get()
* get_mut()
* swap()
* make_contiguous()
* truncate()
* shrink_to_fit()
#### Use Cases:
* Double-ended queue
* Efficient front/back insertion/removal
* Ring buffer scenarios

### LinkedList<T>
#### Key Methods:
* push_back()
* push_front()
* pop_back()
* pop_front()
* append()
* split_off()
* iter()
* iter_mut()
#### Use Cases:
* When frequent insertions/removals in the middle are needed
* Rarely used due to poor cache locality

## Map Collections
### HashMap<K, V>
#### Key Methods:
* insert()
* get()
* remove()
* contains_key()
* entry()
* len()
* capacity()
* with_hasher()
* with_capacity()
#### Use Cases:
* Fast key-value lookup
* Unordered map
* Requires Hash + Eq traits

### BTreeMap<K, V>
#### Key Methods:
* insert()
* get()
* remove()
* first_key_value()
* last_key_value()
* range()
* entry()
#### Use Cases:
* Ordered map
* Range queries
* Requires Ord trait

## Set Collections
### HashSet<T>
#### Key Methods:
* insert()
* remove()
* contains()
* iter()
* drain()
* retain()
* with_hasher()
* with_capacity()
### Use Cases:
* Fast membership testing
* Unordered set
* Requires Hash + Eq traits

### BTreeSet<T>
#### Key Methods:
* insert()
* remove()
* contains()
* get()
* range()
* union()
* intersection()
* difference()
* first()
* last()
* pop_first()
* pop_last()
#### Use Cases:
* Ordered set
* Efficient range operations
* Requires Ord trait

## Misc Collection
### BinaryHeap<T>
#### Key Methods:
* push()
* pop()
* peek()
* peek_mut()
* into_sorted_vec()
* drain_sorted()
* retain()
#### Use Cases:
* Priority queue
* Always returns the largest (or smallest with Reverse) item
* Requires Ord trait

When to use a Rust Set

[Rust - Use the Set variant of any of these Maps when](https://doc.rust-lang.org/std/collections/index.html#use-the-set-variant-of-any-of-these-maps-when)

## 🧠 **Reflection Questions**

1. What are some critical differences between Vec, VecDeque, and LinkedList in Rust? When would you choose each one?
    - **Vec**:
        - Backed by a contiguous array.
        - Fast random access (`O(1)`), efficient push/pop at the end.
        - Poor performance for insertions/removals at the front or middle (`O(n)`).
        - **Use when**: You need fast indexing and mostly push/pop at the end.

    - **VecDeque**:
        - Double-ended queue implemented with a ring buffer.
        - Efficient push/pop at both ends (`O(1)`).
        - Slightly slower random access than `Vec`.
        - **Use when**: You need to add/remove from both ends efficiently.

    - **LinkedList**:
        - Doubly-linked list.
        - Efficient insertions/removals anywhere (`O(1)` if you have the node).
        - Poor cache locality and slower iteration.
        - **Use when**: You need frequent insertions/removals in the middle and don’t care about indexing.

---

2. How does the performance of HashMap and BTreeMap differ for operations like insertion and lookup, according to the docs?
    - **HashMap**:
        - Uses a hash table.
        - Average-case `O(1)` for insertions and lookups.
        - Unordered.
        - **Best for**: Fast access when order doesn’t matter.

    - **BTreeMap**:
        - Ordered map using a B-tree.
        - `O(log n)` for insertions and lookups.
        - Maintains sorted order.
        - **Best for**: When you need ordered keys or range queries.

---

3. What role do iterators play in manipulating Rust collections? What iterator methods seem most useful?
    - Iterators provide a powerful, lazy way to process collections.
    - Common methods:
        - `.map()`, `.filter()`, `.fold()`, `.collect()`, `.enumerate()`, `.zip()`
    - **Useful for**:
        - Transforming data.
        - Chaining operations.
        - Writing concise and expressive code.

---

4. When can using entry API methods like or_insert optimize accumulators and maps? What problem does it help solve?
    - `entry` lets you access or insert a value in one step.
        - Example: `map.entry(word).or_insert(0) += 1`
    - **Solves**: Avoids checking if a key exists before inserting/updating.
    - **Optimizes**: Cleaner and faster accumulator logic.

---

5. What are some ways to manage capacity and prevent unnecessary allocations with Rust collections?
    - Use `.with_capacity(n)` to preallocate space.
    - Avoids repeated reallocations during growth.
    - Methods like `.shrink_to_fit()` help reduce memory usage.
    - **Best for**: Performance-sensitive code or large datasets.

---

## 🧪 **Challenges**

1. Word frequencies
    * Run:

            cargo run --features word_frequencies
    
    * [src](src/word_frequencies.rs)
    * [Input txt](src/word_frequencies.txt)
# Rust Collections Docs

https://doc.rust-lang.org/std/collections/index.html

## Reflection Questions:

* What are the main collection types provided in Rust's standard library? When might you use each one?
* How does the documentation recommend deciding between a Vec or HashMap? When would other collections be more optimal?
* According to the docs, what are some key factors to consider when choosing between HashMap and BTreeMap?
* What are some differences in performance characteristics between Vec, VecDeque, and LinkedList? When does this matter?
* How could using entry API methods like or_insert help optimize accumulators and other use cases?

Dtailed breakdown based on the [Rust Collections documentation](https://doc.rust-lang.org/std/collections/index.html):

---

### 📦 What Are the Main Collection Types in Rust?

Rust’s standard library groups collections into four major categories:

| Category     | Types Included                                  | Use Cases |
|--------------|--------------------------------------------------|-----------|
| **Sequences** | `Vec`, `VecDeque`, `LinkedList`                 | Ordered data, stacks, queues |
| **Maps**      | `HashMap`, `BTreeMap`                           | Key-value storage |
| **Sets**      | `HashSet`, `BTreeSet`                           | Unique elements |
| **Misc**      | `BinaryHeap`                                    | Priority queues |

Each has specific strengths. For example:
- Use `Vec` for simple, ordered storage and fast appends.
- Use `HashMap` for fast key-value lookups.
- Use `BTreeMap` when you need sorted keys or range queries.

---

### 🧠 Vec vs HashMap: How to Choose?

The docs say: _“You should probably just use Vec or HashMap.”_ They’re optimized for general-purpose use and cover most needs.

- **Use `Vec`** when:
  - You need ordered elements.
  - You’re appending frequently.
  - You want a resizable array or stack.

- **Use `HashMap`** when:
  - You need fast key-based access.
  - You’re building a cache or lookup table.

Other collections are more niche but can be optimal for specific patterns:
- `VecDeque` for double-ended queues.
- `BTreeMap` for sorted maps and range queries.
- `BinaryHeap` for priority-based processing.

---

### 🔍 HashMap vs BTreeMap: Key Considerations

| Feature             | `HashMap`                        | `BTreeMap`                        |
|---------------------|----------------------------------|-----------------------------------|
| **Ordering**        | Unordered                        | Sorted by key                     |
| **Performance**     | Expected O(1) for most ops       | O(log n) for most ops             |
| **Range Queries**   | Not supported                    | Efficient range queries           |
| **Memory Use**      | Slightly more memory-efficient   | Slightly heavier due to tree nodes |

Use `BTreeMap` when:
- You need ordered keys.
- You want to iterate in sorted order.
- You need to find the smallest/largest key or do range-based lookups.

---

### ⚙️ Vec vs VecDeque vs LinkedList: Performance Differences

| Operation         | `Vec`         | `VecDeque`     | `LinkedList`   |
|------------------|---------------|----------------|----------------|
| **Access by index** | O(1)        | O(1)           | O(n)           |
| **Insert/remove at ends** | Fast at end | Fast at both ends | Fast anywhere |
| **Memory locality** | Excellent   | Good           | Poor           |
| **Use case**      | Stack, buffer | Queue, deque   | Rare—only when splitting/appending lists efficiently |

The docs caution: _“You are absolutely certain you really, truly, want a doubly linked list”_—meaning `LinkedList` is rarely the best choice.

---

### 🔑 Entry API and `or_insert`: Why It’s Powerful

The `entry` API lets you avoid redundant lookups when updating maps. Instead of checking if a key exists and then inserting, you do both in one step:

```rust
use std::collections::HashMap;

let mut count = HashMap::new();
for word in ["apple", "banana", "apple"] {
    *count.entry(word).or_insert(0) += 1;
}
```

Benefits:
- Avoids double lookup.
- Cleaner, more efficient accumulator logic.
- Works well for counters, grouping, or initializing complex values.

This is especially useful in performance-sensitive code or when building frequency maps, caches, or conditional inserts.


## Challenges:

* Implement a simple word counter in Rust using a HashMap and the entry API.

* Build a priority queue data structure in Rust using a BinaryHeap to sort items by priority.

* Benchmark different collections in Rust on insert and lookup operations to compare performance.

* Create a small Rust command line app that parses arguments using Vec, HashMap, and BTreeSet.

* Write tests for a custom Rust struct that implements Ord and other traits to be usable in BTreeMap and BTreeSet.

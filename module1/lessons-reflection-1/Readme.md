# Lesson Reflection
## Top 3 Key Points

* Vectors and HashMaps are the most common and useful Rust collection types, similar to Python lists and dicts
* Rust focuses on type safety - collections are immutable by default unlike Python
* Structuring data properly from the start avoids bugs down the line

## 5 Reflection Questions

### 1. Which Rust collection type is closest to a Python list? Why?

Rust’s `Vec<T>` (vector) is the closest analogue to Python’s built-in `list`.

Reasons:
- Both are growable, contiguous arrays.
- Both support indexing in O(1) (amortized for push at the end).
- Both allow heterogeneous types only via indirection (Python naturally via dynamic typing; Rust via `enum`, trait objects like `Vec<Box<dyn Trait>>`, or `Vec<anyhow::Result<_>>`, etc.).
- Both preserve insertion order and allow iteration in that order.

Key difference: Python lists are dynamically typed (elements can differ in type at runtime); `Vec<T>` is monomorphic (all elements must be the same concrete `T` at compile time).

---

### 2. How can you make a Rust collection mutable? Should you always use mutable collections?

To mutate (change length or elements) you need a mutable binding to the collection:

```rust
let mut v = Vec::new();
v.push(42);
```

Or if you only need to mutate through a reference:

```rust
fn add_one(v: &mut Vec<i32>) {
    v.push(1);
}
```

Key points:
- Mutability in Rust is a property of the *binding* (`let mut`) or the *borrow* (`&mut`), not of the type itself (types like `Vec<T>` are always internally capable of mutation).
- You should NOT always make collections mutable:
  - Prefer immutability (`let v = ...`) when the collection won’t change—improves reasoning, enables compiler optimizations, and prevents accidental mutation.
  - Use mutation when building or updating a collection incrementally.
  - After construction, you can allow it to become immutable by not exposing `&mut` references further (sometimes called “freeze after build” pattern).

Immutability is the default in Rust to promote safety and clarity.

---

### 3. What are some differences between a Vector and a Linked List? When would you pick one over the other?

Rust provides `Vec<T>` in the standard library and a doubly linked list as `std::collections::LinkedList<T>`. (Singly linked lists are typically hand-rolled for educational purposes.)

Comparison:

| Aspect | `Vec<T>` | `LinkedList<T>` |
|--------|----------|-----------------|
| Memory layout | Contiguous block | Each node separately allocated (pointer chasing) |
| Cache locality | Excellent | Poor |
| Random indexing | O(1) | O(n) |
| Push/pop end | Amortized O(1) | O(1) (back/front) |
| Insert/remove in middle (with iterator) | O(n) due to shifting | O(1) once you have the node |
| Overall constant factors | Low | High (alloc per node) |
| Typical real-world performance | Usually faster for most workloads | Rarely faster; niche use cases |

When to pick:
- Use `Vec<T>` almost always: best general-purpose performance.
- Consider `LinkedList<T>` only if:
  - You truly need frequent insert/remove operations in the middle with many existing stable iterators or references to nodes.
  - You are splicing lists together (its `split_off`, `append` can be O(1) operations on node chains).
  - You need a queue-like structure but still, `VecDeque` is usually better.

In practice, `LinkedList` is seldom the optimal choice; `Vec` or `VecDeque` or even a small custom arena-backed structure usually wins.

---

### 4. What Rust collection type has fast push/pop from both ends?

`std::collections::VecDeque<T>` (a growable ring buffer).

Characteristics:
- `push_front`, `push_back`, `pop_front`, `pop_back` are all amortized O(1).
- Provides queue and double-ended queue semantics efficiently.
- Internally uses a ring buffer, avoiding shifting elements like a `Vec<T>` would for front operations.

Use it for:
- FIFO queues
- Sliding windows
- Deques where you manipulate both ends

---

### 5. Why is a HashMap useful for counting frequency?

`std::collections::HashMap<K, V>` allows you to map each key (e.g., a word, character, or ID) to a running count with average O(1) insertion and lookup.

Typical pattern:

```rust
use std::collections::HashMap;

let mut freq = HashMap::new();
for word in words_iter {
    *freq.entry(word).or_insert(0) += 1;
}
```

Reasons it’s ideal:
- Fast average-case updates (`entry` API avoids double lookup).
- Only stores keys actually seen (sparse vs. pre-allocating an array).
- Flexible key types (anything hash + eq).
- Clear, idiomatic frequency-count code.

If keys are small, dense integers (e.g., bytes 0–255), a fixed-size array (`[usize; 256]`) can be faster. But for general or sparse keys, `HashMap` is the ergonomic and performant choice.

----
----


## 5 Challenge Exercises

### Create a program that stores a grocery list in a vector and prints out each item
    see grocery-list dir

### Implement a queue by using a VecDeque and simulate people waiting in line
    see waiting-line dir

### Build a simple phone book as a hash map of names to phone numbers
    see phone-book dir

### Collect word counts in a text file using a hash map
    see word-count dir

Store RGB color values in a vector or array and print them out

Runnable Rust Code Examples
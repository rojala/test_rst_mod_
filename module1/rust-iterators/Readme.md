Rust iterators

[Rust doc iterators](https://doc.rust-lang.org/std/collections/index.html#iterators)

## 🧠 Reflection Questions

### 1. What are some of the key benefits of using iterators in Rust? (lazily evaluated, efficient, safe, convenient)? 
- **Lazily evaluated**: Iterators don’t compute values until needed, which saves computation.
- **Efficient**: They avoid intermediate allocations and can be fused into a single loop.
- **Safe**: Rust’s ownership and borrowing rules ensure memory safety during iteration.
- **Convenient**: Rich set of adapter methods (`map`, `filter`, `fold`, etc.) makes data processing expressive and concise.

---

### 2. How can iterators help prevent unnecessary allocations? (only values that are used are produced)?
- Iterators **produce values only when consumed**, avoiding temporary collections.
- Example:
  ```rust
  let doubled: Vec<_> = (1..10).map(|x| x * 2).collect();
  ```
  - Here, no intermediate vector is created during mapping — only the final result is allocated.

---

### 3. What are the three primary iterators that collections should provide? (iter, iter_mut, into_iter)?
- **`iter()`**: Immutable references (`&T`)
- **`iter_mut()`**: Mutable references (`&mut T`)
- **`into_iter()`**: Ownership of values (`T`)

These allow flexible access patterns depending on whether you want to read, modify, or consume the collection.

---

### 4. When would you want to use iter vs iter_mut vs into_iter? (iter for immutable access, iter_mut for mutable access, into_iter for ownership transfer)?

| Method       | Use Case                          | Ownership |
|--------------|-----------------------------------|-----------|
| `iter()`     | Read-only access                  | Borrowed  |
| `iter_mut()` | Modify elements in-place          | Borrowed (mutable) |
| `into_iter()`| Move values out of the collection | Owned     |

Example:
```rust
let mut vec = vec![1, 2, 3];

// Read-only
for x in vec.iter() {
    println!("Read: {}", x);
}

// Modify
for x in vec.iter_mut() {
    *x *= 2;
}

// Consume
for x in vec.into_iter() {
    println!("Owned: {}", x);
}
```

---

### 5. What are some examples of adapter methods provided by iterators? (map, fold, skip, take, rev)?
These methods transform or operate on iterators:

- **`map`**: Transform each item.
- **`filter`**: Keep items that match a condition.
- **`fold`**: Accumulate values (like reduce).
- **`skip` / `take`**: Limit the range of iteration.
- **`rev`**: Reverse the order of iteration.

Example:
```rust
let sum: i32 = (1..10).filter(|x| x % 2 == 0).map(|x| x * x).sum();
```

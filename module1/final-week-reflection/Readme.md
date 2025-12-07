# Final Week-Reflection

## Week 1 Reflection Questions:
1. What was most useful about Rust's data structure capabilities for data engineering tasks? What capabilities stood out as particularly valuable?
- **Memory safety without garbage collection**: Rust’s ownership and borrowing rules ensure that data structures are safe to use in concurrent or high-throughput pipelines without runtime overhead.
- **Predictable performance**: Collections like `Vec`, `HashMap`, and `BTreeMap` are implemented with tight control over allocation and iteration, which is critical for large-scale data processing.
- **Expressive APIs**: Iterators, combinators (`map`, `filter`, `fold`), and pattern matching make it easy to transform and aggregate data in a functional style while staying efficient.
- **Concurrency readiness**: Many structures integrate well with multi-threaded contexts (`Arc`, `Mutex`, `RwLock`), which is valuable for parallel data engineering tasks.

2. How did working with sequences like vectors and linked lists expand your skills in handling data in Rust? What use cases are best suited for each sequence type?
When starting from zero knowledge of Rust's data structures, working with sequences like vectors and linked lists provided a foundational understanding of how to manage collections of data efficiently. Each sequence type has its own strengths and ideal use cases, which helped me appreciate the importance of choosing the right data structure for specific tasks.
- **Vectors (`Vec<T>`)**
  - Best for: bulk storage, random access, and cache-friendly iteration.
  - Use cases: reading large datasets into memory, batch transformations, numerical computations.
  - Expansion of skills: learning how to manage capacity, amortized growth, and slice-based operations.
- **Linked Lists (`LinkedList<T>`)**
  - Best for: frequent insertions/removals in the middle of a sequence.
  - Use cases: streaming pipelines where elements are consumed and replaced, or when order matters but random access is rare.
  - Expansion of skills: understanding trade-offs between pointer-heavy structures and contiguous memory, and when locality of reference matters.

3. What insights did you gain from using HashMaps and BTreeMaps for tasks like counting word frequencies and comparing languages? When would you choose one over the other?
- **HashMap**
  - Strengths: average-case O(1) lookups, great for tasks like word frequency counting.
  - Use cases: language comparison by token frequency, fast membership checks.
- **BTreeMap**
  - Strengths: ordered keys, efficient range queries, predictable iteration order.
  - Use cases: comparing languages by sorted word lists, prefix-based queries, or when deterministic ordering is required.
- **Insight gained**: choosing between them depends on whether *speed* or *ordering* is more critical. For example, a word frequency counter benefits from `HashMap`, while a lexicographic comparison of languages benefits from `BTreeMap`.

4. How did HashSet and BTreeSet help you manage data differently than the sequences? What are some key differences in using sets versus sequences or maps?
- **HashSet / BTreeSet**
  - Key difference: uniqueness is enforced automatically, unlike sequences which allow duplicates.
  - HashSet: fast membership checks, useful for deduplication (e.g., unique words in a corpus).
  - BTreeSet: ordered uniqueness, useful for sorted unique collections (e.g., alphabetically sorted vocabulary).
- **Compared to sequences**: sets don’t preserve insertion order or allow duplicates, so they’re better for tasks where uniqueness and membership are the focus.
- **Compared to maps**: sets store only keys, not key-value pairs, making them lighter when values aren’t needed.

5. How did implementing algorithms like page rank further expand your Rust data structure skills? What new capabilities did you learn?
- **Expansion of skills**:
  - Learned to combine multiple structures (graphs via adjacency lists in `HashMap<Vec<T>>`, ranking scores in `HashMap<Node, f64>`).
  - Practiced iterative refinement using Rust’s iterator chains and ownership rules.
  - Gained experience with floating-point precision, convergence checks, and efficient graph traversal.
- **New capabilities**:
  - Building custom graph abstractions with modular code.
  - Leveraging Rust’s borrow checker to prevent aliasing bugs in recursive or iterative algorithms.
  - Understanding how to integrate algorithmic logic with Rust’s standard collections for scalable computation.

## Week 1 Challenges:

1. Implement a program that reads data from a CSV file into a vector and calculates simple summary statistics like min, max, and mean on one column.
[See csv/src/main.rs](csv/src/main.rs) for implementation.

2. Build a hash map that counts the frequency of words in a large text file. Handle capitalization and punctuation to combine different forms of the same word.
[See hash-map/src/main.rs](hash-map/src/main.rs) for implementation.

3. Create a linked list of the most common words in a text file. Then, sort the list alphabetically and print the top 10 most common words.
[See linked-list/src/main.rs](linked-list/src/main.rs) for implementation.

4. Develop a function determining if a graph defined using Rust's graph data structures is fully connected.
[See graph/src/main.rs](graph/src/main.rs) for implementation.

5. Implement Kosaraju's algorithm for detecting strongly connected components in a directed graph.
[See kosaraju/src/main.rs](kosaraju/src/main.rs) for implementation.

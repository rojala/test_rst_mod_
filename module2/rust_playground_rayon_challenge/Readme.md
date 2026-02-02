# Rust Playground Rayon Challenge

## Lab Goal

Apply concepts from the concurrency lesson to parallelize an operation using Rayon and benchmark performance.

## Lab Description

In this lab you will parallelize a simple vector sum calculation using Rayon and compare the performance to sequential processing. By spawning multiple threads, rayon will divide the work and utilize more CPU cores to speed up the job. Proper benchmarks are key to justifying complexity so you will add timers before and after the parallel operation.

## Lab Steps

Run the existing simple sum program:

```rust
/* Extend this example */
use rayon::prelude::*;

fn main() {
    let data = vec![1, 2, 3];

    let parallel_sum: i32 = data.par_iter() // Specify the type
        .map(|x| x * x)
        .sum();

    println!("Parallel sum: {}", parallel_sum);
}
```

### 1. Add a timer before and after the sum using Instant

### 2. Parallelize the map/sum using Rayon's par_iter

### 3. Compare duration of the sequential and parallel sums

## Reflection Questions

### How much faster was the parallelized version?

### What hyperparameter could you tweak to influence performance?

### What tradeoffs exist when parallelizing algorithms?

### In what scenario would concurrency not improve performance?

### How does Rust help mitigate common pitfalls like race conditions?

## Challenge Exploration

### Try different vector sizes - what impact on speedup?

### Explore num_threads() setting - what is the impact?

### Simulate random I/O wait with sleep() inside the map

### Could parallel slowdown ever happen? Explore.

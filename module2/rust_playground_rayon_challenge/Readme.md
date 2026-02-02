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
``` bash
Sequential duration: 872ns
Parallel duration: 221.831µs
```

## Reflection Questions

### How much faster was the parallelized version?
Parallel is much slower here because the input size is tiny (3 elements). Rayon's overhead (thread pool scheduling, task splitting, synchronization) dominates, so sequential wins by orders of magnitude. For larger datasets or heavier per-item work, parallel can outperform once overhead is amortized.

### What hyperparameter could you tweak to influence performance?
Rayon thread pool size (num_threads) is the key hyperparameter; increasing or decreasing worker threads directly affects performance.

### What tradeoffs exist when parallelizing algorithms?
Parallelization trades higher overhead and complexity (thread setup, synchronization, contention, nondeterminism) for potential speedup; benefits appear only when the workload is large enough and the algorithm scales well.

### In what scenario would concurrency not improve performance?
Concurrency won’t help for tiny workloads, highly sequential dependencies, or when the task is dominated by overhead, shared resource contention, or single-core I/O bottlenecks.

### How does Rust help mitigate common pitfalls like race conditions?
Rust prevents data races at compile time via ownership, borrowing, and trait bounds (Send/Sync), so shared mutable access must be synchronized (e.g., Mutex, RwLock) before it compiles.

## Challenge Exploration

### Try different vector sizes - what impact on speedup?Size: 3
With codespace, I observed the following results with sizes ranging from 3 to 1,000,000,000:

```bash
Size: 3
Sequential sum: 14
Sequential duration: 943ns
Parallel sum: 14
Parallel duration: 236.341µs
Speedup (seq/par): 0.004x

Size: 1000
Sequential sum: 333833500
Sequential duration: 18.905µs
Parallel sum: 333833500
Parallel duration: 90.513µs
Speedup (seq/par): 0.209x

Size: 100000
Sequential sum: 333338333350000
Sequential duration: 978.226µs
Parallel sum: 333338333350000
Parallel duration: 1.075528ms
Speedup (seq/par): 0.910x

Size: 1000000
Sequential sum: 333333833333500000
Sequential duration: 18.924555ms
Parallel sum: 333333833333500000
Parallel duration: 14.323569ms
Speedup (seq/par): 1.321x
```
Speedup comes after large numbers of elements (1 million+) where parallel overhead is amortized. But that depends also what is going on on PC.

### Explore num_threads() setting - what is the impact?
num_threads() controls the size of Rayon's worker pool. More threads can increase throughput on CPU‑bound work up to core count, but too many can hurt from overhead, contention, and context switching; too few underutilizes cores. The optimal value depends on workload and hardware.

Using the same code, I experimented with num_threads set to 1 and 2, and observed the following results (codespace environment):

```bash
Size: 3
Sequential sum: 14
Sequential duration: 697ns
Parallel sum (threads=1): 14
Parallel duration (threads=1): 6.471µs
Speedup (seq/par, threads=1): 0.108x
Parallel sum (threads=2): 14
Parallel duration (threads=2): 6.65µs
Speedup (seq/par, threads=2): 0.105x

Size: 1000
Sequential sum: 333833500
Sequential duration: 26.504µs
Parallel sum (threads=1): 333833500
Parallel duration (threads=1): 16.962µs
Speedup (seq/par, threads=1): 1.563x
Parallel sum (threads=2): 333833500
Parallel duration (threads=2): 56.146µs
Speedup (seq/par, threads=2): 0.472x

Size: 100000
Sequential sum: 333338333350000
Sequential duration: 1.47416ms
Parallel sum (threads=1): 333338333350000
Parallel duration (threads=1): 2.129374ms
Speedup (seq/par, threads=1): 0.692x
Parallel sum (threads=2): 333338333350000
Parallel duration (threads=2): 2.597887ms
Speedup (seq/par, threads=2): 0.567x

Size: 1000000
Sequential sum: 333333833333500000
Sequential duration: 15.382758ms
Parallel sum (threads=1): 333333833333500000
Parallel duration (threads=1): 14.653973ms
Speedup (seq/par, threads=1): 1.050x
Parallel sum (threads=2): 333333833333500000
Parallel duration (threads=2): 16.841061ms
Speedup (seq/par, threads=2): 0.913x

Size: 1000000000
Sequential sum: 333333333833333333500000000
Sequential duration: 14.655008425s
Parallel sum (threads=1): 333333333833333333500000000
Parallel duration (threads=1): 13.743936924s
Speedup (seq/par, threads=1): 1.066x
Parallel sum (threads=2): 333333333833333333500000000
Parallel duration (threads=2): 16.416692656s
Speedup (seq/par, threads=2): 0.893x
```

### Simulate random I/O wait with sleep() inside the map
```bash
cargo run -- --random-delay 5
cargo run -- --random-delay 5 --max-size 100000
```

### Could parallel slowdown ever happen? Explore.
Parallel slowdown happens when overhead and contention outweigh gains—e.g., tiny workloads, low per‑item compute, memory‑bandwidth bound loops, heavy synchronization, or too many threads. Your results already show it for small sizes and some thread counts.

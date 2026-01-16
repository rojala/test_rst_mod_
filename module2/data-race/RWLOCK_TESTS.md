# RwLock Implementation and Test Results

## Overview
Created a comprehensive RwLock implementation with test cases demonstrating the advantages and disadvantages compared to Mutex.

## Files Added
- `src/rwlock_example.rs` - RwLock implementation with 5 test cases and a demonstration example

## Test Results
All 5 tests passed successfully:

```
running 5 tests
test tests::test_rwlock_multiple_readers ... ok
test tests::test_rwlock_exclusive_write ... ok
test tests::test_rwlock_vs_mutex_behavior ... ok
test tests::test_rwlock_writer_modifications ... ok
test tests::test_rwlock_poison_on_panic ... ok

test result: ok. 5 passed; 0 failed
```

## Test Cases Explained

### 1. test_rwlock_multiple_readers
- **Purpose**: Demonstrates that multiple threads can acquire read locks simultaneously
- **Key Learning**: RwLock allows concurrent readers, unlike Mutex which blocks all access

### 2. test_rwlock_exclusive_write
- **Purpose**: Shows that write access is exclusive and blocks readers
- **Key Learning**: Only one writer can hold the lock at a time; readers must wait

### 3. test_rwlock_writer_modifications
- **Purpose**: Verifies that concurrent write operations modify data correctly
- **Key Learning**: Each writer can safely modify the data without interference

### 4. test_rwlock_vs_mutex_behavior
- **Purpose**: Compares RwLock behavior with Mutex expectations
- **Key Learning**: RwLock provides equivalent thread safety with better read concurrency

### 5. test_rwlock_poison_on_panic
- **Purpose**: Demonstrates lock poisoning when a thread panics while holding a lock
- **Key Learning**: RwLock (like Mutex) poisons the lock on panic to prevent use of potentially corrupted data

## Example Output
```
=== RwLock Example ===

Writer 0 acquired write lock
Writer 0 modified data[0] to 11
Writer 1 acquired write lock
Writer 1 modified data[1] to 12
Writer 2 acquired write lock
Writer 2 modified data[2] to 13
Reader 0 acquired read lock, data: [11, 12, 13]
Reader 2 acquired read lock, data: [11, 12, 13]
Reader 1 acquired read lock, data: [11, 12, 13]

Final data: [11, 12, 13]
```

Note: Multiple readers can hold the lock simultaneously (Readers 0, 1, 2 all acquire read lock), while writers have exclusive access.

## Comparison: Mutex vs RwLock

| Feature | Mutex | RwLock |
|---------|-------|--------|
| Multiple concurrent readers | ❌ No | ✅ Yes |
| Exclusive write access | ✅ Yes | ✅ Yes |
| Overhead | Low | Higher |
| Best for | Write-heavy workloads | Read-heavy workloads |
| Complexity | Simple | More complex |
| Writer starvation risk | ❌ No | ⚠️ Possible |

## When to Use RwLock

Use `RwLock` when:
- Your application has many more read operations than write operations
- You want to maximize concurrent read access
- Lock contention from readers is a performance bottleneck

Use `Mutex` when:
- Your read/write ratio is relatively balanced
- Simplicity is more important than concurrency
- You have write-heavy workloads

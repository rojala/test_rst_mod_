// Benchmark different collections in Rust on insert and lookup operations to compare performance.
use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};
use std::time::Instant;
use rand::seq::SliceRandom;
use rand::thread_rng;
pub fn benchmark_collections() {
    let mut hash_map = HashMap::new();
    let mut btree_map = BTreeMap::new();
    let mut vec_deque = VecDeque::new();
    let mut linked_list = LinkedList::new();

    // Insert
    let start = Instant::now();
    for i in 0..1000 {
        hash_map.insert(i, i);
    }
    let duration = start.elapsed();
    println!("HashMap insert duration: {:?}", duration);

    let start = Instant::now();
    for i in 0..1000 {
        btree_map.insert(i, i);
    }
    let duration = start.elapsed();
    println!("BTreeMap insert duration: {:?}", duration);

    let start = Instant::now();
    for i in 0..1000 {
        vec_deque.push_back(i);
    }
    let duration = start.elapsed();
    println!("VecDeque insert duration: {:?}", duration);

    let start = Instant::now();
    for i in 0..1000 {
        linked_list.push_back(i);
    }
    let duration = start.elapsed();
    println!("LinkedList insert duration: {:?}", duration);

    // Lookup
    let start = Instant::now();
    for i in 0..1000 {
        hash_map.get(&i);
    }
    let duration = start.elapsed();
    println!("HashMap lookup duration: {:?}", duration);

    let start = Instant::now();
    for i in 0..1000 {
        btree_map.get(&i);
    }
    let duration = start.elapsed();
    println!("BTreeMap lookup duration: {:?}", duration);

    let start = Instant::now();
    for i in 0..1000 {
        vec_deque.get(i);
    }
    let duration = start.elapsed();
    println!("VecDeque lookup duration: {:?}", duration);

    let start = Instant::now();
    for i in 0..1000 {
        linked_list.iter().nth(i);
    }
    let duration = start.elapsed();
    println!("LinkedList lookup duration: {:?}", duration);

    // Random delete/drop
    let mut rng = thread_rng();
    let mut indices: Vec<usize> = (0..1000).collect();
    indices.shuffle(&mut rng);

    // HashMap
    let start = Instant::now();
    for &i in &indices {
        hash_map.remove(&i);
    }
    let duration = start.elapsed();
    println!("HashMap random delete duration: {:?}", duration);

    // BTreeMap
    let start = Instant::now();
    for &i in &indices {
        btree_map.remove(&i);
    }
    let duration = start.elapsed();
    println!("BTreeMap random delete duration: {:?}", duration);

    // VecDeque
    let start = Instant::now();
    for &i in &indices {
        if i < vec_deque.len() {
            vec_deque.remove(i);
        }
    }
    let duration = start.elapsed();
    println!("VecDeque random delete duration: {:?}", duration);

    // LinkedList (removing by value, since no index-based removal)
    let start = Instant::now();
    for &i in &indices {
        // Remove all occurrences of i from the linked list
        let mut new_list = LinkedList::new();
        for &x in linked_list.iter() {
            if x != i {
                new_list.push_back(x);
            }
        }
        linked_list = new_list;
    }
    let duration = start.elapsed();
    println!("LinkedList random delete duration: {:?}", duration);
}

// Test code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_collections() {
        benchmark_collections();
    }
}

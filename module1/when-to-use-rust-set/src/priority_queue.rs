use std::collections::BinaryHeap;
use rand::Rng;

pub fn run() {
    // Generate 10,000 random integers
    let mut rng = rand::thread_rng();
    let data: Vec<u64> = (0..10_000).map(|_| rng.gen_range(0..100_000_000_000)).collect();

    // Create a max-heap (BinaryHeap)
    let mut heap = BinaryHeap::from(data.clone());

    // Perform heap sort: pop elements from heap into a sorted vector
    let mut sorted = Vec::with_capacity(data.len());
    while let Some(val) = heap.pop() {
        sorted.push(val);
    }

    // Print first 20 and last 20 sorted values
    println!("First 20 (highest values): {:?}", &sorted[..20]);
    println!("Last 20 (lowest values): {:?}", &sorted[sorted.len() - 20..]);
}

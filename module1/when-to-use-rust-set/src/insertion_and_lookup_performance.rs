use std::collections::{LinkedList, HashMap, BTreeMap};
use std::time::Instant;
use rand::seq::SliceRandom;

pub fn run() {
    let size = 10_000;
    let lookup_size = 1_000;
    let keys: Vec<_> = (0..size).collect();
    let lookup_keys: Vec<_> = keys.choose_multiple(&mut rand::thread_rng(), lookup_size).cloned().collect();

    // Vec
    let mut vec = Vec::new();
    let start = Instant::now();
    for &key in &keys {
        vec.push((key, key));
    }
    let insert_vec = start.elapsed();

    let start = Instant::now();
    for &key in &lookup_keys {
        vec.iter().find(|&&(k, _)| k == key);
    }
    let lookup_vec = start.elapsed();

    // LinkedList
    let mut list = LinkedList::new();
    let start = Instant::now();
    for &key in &keys {
        list.push_back((key, key));
    }
    let insert_list = start.elapsed();

    let start = Instant::now();
    for &key in &lookup_keys {
        list.iter().find(|&&(k, _)| k == key);
    }
    let lookup_list = start.elapsed();

    // HashMap
    let mut hashmap = HashMap::new();
    let start = Instant::now();
    for &key in &keys {
        hashmap.insert(key, key);
    }
    let insert_hashmap = start.elapsed();

    let start = Instant::now();
    for &key in &lookup_keys {
        hashmap.get(&key);
    }
    let lookup_hashmap = start.elapsed();

    // BTreeMap
    let mut btreemap = BTreeMap::new();
    let start = Instant::now();
    for &key in &keys {
        btreemap.insert(key, key);
    }
    let insert_btreemap = start.elapsed();

    let start = Instant::now();
    for &key in &lookup_keys {
        btreemap.get(&key);
    }
    let lookup_btreemap = start.elapsed();

    // Results
    println!("Vec: insert = {:?}, lookup = {:?}", insert_vec, lookup_vec);
    println!("LinkedList: insert = {:?}, lookup = {:?}", insert_list, lookup_list);
    println!("HashMap: insert = {:?}, lookup = {:?}", insert_hashmap, lookup_hashmap);
    println!("BTreeMap: insert = {:?}, lookup = {:?}", insert_btreemap, lookup_btreemap);
}

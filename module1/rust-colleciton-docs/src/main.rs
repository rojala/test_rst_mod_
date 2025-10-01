use std::collections::{HashMap, BTreeSet};
use clap::{Arg, Command};

mod lib_benchmark;
mod lib_priorityqueue;
mod lib_wordcount;

fn main_with_clap() {
    let matches = Command::new("rust-collection-docs")
        .version("1.0")
        .author("Author Name")
        .about("Parses command line arguments using Vec, HashMap, and BTreeSet")
        .arg(
            Arg::new("args")
                .help("Arguments to parse")
                .required(false)
                .num_args(0..)
        )
        .get_matches();

    let args: Vec<String> = matches
        .get_many::<String>("args")
        .map(|vals| vals.cloned().collect())
        .unwrap_or_else(Vec::new);

    parse_args(args);
}

fn parse_args(args: Vec<String>) {
    // Store argument counts in a HashMap
    let mut arg_counts = HashMap::new();
    // Store unique arguments in a BTreeSet
    let mut unique_args = BTreeSet::new();

    for arg in &args {
        *arg_counts.entry(arg.clone()).or_insert(0) += 1;
        unique_args.insert(arg.clone());
    }

    println!("Arguments as Vec: {:?}", args);
    println!("Argument counts (HashMap): {:?}", arg_counts);
    println!("Unique arguments (BTreeSet): {:?}", unique_args);
}

fn test_parse_args() {
    // Simulate command line arguments
    let args = vec![
        "foo".to_string(),
        "bar".to_string(),
        "foo".to_string(),
        "baz".to_string(),
    ];
    parse_args(args);
}

fn test_word_count() {
    let text = "Hello world hello";
    let result = lib_wordcount::word_count(text);
    println!("{:?}", result);
    assert_eq!(result.len(), 2);
    assert_eq!(result.get("hello"), Some(&2));
    assert_eq!(result.get("world"), Some(&1));
}

fn test_priority_queue() {
    let mut pq = lib_priorityqueue::PriorityQueue::new();
    pq.push(3);
    pq.push(5);
    pq.push(1);

    println!("Priority Queue length: {}", pq.len());
    println!("Priority Queue contents: {:?}", pq);

    assert_eq!(pq.len(), 3);

    assert_eq!(pq.peek(), Some(&5));
    assert_eq!(pq.pop(), Some(5));
    assert_eq!(pq.pop(), Some(3));
    assert_eq!(pq.pop(), Some(1));
    assert!(pq.is_empty());
}

fn test_benchmark_collections() {
    lib_benchmark::benchmark_collections();
}

fn main() {
    test_word_count();
    test_priority_queue();
    test_benchmark_collections();
    test_parse_args();
    main_with_clap();
}

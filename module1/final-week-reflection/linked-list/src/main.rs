use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Node {
    word: String,
    count: usize,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(word: String, count: usize) -> Self {
        Node {
            word,
            count,
            next: None,
        }
    }
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn insert(&mut self, word: String, count: usize) {
        let mut new_node = Box::new(Node::new(word, count));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    fn to_vec(&self) -> Vec<(String, usize)> {
        let mut result = Vec::new();
        let mut current = &self.head;
        
        while let Some(node) = current {
            result.push((node.word.clone(), node.count));
            current = &node.next;
        }
        
        result
    }
}

fn main() -> io::Result<()> {
    // Read and count words
    let file = File::open("large_text_file.txt")?;
    let reader = io::BufReader::new(file);
    let mut word_counts: HashMap<String, usize> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        for word in line.split_whitespace() {
            let word = word.to_lowercase()
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_string();
            if !word.is_empty() {
                *word_counts.entry(word).or_insert(0) += 1;
            }
        }
    }

    // Create linked list
    let mut list = LinkedList::new();
    for (word, count) in word_counts {
        list.insert(word, count);
    }
    // 1) Sort by occurrences (descending)
    let mut words = list.to_vec();
    words.sort_by(|a, b| b.1.cmp(&a.1));

    // 2) Take the 10 most occurred
    let mut top_10: Vec<_> = words.into_iter().take(10).collect();

    // 3) Sort those 10 alphabetically
    top_10.sort_by(|a, b| a.0.cmp(&b.0));

    // Print short list with occurrences
    println!("Top 10 most common words (alphabetically sorted):");
    for (i, (word, count)) in top_10.iter().enumerate() {
        println!("{}. {} - {} occurrences", i + 1, word, count);
    }

    Ok(())
}
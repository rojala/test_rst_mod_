//  Build a hash map that counts the frequency of words in a large text file. Handle capitalization and punctuation to combine different forms of the same word.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use regex::Regex;

fn main() {
    let path = "large_text_file.txt"; // Replace with your file path
    let word_count = count_word_frequencies(path);
    
    // Convert to vector and sort by frequency (descending)
    let mut word_vec: Vec<_> = word_count.iter().collect();
    word_vec.sort_by(|a, b| b.1.cmp(a.1));
    
    // Limit output to 50 words
    for (word, count) in word_vec.iter().take(50) {
        println!("{}: {}", word, count);
    }
}

fn count_word_frequencies<P: AsRef<Path>>(filename: P) -> HashMap<String, usize> {
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut word_count = HashMap::new();
    let re = Regex::new(r"\w+").unwrap();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        for word in re.find_iter(&line) {
            let word = word.as_str().to_lowercase();
            *word_count.entry(word).or_insert(0) += 1;
        }
    }

    word_count
}


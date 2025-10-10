use std::collections::HashMap;
use std::fs;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Read the entire file into a string
    let contents = fs::read_to_string("word_frequencies.txt")?;

    // Create a HashMap to store word counts
    let mut word_counts: HashMap<String, usize> = HashMap::new();

    // Iterate over words, normalize and count
    for word in contents.split_whitespace() {
        let word = word
            .to_lowercase()
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_string();

        if !word.is_empty() {
            *word_counts.entry(word).or_insert(0) += 1;
        }
    }

    // Print the word frequencies
    for (word, count) in &word_counts {
        println!("{word}: {count}");
    }

    Ok(())
}
// Collect word counts in a text file using a hash map
use std::collections::HashMap;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Read the contents of the file into a string
    let filename = "sample.txt"; // Change this to your file path
    let contents = fs::read_to_string(filename)?;

    // Create a HashMap to store word counts
    let mut word_count: HashMap<String, usize> = HashMap::new();

    // Split the contents into words and count them
    for word in contents.split_whitespace() {
        let word = word.to_lowercase(); // Normalize to lowercase
        *word_count.entry(word).or_insert(0) += 1;
    }

    // Print the word counts
    for (word, count) in &word_count {
        println!("{}: {}", word, count);
    }

    Ok(())
}

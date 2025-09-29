use std::collections::HashMap;

// Implement a simple word counter in Rust using a HashMap and the entry API.

/// Counts the occurrences of each word in the given text.
/// Words are split by whitespace.
pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        let text = "Hello world hello";
        let result = word_count(text);
        assert_eq!(result.get("hello"), Some(&2));
        assert_eq!(result.get("world"), Some(&1));
    }
}

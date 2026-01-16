use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(name = "Fruit Vector Lab")]
#[command(about = "Remove a specific fruit from a vector", long_about = None)]
struct Args {
    /// The fruit name to remove from the vector
    #[arg(short, long)]
    fruit: String,
    
    /// Sort fruits alphabetically
    #[arg(short, long)]
    sort: bool,
    
    /// Count occurrences of each fruit
    #[arg(short, long)]
    count: bool,
}

/// Function to remove a specific fruit from a vector
fn remove_fruit(vector: &mut Vec<&str>, fruit_name: &str) -> bool {
    if let Some(pos) = vector.iter().position(|&f| f == fruit_name) {
        vector.remove(pos);
        true
    } else {
        false
    }
}

/// Function to sort fruits alphabetically
fn sort_fruits(vector: &mut Vec<&str>) {
    vector.sort();
}

/// Function to count occurrences of each fruit in a vector
fn count_fruits<'a>(vector: &'a Vec<&'a str>) -> HashMap<&'a str, usize> {
    let mut counts = HashMap::new();
    for &fruit in vector {
        *counts.entry(fruit).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let args = Args::parse();

    // Create a mutable vector of fruits.
    let mut fruit_salad = vec!["apple", "banana", "cherry", "dates", "elderberries"];
    println!("Original fruit salad: {:?}", fruit_salad);

    fruit_salad.push("figs");
    fruit_salad.push("mansikka");
    fruit_salad.push("mustikka");

    // Remove the specified fruit from the Vector using the remove_fruit function.
    if remove_fruit(&mut fruit_salad, &args.fruit) {
        println!("Successfully removed '{}' from the fruit salad", args.fruit);
    } else {
        println!("'{}' not found in the fruit salad", args.fruit);
    }

    println!("\nModified fruit salad: {:?}", fruit_salad);
    
    // Sort fruits alphabetically if --sort flag is provided
    if args.sort {
        sort_fruits(&mut fruit_salad);
        println!("\nSorted fruit salad: {:?}", fruit_salad);
    }
    
    // Count occurrences if --count flag is provided
    if args.count {
        let counts = count_fruits(&fruit_salad);
        println!("\nFruit occurrences:");
        let mut sorted_fruits: Vec<_> = counts.iter().collect();
        sorted_fruits.sort_by_key(|a| a.0);
        for (fruit, count) in sorted_fruits {
            println!("  {}: {}", fruit, count);
        }
    }
    
    // Iterate through the Vector and print each fruit.
    println!("\nFruits in the salad:");
    for fruit in &fruit_salad {
        println!("  - {}", fruit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_existing_fruit() {
        let mut fruits = vec!["apple", "banana", "cherry"];
        let result = remove_fruit(&mut fruits, "banana");
        
        assert!(result);
        assert_eq!(fruits, vec!["apple", "cherry"]);
    }

    #[test]
    fn test_remove_first_occurrence_of_duplicate() {
        let mut fruits = vec!["apple", "banana", "banana", "cherry"];
        let result = remove_fruit(&mut fruits, "banana");
        
        assert!(result);
        assert_eq!(fruits, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_remove_nonexistent_fruit() {
        let mut fruits = vec!["apple", "banana", "cherry"];
        let result = remove_fruit(&mut fruits, "orange");
        
        assert!(!result);
        assert_eq!(fruits, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_remove_from_empty_vector() {
        let mut fruits: Vec<&str> = vec![];
        let result = remove_fruit(&mut fruits, "apple");
        
        assert!(!result);
        assert_eq!(fruits, vec![] as Vec<&str>);
    }

    #[test]
    fn test_remove_from_single_element_vector() {
        let mut fruits = vec!["apple"];
        let result = remove_fruit(&mut fruits, "apple");
        
        assert!(result);
        assert_eq!(fruits, vec![] as Vec<&str>);
    }

    #[test]
    fn test_remove_first_fruit() {
        let mut fruits = vec!["apple", "banana", "cherry"];
        let result = remove_fruit(&mut fruits, "apple");
        
        assert!(result);
        assert_eq!(fruits, vec!["banana", "cherry"]);
    }

    #[test]
    fn test_remove_last_fruit() {
        let mut fruits = vec!["apple", "banana", "cherry"];
        let result = remove_fruit(&mut fruits, "cherry");
        
        assert!(result);
        assert_eq!(fruits, vec!["apple", "banana"]);
    }

    #[test]
    fn test_sort_fruits_basic() {
        let mut fruits = vec!["cherry", "apple", "banana"];
        sort_fruits(&mut fruits);
        
        assert_eq!(fruits, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_sort_fruits_already_sorted() {
        let mut fruits = vec!["apple", "banana", "cherry"];
        sort_fruits(&mut fruits);
        
        assert_eq!(fruits, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_sort_fruits_reverse_order() {
        let mut fruits = vec!["cherry", "banana", "apple"];
        sort_fruits(&mut fruits);
        
        assert_eq!(fruits, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_sort_fruits_with_duplicates() {
        let mut fruits = vec!["cherry", "apple", "banana", "apple"];
        sort_fruits(&mut fruits);
        
        assert_eq!(fruits, vec!["apple", "apple", "banana", "cherry"]);
    }

    #[test]
    fn test_sort_fruits_single_element() {
        let mut fruits = vec!["apple"];
        sort_fruits(&mut fruits);
        
        assert_eq!(fruits, vec!["apple"]);
    }

    #[test]
    fn test_sort_fruits_empty_vector() {
        let mut fruits: Vec<&str> = vec![];
        sort_fruits(&mut fruits);
        
        assert_eq!(fruits, vec![] as Vec<&str>);
    }

    #[test]
    fn test_sort_fruits_with_special_characters() {
        let mut fruits = vec!["mansikka", "apple", "elderberries"];
        sort_fruits(&mut fruits);
        
        assert_eq!(fruits, vec!["apple", "elderberries", "mansikka"]);
    }

    #[test]
    fn test_count_fruits_basic() {
        let fruits = vec!["apple", "banana", "cherry"];
        let counts = count_fruits(&fruits);
        
        assert_eq!(counts.len(), 3);
        assert_eq!(counts.get("apple"), Some(&1));
        assert_eq!(counts.get("banana"), Some(&1));
        assert_eq!(counts.get("cherry"), Some(&1));
    }

    #[test]
    fn test_count_fruits_with_duplicates() {
        let fruits = vec!["apple", "banana", "apple", "cherry", "banana", "apple"];
        let counts = count_fruits(&fruits);
        
        assert_eq!(counts.len(), 3);
        assert_eq!(counts.get("apple"), Some(&3));
        assert_eq!(counts.get("banana"), Some(&2));
        assert_eq!(counts.get("cherry"), Some(&1));
    }

    #[test]
    fn test_count_fruits_all_same() {
        let fruits = vec!["apple", "apple", "apple", "apple"];
        let counts = count_fruits(&fruits);
        
        assert_eq!(counts.len(), 1);
        assert_eq!(counts.get("apple"), Some(&4));
    }

    #[test]
    fn test_count_fruits_empty_vector() {
        let fruits: Vec<&str> = vec![];
        let counts = count_fruits(&fruits);
        
        assert_eq!(counts.len(), 0);
    }

    #[test]
    fn test_count_fruits_single_element() {
        let fruits = vec!["apple"];
        let counts = count_fruits(&fruits);
        
        assert_eq!(counts.len(), 1);
        assert_eq!(counts.get("apple"), Some(&1));
    }

    #[test]
    fn test_count_fruits_many_duplicates() {
        let fruits = vec!["apple", "apple", "banana", "banana", "banana", "cherry", "cherry", "cherry", "cherry"];
        let counts = count_fruits(&fruits);
        
        assert_eq!(counts.len(), 3);
        assert_eq!(counts.get("apple"), Some(&2));
        assert_eq!(counts.get("banana"), Some(&3));
        assert_eq!(counts.get("cherry"), Some(&4));
    }
}
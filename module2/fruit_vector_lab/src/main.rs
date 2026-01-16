use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Fruit Vector Lab")]
#[command(about = "Remove a specific fruit from a vector", long_about = None)]
struct Args {
    /// The fruit name to remove from the vector
    #[arg(short, long)]
    fruit: String,
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
}
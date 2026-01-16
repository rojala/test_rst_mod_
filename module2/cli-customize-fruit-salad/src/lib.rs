/*
This code defines a function called create_fruit_salad
that takes a mutable vector of strings as input and returns
a new vector of strings that contains the same elements as the input vector,
but in a random order. It also includes functionality to add random dressing options.
*/

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(mut fruits: Vec<String>) -> Vec<String> {
    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);

    fruits
}

pub fn get_random_dressing() -> String {
    let dressings = vec![
        "Honey Drizzle",
        "Maple Syrup",
        "Cinnamon Spice",
        "Vanilla Yogurt",
        "Mint Lime",
        "Balsamic Reduction",
        "Brown Sugar Glaze",
        "Coconut Cream",
    ];

    let mut rng = thread_rng();
    let dressing = dressings.choose(&mut rng).unwrap();
    dressing.to_string()
}

pub fn write_salad_to_csv(
    fruits: &[String],
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = fruits.join(",");
    std::fs::write(filename, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_fruit_salad_returns_same_fruits() {
        let fruits = vec![
            "apple".to_string(),
            "banana".to_string(),
            "orange".to_string(),
        ];
        let original_count = fruits.len();

        let result = create_fruit_salad(fruits);

        assert_eq!(
            result.len(),
            original_count,
            "Salad should have same number of fruits"
        );
    }

    #[test]
    fn test_create_fruit_salad_contains_all_fruits() {
        let fruits = vec![
            "apple".to_string(),
            "banana".to_string(),
            "orange".to_string(),
        ];

        let result = create_fruit_salad(fruits.clone());

        // Check that all original fruits are still in the result
        for fruit in &fruits {
            assert!(result.contains(fruit), "Result should contain {}", fruit);
        }
    }

    #[test]
    fn test_create_fruit_salad_with_empty_vector() {
        let fruits: Vec<String> = vec![];

        let result = create_fruit_salad(fruits);

        assert_eq!(result.len(), 0, "Empty salad should remain empty");
    }

    #[test]
    fn test_create_fruit_salad_with_single_fruit() {
        let fruits = vec!["apple".to_string()];

        let result = create_fruit_salad(fruits.clone());

        assert_eq!(result.len(), 1, "Single fruit salad should have one fruit");
        assert_eq!(result[0], "apple", "Single fruit should remain unchanged");
    }

    #[test]
    fn test_get_random_dressing_returns_valid_dressing() {
        let valid_dressings = vec![
            "Honey Drizzle",
            "Maple Syrup",
            "Cinnamon Spice",
            "Vanilla Yogurt",
            "Mint Lime",
            "Balsamic Reduction",
            "Brown Sugar Glaze",
            "Coconut Cream",
        ];

        let dressing = get_random_dressing();

        assert!(
            valid_dressings.contains(&dressing.as_str()),
            "Dressing '{}' should be in valid options",
            dressing
        );
    }

    #[test]
    fn test_get_random_dressing_returns_string() {
        let dressing = get_random_dressing();

        assert!(!dressing.is_empty(), "Dressing should not be empty");
        assert!(dressing.len() > 0, "Dressing should have length > 0");
    }

    #[test]
    fn test_create_fruit_salad_with_duplicate_fruits() {
        let fruits = vec![
            "apple".to_string(),
            "apple".to_string(),
            "banana".to_string(),
        ];

        let result = create_fruit_salad(fruits.clone());

        // Count apples in result
        let apple_count = result.iter().filter(|f| f == &"apple").count();
        assert_eq!(apple_count, 2, "Should maintain duplicate fruits");
    }

    #[test]
    fn test_create_fruit_salad_with_many_fruits() {
        let fruits = vec![
            "apple".to_string(),
            "banana".to_string(),
            "orange".to_string(),
            "grape".to_string(),
            "mango".to_string(),
            "pear".to_string(),
            "kiwi".to_string(),
            "strawberry".to_string(),
        ];
        let original_count = fruits.len();

        let result = create_fruit_salad(fruits.clone());

        assert_eq!(result.len(), original_count, "Should preserve all fruits");
        for fruit in &fruits {
            assert!(result.contains(fruit), "Should contain {}", fruit);
        }
    }

    #[test]
    fn test_write_salad_to_csv_success() {
        let fruits = vec![
            "apple".to_string(),
            "banana".to_string(),
            "orange".to_string(),
        ];
        let filename = "/tmp/test_salad.csv";

        let result = write_salad_to_csv(&fruits, filename);

        assert!(result.is_ok(), "Should successfully write to file");

        // Clean up
        let _ = std::fs::remove_file(filename);
    }

    #[test]
    fn test_write_salad_to_csv_content() {
        let fruits = vec!["apple".to_string(), "banana".to_string()];
        let filename = "/tmp/test_salad_content.csv";

        let _ = write_salad_to_csv(&fruits, filename);

        let content = std::fs::read_to_string(filename).expect("Could not read file");
        assert_eq!(content, "apple,banana", "CSV content should match");

        // Clean up
        let _ = std::fs::remove_file(filename);
    }

    #[test]
    fn test_write_salad_to_csv_with_special_characters() {
        let fruits = vec!["açaí".to_string(), "limão".to_string(), "café".to_string()];
        let filename = "/tmp/test_salad_special.csv";

        let result = write_salad_to_csv(&fruits, filename);
        assert!(result.is_ok());

        let content = std::fs::read_to_string(filename).expect("Could not read file");
        assert!(content.contains("açaí"));
        assert!(content.contains("limão"));

        // Clean up
        let _ = std::fs::remove_file(filename);
    }

    #[test]
    fn test_write_salad_to_csv_empty() {
        let fruits: Vec<String> = vec![];
        let filename = "/tmp/test_salad_empty.csv";

        let result = write_salad_to_csv(&fruits, filename);
        assert!(result.is_ok());

        let content = std::fs::read_to_string(filename).expect("Could not read file");
        assert_eq!(content, "", "Empty salad should write empty content");

        // Clean up
        let _ = std::fs::remove_file(filename);
    }
}

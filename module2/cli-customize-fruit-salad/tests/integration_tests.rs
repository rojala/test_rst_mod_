use fruit_salad_maker::{create_fruit_salad, get_random_dressing, write_salad_to_csv};

#[test]
fn test_integration_basic_salad() {
    let fruits = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ];

    let salad = create_fruit_salad(fruits.clone());

    assert_eq!(salad.len(), 3);
    for fruit in fruits {
        assert!(salad.contains(&fruit));
    }
}

#[test]
fn test_integration_salad_with_dressing() {
    let fruits = vec!["mango".to_string(), "pineapple".to_string()];
    let salad = create_fruit_salad(fruits);
    let dressing = get_random_dressing();

    assert_eq!(salad.len(), 2);
    assert!(!dressing.is_empty());
}

#[test]
fn test_integration_multiple_dressing_calls() {
    let mut dressings = vec![];

    for _ in 0..10 {
        dressings.push(get_random_dressing());
    }

    // Should have 10 dressing selections
    assert_eq!(dressings.len(), 10);

    // All should be non-empty
    for dressing in dressings {
        assert!(!dressing.is_empty());
    }
}

#[test]
fn test_integration_large_fruit_salad() {
    let fruits = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
        "elderberry".to_string(),
        "fig".to_string(),
        "grape".to_string(),
        "honeydew".to_string(),
        "kiwi".to_string(),
        "lemon".to_string(),
    ];

    let salad = create_fruit_salad(fruits.clone());

    assert_eq!(salad.len(), fruits.len());

    for fruit in fruits {
        assert!(salad.contains(&fruit));
    }
}

#[test]
fn test_integration_special_characters_in_fruit_names() {
    let fruits = vec![
        "açaí".to_string(),
        "café".to_string(),
        "limão".to_string(),
        "pêssego".to_string(),
    ];

    let salad = create_fruit_salad(fruits.clone());

    assert_eq!(salad.len(), 4);
    for fruit in fruits {
        assert!(salad.contains(&fruit));
    }
}

#[test]
fn test_integration_repeated_salad_creation() {
    let fruits = vec!["apple".to_string(), "banana".to_string()];

    let salad1 = create_fruit_salad(fruits.clone());
    let salad2 = create_fruit_salad(fruits.clone());

    // Both should have same fruits
    assert_eq!(salad1.len(), 2);
    assert_eq!(salad2.len(), 2);

    for fruit in vec!["apple", "banana"] {
        assert!(salad1.contains(&fruit.to_string()));
        assert!(salad2.contains(&fruit.to_string()));
    }

    // Note: Due to random shuffling, salad1 might be different from salad2
    // but both should contain the same fruits
}

#[test]
fn test_integration_salad_to_csv() {
    let fruits = vec![
        "apple".to_string(),
        "banana".to_string(),
        "orange".to_string(),
    ];
    let filename = "/tmp/test_integration_salad.csv";

    let salad = create_fruit_salad(fruits.clone());
    let result = write_salad_to_csv(&salad, filename);

    assert!(result.is_ok());

    let content = std::fs::read_to_string(filename).expect("Could not read file");
    assert!(!content.is_empty());

    // All fruits should be in the CSV
    for fruit in fruits {
        assert!(content.contains(&fruit));
    }

    // Clean up
    let _ = std::fs::remove_file(filename);
}

#[test]
fn test_integration_full_workflow_with_csv_output() {
    let fruits = vec!["grape".to_string(), "kiwi".to_string(), "mango".to_string()];
    let output_file = "/tmp/test_full_workflow.csv";

    // Create salad with dressing
    let salad = create_fruit_salad(fruits.clone());
    let _dressing = get_random_dressing();

    // Write to output file
    let result = write_salad_to_csv(&salad, output_file);
    assert!(result.is_ok());

    // Verify output file exists and has content
    let content = std::fs::read_to_string(output_file).expect("Could not read output file");
    assert_eq!(salad.len(), 3);
    assert!(!content.is_empty());

    // Clean up
    let _ = std::fs::remove_file(output_file);
}

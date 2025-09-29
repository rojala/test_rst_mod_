use rand::seq::SliceRandom;
use rand::thread_rng;

/// Creates a fruit salad by selecting a specified number of fruits from either a predefined list or a custom list.
///
/// # Parameters
/// - `num_fruits`: The number of fruits to include in the salad. If set to 0, all available fruits are used.
/// - `custom_fruits`: An optional slice of custom fruit names. If empty, a predefined list is used.
/// - `alphabetical`: If true, the selected fruits are sorted alphabetically; otherwise, they are shuffled randomly.
///
/// # Returns
/// Returns `Ok(Vec<String>)` containing the selected fruits, or `Err(String)` if the requested number exceeds available fruits.
pub fn create_fruit_salad(
    num_fruits: usize,
    custom_fruits: &[String],
    alphabetical: bool,
) -> Result<Vec<String>, String> {
    // List of predefined fruits
    let fruits = vec![
        "Arbutus".to_string(),
        "Loquat".to_string(),
        "Strawberry Tree Berry".to_string(),
        "Pomegranate".to_string(),
        "Fig".to_string(),
        "Cherry".to_string(),
        "Orange".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
        "Apple".to_string(),
    ];

    let fruits = if !custom_fruits.is_empty() {
        custom_fruits.to_vec()
    } else {
        fruits
    };

    // Validate input
    if num_fruits > fruits.len() {
        return Err("Requested number of fruits exceeds available fruits.".to_string());
    }

    let num_fruits = if num_fruits == 0 {
        fruits.len()
    } else {
        num_fruits
    };

    // Sort or shuffle the fruits based on the 'alphabetical' flag.
    let fruits = if alphabetical {
        let mut sorted_fruits = fruits.clone();
        sorted_fruits.sort();
        sorted_fruits
    } else {
        let mut rng = thread_rng();
        let mut fruits = fruits;
        fruits.shuffle(&mut rng);
        fruits
    };

    Ok(fruits.into_iter().take(num_fruits).collect())
}

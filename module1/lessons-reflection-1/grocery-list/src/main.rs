// Create a program that stores a grocery list in a vector and prints out each item
fn main() {
    let grocery_list = vec![
        "Apples",
        "Bananas",
        "Carrots",
        "Dairy Milk",
        "Eggs",
        "Flour",
        "Grapes",
    ];

    println!("Grocery List:");
    for item in &grocery_list {
        println!("- {}", item);
    }
}

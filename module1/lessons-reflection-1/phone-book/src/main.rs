// Build a simple phone book as a hash map of names to phone numbers
use std::collections::HashMap;

fn main() {
    let mut phone_book: HashMap<String, String> = HashMap::new();

    // Adding entries to the phone book
    phone_book.insert("Alice".to_string(), "555-1234".to_string());
    phone_book.insert("Bob".to_string(), "555-5678".to_string());
    phone_book.insert("Charlie".to_string(), "555-8765".to_string());

    // Retrieving and printing a phone number
    if let Some(number) = phone_book.get("Alice") {
        println!("Alice's phone number is: {}", number);
    } else {
        println!("Alice's phone number is not found.");
    }

    // Printing all entries in the phone book
    println!("Phone Book Entries:");
    for (name, number) in &phone_book {
        println!("{}: {}", name, number);
    }
}

// Implement a queue by using a VecDeque and simulate people waiting in line
use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<&str> = VecDeque::new();

    // Simulate people joining the line
    queue.push_back("Alice");
    queue.push_back("Bob");
    queue.push_back("Charlie");

    println!("People in line:");
    for person in &queue {
        println!("- {}", person);
    }

    // Simulate serving people from the line
    while let Some(served) = queue.pop_front() {
        println!("Serving: {}", served);
    }

    println!("All served, line is now empty.");
}

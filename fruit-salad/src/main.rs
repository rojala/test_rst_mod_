/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;

// Change app to accept fruits from command line arguments
// and add them to the fruit salad. If no arguments are provided,
// use the default list of fruits.
/*
fn add_fruit(fruit: &mut Vec<&str>, new_fruit: &str) {
    fruit.push(new_fruit);
}

*/

fn get_command_line_fruits() -> Vec<String> {
    std::env::args().skip(1).collect()
}

fn main() {
    let mut fruit = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];

    let binding = get_command_line_fruits();
    fruit.extend(binding.iter().map(|s| s.as_str()));

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}

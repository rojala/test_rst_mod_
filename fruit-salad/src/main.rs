/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

use clap::Parser; // clap is a command line argument parser for Rust
use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;

// 1.  Modify the program to accept fruits from the user and then add them to the fruit salad?
//     Using clap --fruit/-f option with multiple arguments.
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Fruits to add to the salad
    #[clap(short, long, value_name = "FRUIT", num_args = 1.., required = false)]
    fruit: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let mut fruit = vec![
        "Orange".to_string(),
        "Fig".to_string(),
        "Pomegranate".to_string(),
        "Cherry".to_string(),
        "Apple".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
    ];

    // Add user-provided fruits to the fruit salad
    for f in args.fruit {
        fruit.push(f);
    }

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

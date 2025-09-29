/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/

use clap::Parser; // clap is a command line argument parser for Rust
use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::VecDeque;

// 1. Modify the program to allow the user to add fruits to either end of the queue after shuffling?
//    Using clap command line arguments.
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Fruits to add to end of the salad
    #[clap(short = 'e', long, value_name = "FRUIT_END", num_args = 1.., required = false)]
    fruit_end: Vec<String>,
    /// Fruits to add to front of the salad
    #[clap(short = 'f', long, value_name = "FRUIT_FRONT", num_args = 1.., required = false)]
    fruit_front: Vec<String>,
}

fn main() {

    // 1. claps command line arguments
    let args = Args::parse();

    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the queue after shuffling
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // 3. Can you adjust the program to remove a fruit from either end of the queue,
    //    displaying the name of the removed fruit and the state of the queue afterwards?
    
    // Re-scramble (shuffle) the fruit - no sense to remove always Pomergranate and Cherry
    let mut rng = thread_rng();
    let mut fruit_vec: Vec<_> = fruit.into_iter().collect();
    fruit_vec.shuffle(&mut rng);
    fruit = fruit_vec.into_iter().collect();

    // Remove a fruit from the front
    if let Some(removed_fruit) = fruit.pop_front() {
        println!("Removed Fruit from Front: {}", removed_fruit);
    }

    // Remove a fruit from the back
    if let Some(removed_fruit) = fruit.pop_back() {
        println!("Removed Fruit from Back: {}", removed_fruit);
    }

    // 1. 1. Add user-provided fruits to the front of the fruit salad
    for f in args.fruit_front {
        let fruit_str = f.clone();
        fruit.push_front(Box::leak(fruit_str.into_boxed_str()));
    }

    // 1. 2. Add user-provided fruits to the end of the fruit salad
    for f in args.fruit_end {
        let fruit_str = f.clone();
        fruit.push_back(Box::leak(fruit_str.into_boxed_str()));
    }

    // 2. The SliceRandom trait provides a method choose(&self, rng: &R) -> Option<&T>. 
    //    Can you use this to select a random fruit from the salad?
    let mut rng = thread_rng();
    // Convert VecDeque to Vec for choosing a random fruit
    let fruit_vec: Vec<_> = fruit.iter().collect();
    let random_fruit = fruit_vec.choose(&mut rng);
    if let Some(random_fruit) = random_fruit {
        println!("Random Fruit: {}", random_fruit);
    }

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
/*
As with the VecDeque example, this code starts by creating a LinkedList of fruits,
converts it to a Vec for shuffling, and then converts it back to a LinkedList.
After the shuffling, it adds "Pomegranate", "Fig", and "Cherry" to the end of the list.
Finally, it prints out the final fruit salad.

This example shows how to use a LinkedList, but remember that LinkedList
has a higher memory overhead and worse cache locality than Vec or VecDeque,
so it's typically not the best choice unless you have a specific need for the properties
of a linked list. In Rust, it's usually better to use a Vec or VecDeque.

A LinkedList is a doubly-linked list, which means that each element in the list
has a pointer to the next element and the previous element.
A great example of when to use a LinkedList is when you need to insert or remove elements
from the middle of the list.
*/

use clap::Parser; // clap is a command line argument parser for Rust
use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::LinkedList;

// 1. Add command line option to give fruit and its position
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Postition of random fruits to be added
    #[clap(
        short,
        long,
        value_name = "POSITION",
        required = false,
        default_value_t = 0 // Store fruit at the front if position is not given
    )]
    position: usize,

    /// Fruits to add to the salad
    #[clap(short, long, value_name = "FRUIT", num_args = 1, required = false)]
    fruit: Vec<String>,

    /// Position of fruit to be removed
    #[clap(
        short,
        long,
        value_name = "REMOVE_POSITION",
        required = false,
        default_value_t = -1,
    )]
    remove_position: isize,
}

fn main() {
    let _args = Args::parse();

    let mut fruit: LinkedList<String> = LinkedList::new();
    fruit.push_back("Arbutus".to_string());
    fruit.push_back("Loquat".to_string());
    fruit.push_back("Strawberry Tree Berry".to_string());

    /*
    Please note that converting a LinkedList to a Vec and back to a LinkedList
    isn't a common operation in practice. I included
    it in this example to keep the code as similar as possible
    to the original VecDeque example.
     */

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to LinkedList
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the list after shuffling
    fruit.push_front("Pomegranate".to_string());
    fruit.push_back("Fig".to_string());
    fruit.push_back("Cherry".to_string());

    // 1. Push command line fruit to the specified position
    // if position is greater than length of list then push to back
    // fruit changed from &str to String to handle command line input
    if _args.position >= fruit.len() {
        for f in _args.fruit {
            fruit.push_back(f.to_string());
        }
    } else {
        // Create a temporary LinkedList to hold the new fruit list
        // Insert the new fruits at the specified position
        // Then append the rest of the original list
        // Finally, replace the original list with the new list
        let mut temp_list: LinkedList<String> = LinkedList::new();
        for (i, item) in fruit.iter().enumerate() {
            if i == _args.position {
                for f in &_args.fruit {
                    temp_list.push_back(f.to_string());
                }
            }
            temp_list.push_back(item.to_string());
        }
        fruit = temp_list;
    }

    // 2. The SliceRandom trait provides a method choose(&self, rng: &R) -> Option<&T>.
    //    Use this to select a random fruit from the salad.
    let mut rng = thread_rng();
    let fruit_vec: Vec<_> = fruit.iter().collect();
    if let Some(random_fruit) = fruit_vec.choose(&mut rng) {
        println!("Random Fruit: {}", random_fruit);
    }

    // 3. Adjust the program to remove a fruit from any position in the LinkedList,
    //    displaying the name of the removed fruit and the state of the list afterwards.
    //    If remove_position is not set then ignore this step.
    
    if _args.remove_position > 0 && _args.remove_position < fruit.len().try_into().unwrap() {
        let mut temp_list: LinkedList<String> = LinkedList::new();
        let mut removed_fruit = String::new();
        for (i, item) in fruit.iter().enumerate() {
            if i == _args.remove_position.try_into().unwrap() {
                removed_fruit = item.to_string();
                continue; // Skip adding this item to the new list
            }
            temp_list.push_back(item.to_string());
        }
        fruit = temp_list;
        println!("Removed Fruit: {} at {}", removed_fruit, _args.remove_position);
    } else {
        if _args.remove_position >= fruit.len().try_into().unwrap() {
            println!("Remove position is out of bounds {} >= {}, no fruit removed.", _args.remove_position, fruit.len());
        }
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

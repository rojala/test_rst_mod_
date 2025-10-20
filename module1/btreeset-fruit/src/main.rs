use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::BTreeSet;

/// Fruit selector with optional removal
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Fruit to remove from each selection
    #[arg(short, long)]
    remove: Option<String>,
}

fn main() {
    let args = Args::parse();

    let fruits = vec![
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
    ];
    let amounts = [1, 3, 5, 7, 9];

    let mut all_selected = BTreeSet::new();
    let mut rng = thread_rng();

    for amount in amounts.iter() {
        let mut fruit_set = BTreeSet::new();
        let mut shuffled_fruits = fruits.clone();
        shuffled_fruits.shuffle(&mut rng);

        for fruit in shuffled_fruits {
            fruit_set.insert(fruit);
            if fruit_set.len() >= *amount {
                break;
            }
        }

        if let Some(ref to_remove) = args.remove {
            fruit_set.remove(to_remove.as_str());
            println!("Removed {}", to_remove);
        }

        println!("{}: {:?}", amount, fruit_set);
        all_selected.extend(fruit_set);
    }

    println!("\nAll unique fruits selected (reversed):");
    println!("{:?}", all_selected.iter().rev() );
}

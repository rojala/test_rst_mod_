use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ord;
use std::collections::BinaryHeap;

/// Command-line arguments
#[derive(Parser)]
#[command(name = "Fruit Salad Generator")]
#[command(about = "Generates a fruit salad with figs and optional removal", long_about = None)]
struct Args {
    /// Fruit to remove (e.g., Fig or Apple)
    #[arg(short, long)]
    remove: Option<String>,
}

#[derive(Eq, PartialEq, Debug)]
enum Fruit {
    Fig,
    Other(String),
}

// We define Figs as the highest priority by implementing Ord
impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Fruit::Fig, Fruit::Fig) => std::cmp::Ordering::Equal,
            (Fruit::Fig, Fruit::Other(_)) => std::cmp::Ordering::Greater,
            (Fruit::Other(_), Fruit::Fig) => std::cmp::Ordering::Less,
            (Fruit::Other(_), Fruit::Other(_)) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn generate_fruit_salad() -> BinaryHeap<Fruit> {
    let mut rng = thread_rng();
    let fruits = vec![
        "Apple", "Orange", "Pear", "Peach", "Banana", "Fig", "Fig", "Fig", "Fig",
    ];
    let mut fruit_salad = BinaryHeap::new();

    let mut figs_count = 0;
    while figs_count < 2 {
        let fruit = fruits.choose(&mut rng).unwrap();
        if *fruit == "Fig" {
            figs_count += 1;
            fruit_salad.push(Fruit::Fig);
        } else {
            fruit_salad.push(Fruit::Other(fruit.to_string()));
        }
    }

    fruit_salad
}

fn remove_fruit(heap: BinaryHeap<Fruit>, target: &str) -> BinaryHeap<Fruit> {
    let filtered = heap
        .into_iter()
        .filter(|fruit| match fruit {
            Fruit::Fig => target.to_lowercase() != "fig",
            Fruit::Other(name) => name.to_lowercase() != target.to_lowercase(),
        })
        .collect::<Vec<_>>();

    BinaryHeap::from(filtered)
}

fn main() {
    let args = Args::parse();
    let fruit_salad = generate_fruit_salad();
    
    let fruit_salad = if let Some(fruit_to_remove) = args.remove {
        println!("Removing fruit: {}", fruit_to_remove);
        remove_fruit(fruit_salad, &fruit_to_remove)
    } else {
        fruit_salad
    };

    println!("Random Fruit Salad With Two Servings of Figs:");
    for fruit in fruit_salad.into_sorted_vec() {
        match fruit {
            Fruit::Fig => println!("Fig"),
            Fruit::Other(fruit_name) => println!("{}", fruit_name),
        }
    }
}

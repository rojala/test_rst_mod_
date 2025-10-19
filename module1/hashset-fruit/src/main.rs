use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

/// Simple program to generate random fruits
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of random fruits to generate (default: 100)
    #[arg(short, long, default_value_t = 100)]
    count: usize,
}

fn generate_fruit() -> &'static str {
    let fruits = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
    ];
    let mut rng = thread_rng();
    fruits.choose(&mut rng).unwrap()
}

fn main() {
    let args = Args::parse();
    let mut fruit_counts: HashMap<&str, usize> = HashMap::new();

    println!("Generating {} random fruits...", args.count);
    for _ in 0..args.count {
        let fruit = generate_fruit();
        *fruit_counts.entry(fruit).or_insert(0) += 1;
    }

    println!("\nFruit generation summary:");
    for (fruit, count) in &fruit_counts {
        println!("{:<10}: {}", fruit, count);
    }

    println!(
        "\nNumber of unique fruits generated: {}",
        fruit_counts.len()
    );

    let unique_fruits: Vec<&str> = fruit_counts.keys().cloned().collect();
    println!("\nUnique fruits generated ({}):", unique_fruits.len());
    println!("{}", unique_fruits.join(", "));
}


use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

/// Simple program to generate random fruits
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of random fruits to generate
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
    let mut fruit_set = HashSet::new();

    println!("Generating {} random fruits...", args.count);
    for _ in 0..args.count {
        fruit_set.insert(generate_fruit());
    }

    println!("Number of unique fruits generated: {}", fruit_set.len());
}

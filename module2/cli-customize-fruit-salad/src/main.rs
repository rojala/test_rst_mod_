/*
Usage:

cargo run -- fruits.csv
or
cargo run -- --fruits "apple, pear"

 */

use clap::Parser;
use fruit_salad_maker::{create_fruit_salad, get_random_dressing, write_salad_to_csv};

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Make a Fruit Salad"
)]
struct Opts {
    /// Fruits input as a string of comma separated values
    #[clap(short, long)]
    fruits: Option<String>,
    csvfile: Option<String>,
    /// Add random dressing to the fruit salad
    #[clap(short, long)]
    dressing: bool,
    /// Output CSV file for the resulting fruit salad
    #[clap(short, long)]
    output: Option<String>,
}

// Function that converts a csv file to a vector of strings
fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',').map(|s| s.trim().to_string()).collect()
}
fn display_fruit_salad(fruits: Vec<String>, dressing: Option<String>) {
    println!("Your fruit salad contains:");
    for fruit in fruits {
        println!("{}", fruit);
    }
    if let Some(d) = dressing {
        println!("\nDressing: {}", d);
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    // Use fruits from CSV file or command-line input
    let fruit_list = match opts.csvfile {
        Some(filename) => {
            let fruits = std::fs::read_to_string(filename).expect("Could not read file");
            csv_to_vec(&fruits)
        }
        None => opts
            .fruits
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect(),
    };

    // Add dressing if requested
    let dressing = if opts.dressing {
        Some(get_random_dressing())
    } else {
        None
    };

    // display fruit salad
    let fruit_salad = create_fruit_salad(fruit_list);
    display_fruit_salad(fruit_salad.clone(), dressing);

    // Write to output CSV file if requested
    if let Some(output_file) = opts.output {
        match write_salad_to_csv(&fruit_salad, &output_file) {
            Ok(_) => println!("\nFruit salad saved to: {}", output_file),
            Err(e) => eprintln!("Error writing to file: {}", e),
        }
    }
}

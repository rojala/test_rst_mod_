use csv::Writer;
use csvwrite::{process_csv_with_buffers, DiscountOptions};
use std::env;
use std::error::Error;
use std::fs::File;

fn orginal() -> Result<(), Box<dyn Error>> {
    // Create a list of fruits and their prices
    let fruits = [
        ("Apple", 1.25),
        ("Banana", 0.75),
        ("Orange", 1.00),
        ("Mango", 2.50),
        ("Pineapple", 3.00),
    ];

    // Open the CSV file in write mode
    let mut wtr = Writer::from_path("output.csv")?;

    // Write the header row
    wtr.write_record(["Fruit", "Price"])?;

    // Write each fruit and its price to the CSV file
    for (fruit, price) in fruits {
        wtr.write_record([fruit, &price.to_string()])?; // Convert price to string
    }

    wtr.flush()?; // Ensure data is written

    Ok(())
}

fn discounted(options: &DiscountOptions) -> Result<(), Box<dyn Error>> {
    let input = File::open("products.csv")?;
    let output = File::create("discounted_products.csv")?;
    let summary = process_csv_with_buffers(input, output, options, 8 * 1024, 8 * 1024)?;

    println!("Processed {} products.", summary.total_items);
    println!("Discounted products: {}.", summary.discounted_items);
    println!("Total original value: ${:.2}", summary.total_original);
    println!("Total discounted value: ${:.2}", summary.total_final);
    println!("Total savings: ${:.2}", summary.total_savings);
    println!("Threads used: {}", options.thread_count);
    println!("Wrote output to discounted_products.csv");

    Ok(())
}

fn parse_options(args: &[String]) -> Result<(bool, DiscountOptions), Box<dyn Error>> {
    let mut run_original = false;
    let mut options = DiscountOptions::default();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--original" => {
                run_original = true;
                i += 1;
            }
            "--clamp" => {
                options.clamp = true;
                i += 1;
            }
            "--discount" => {
                if i + 1 >= args.len() {
                    return Err("Missing value for --discount".into());
                }
                options.discount_percent = args[i + 1].parse()?;
                i += 2;
            }
            "--extra-discount" => {
                if i + 1 >= args.len() {
                    return Err("Missing value for --extra-discount".into());
                }
                options.extra_discount_percent = args[i + 1].parse()?;
                i += 2;
            }
            "--product" => {
                if i + 1 >= args.len() {
                    return Err("Missing product name for --product".into());
                }
                options.product_filter = Some(args[i + 1].clone());
                i += 2;
            }
            "--threads" => {
                if i + 1 >= args.len() {
                    return Err("Missing value for --threads".into());
                }
                options.thread_count = args[i + 1].parse()?;
                if options.thread_count == 0 {
                    return Err("--threads must be greater than 0".into());
                }
                i += 2;
            }
            "--help" | "-h" => {
                println!(
                    "Usage: csvwrite [--original] [--clamp] [--discount <percent>] [--extra-discount <percent>] [--product <name>] [--threads <count>]"
                );
                std::process::exit(0);
            }
            unknown => {
                return Err(format!("Unknown argument: {unknown}").into());
            }
        }
    }

    Ok((run_original, options))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let (run_original, options) = parse_options(&args)?;

    if run_original {
        orginal()?;
    } else {
        discounted(&options)?;
    }

    Ok(())
}


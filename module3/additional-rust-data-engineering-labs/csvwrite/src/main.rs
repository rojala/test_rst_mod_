use csv::{Reader, Writer};
use std::env;
use std::error::Error;

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

fn discounted(clamp: bool) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("products.csv")?;
    let mut wtr = Writer::from_path("discounted_products.csv")?;

    wtr.write_record(["Product", "Price"])?;

    for result in rdr.records() {
        let record = result?;
        let product_name = record.get(0).unwrap_or("");
        let original_price: f64 = record.get(1).unwrap_or("0").parse()?;
        let discounted_price = original_price * 0.9;

        let final_price = if clamp {
            discounted_price.clamp(0.0, original_price)
        } else {
            discounted_price
        };

        wtr.write_record([product_name, &format!("{final_price:.2}")])?;
    }

    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let run_original = args.iter().any(|arg| arg == "--original");
    let clamp = args.iter().any(|arg| arg == "--clamp");

    if run_original {
        orginal()?;
    } else {
        discounted(clamp)?;
    }

    Ok(())
}


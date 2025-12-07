// Implement a program that reads data from a CSV file into a vector and calculates simple summary statistics like min, max, and mean on one column.
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_csv_column(filename: &str, column_index: usize) -> Result<Vec<f64>, Box<dyn Error>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    let mut values = Vec::new();
    
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        // Skip header row
        if line_num == 0 {
            continue;
        }
        
        let fields: Vec<&str> = line.split(',').collect();
        if column_index < fields.len() {
            if let Ok(value) = fields[column_index].trim().parse::<f64>() {
                values.push(value);
            }
        }
    }
    
    Ok(values)
}

fn calculate_statistics(values: &[f64]) -> Option<(f64, f64, f64)> {
    if values.is_empty() {
        return None;
    }
    
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let sum: f64 = values.iter().sum();
    let mean = sum / values.len() as f64;
    
    Some((min, max, mean))
}

fn main() {
    let filename = "data.csv";
    let column_index = 2; // Change this to select different column (0-indexed)
    
    match read_csv_column(filename, column_index) {
        Ok(values) => {
            println!("Read {} values from column {}", values.len(), column_index);
            
            match calculate_statistics(&values) {
                Some((min, max, mean)) => {
                    println!("\nSummary Statistics:");
                    println!("Minimum: {:.2}", min);
                    println!("Maximum: {:.2}", max);
                    println!("Mean:    {:.2}", mean);
                }
                None => println!("No data to calculate statistics"),
            }
        }
        Err(e) => eprintln!("Error reading CSV: {}", e),
    }
}

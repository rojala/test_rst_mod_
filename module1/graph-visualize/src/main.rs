extern crate rasciigraph;

use rasciigraph::{Config, plot};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

// Define struct to represent JSON data structure
#[derive(Debug, Serialize, Deserialize)]
struct CityData {
    city: String,
    distance: f64,
}

fn main() -> io::Result<()> {
    // Read JSON data from file
    let file_content = fs::read_to_string("data.json")?;

    // Parse JSON into vector of CityData structs
    let city_data: Vec<CityData> =
        serde_json::from_str(&file_content).expect("Failed to parse JSON data");

    // Extract cities and distances into separate vectors
    let cities: Vec<String> = city_data.iter().map(|data| data.city.clone()).collect();
    let distances_travelled: Vec<f64> = city_data.iter().map(|data| data.distance).collect();

    println!("{}", cities.join(" > "));

    println!(
        "{}",
        plot(
            distances_travelled,
            Config::default()
                .with_offset(10)
                .with_height(10)
                .with_caption("Travelled distances (km)".to_string())
        )
    );

    Ok(())
}

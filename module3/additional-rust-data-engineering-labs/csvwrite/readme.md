# CSV write

## Lab Goal
Practice working with CSV data in Rust by reading, transforming, and writing CSV files using the csv crate in /home/coder/additional-rust-data-engineering-labs.
Note: path will be differnt...

## Lab Description

In this lab, you will use the csv crate to read in a CSV file containing product data, transform the data by calculating discounts, and write the updated data to a new CSV file. This will demonstrate common workflows for processing and modifying CSV data in Rust.

## Lab Steps

1. Import the csv crate and open products.csv in read mode using csv::Reader
2. Iterate through the rows and extract the product name and price
3. Calculate a 10% discount on the price
4. Open discounted_products.csv in write mode using csv::Writer
5. Write the header row with column names
6. Write each product's name and discounted price to the output file
7. Flush the writer when finished to save the data

## Reflection Questions

1. What are some advantages of using Rust for processing CSV data vs a scripting language like Python?
2. How does buffered reading with csv::Reader improve CSV parsing performance?
3. What error handling strategies should be used when working with I/O in Rust?
4. How can the code be extended to support more complex CSV processing workflows?
5. What Rust concurrency features could help speed up large CSV processing jobs?

## Challenge Exploration

1. Modify the code to calculate multiple kinds of discounts based on product categories
2. Print a summary of the total savings from the discounts
3. Process the CSV data concurrently using threads
4. Write benchmarks to compare performance with different buffer sizes
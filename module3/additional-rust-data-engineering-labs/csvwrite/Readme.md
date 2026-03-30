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
	- Rust typically provides better runtime performance and lower memory overhead for large CSV workloads.
	- Its ownership and borrowing model prevents many memory bugs at compile time.
	- Strong static typing catches data-shape and parsing mistakes earlier.
	- Single binary deployment can simplify running jobs in production environments.
2. How does buffered reading with csv::Reader improve CSV parsing performance?
	- Buffered reading reduces system calls by reading larger chunks from disk at once.
	- Fewer I/O calls means less overhead and better throughput.
	- The parser can process in-memory buffers efficiently instead of waiting on many small reads.
3. What error handling strategies should be used when working with I/O in Rust?
	- Return `Result` from functions and propagate recoverable errors with `?`.
	- Add context where useful (for example, include file path or operation details in logs/errors).
	- Handle expected failures explicitly (missing file, malformed record, parse errors).
	- Avoid `unwrap()` in production code; prefer graceful handling and clear error messages.
4. How can the code be extended to support more complex CSV processing workflows?
	- Deserialize rows into structs with `serde` for safer field access and validation.
	- Add transformation stages (filter, map, group, aggregate) as separate functions/modules.
	- Support configurable inputs/outputs and rules via CLI arguments or config files.
	- Add schema validation, data quality checks, and optional enrichment from other sources.
5. What Rust concurrency features could help speed up large CSV processing jobs?
	- `std::thread` can split work across chunks of data.
	- Message passing with `std::sync::mpsc` or shared state with `Arc<Mutex<_>>` coordinates workers safely.
	- Data parallelism libraries like `rayon` can parallelize CPU-heavy transforms with less boilerplate.
	- Async I/O (`tokio`) can help when pipeline stages are I/O-bound or network-heavy.

## Challenge Exploration

1. Modify the code to calculate multiple kinds of discounts based on product categories
    Follwoing options added: --discount, --extra-disocunt, --product, --clamp.
2. Print a summary of the total savings from the discounts
3. Process the CSV data concurrently using threads
    --threads <n> argument added.
4. Write benchmarks to compare performance with different buffer sizes
	Added Criterion benchmark: benches/csv_processing.rs

## Command Line Parameters

The program now supports configurable discount behavior from the command line.

- --discount <percent>
	- Base discount percent.
	- Default is 10.
- --extra-discount <percent>
	- Adds an additional discount on top of the base discount.
	- Default is 0.
- --product <name>
	- Applies the discount only to a specific product name.
	- If not provided, discount is applied to all products.
- --threads <count>
	- Number of worker threads used to process CSV rows concurrently.
	- Default is 1 (single-threaded).
- --clamp
	- Clamps discounted price so it does not go below 0 or above original price.
- --original
	- Runs the original fruit-writing example instead of products.csv discount flow.

Total discount calculation:

- total discount percent = discount + extra-discount
- default total discount percent = 10 + 0 = 10

Examples:

- Apply default 10 percent discount to all products:
	- cargo run
- Apply 15 percent total discount to all products (10 + 5):
	- cargo run -- --extra-discount 5
- Apply default 10 percent discount only to Apple:
	- cargo run -- --product Apple
- Apply 20 percent total discount only to Banana with clamping:
	- cargo run -- --discount 15 --extra-discount 5 --product Banana --clamp
- Process discounts concurrently with 4 threads:
	- cargo run -- --threads 4

## Benchmarking

Benchmarks compare CSV processing performance across:

- Different buffer sizes (4 KB, 16 KB, 64 KB, 256 KB) using 1 thread.
- Different thread counts (1, 2, 4, 8) using 64 KB buffers.

Run benchmarks:

- cargo bench --bench csv_processing

Run benchmarks quickly (short sample run):

- cargo bench --bench csv_processing -- --sample-size 10 --warm-up-time 1 --measurement-time 2

Benchmark source file:

- benches/csv_processing.rs
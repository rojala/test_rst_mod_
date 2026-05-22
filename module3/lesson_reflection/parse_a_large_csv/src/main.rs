use std::error::Error;
use std::fs;
use std::path::Path;

const DATA_DIR: &str = "data";
const OUTPUT_DIR: &str = "output";
const DATASET_PATH: &str = "data/files_dataset.csv";
const TOTAL_ROWS: usize = 200_000;

fn main() -> Result<(), Box<dyn Error>> {
	fs::create_dir_all(DATA_DIR)?;
	fs::create_dir_all(OUTPUT_DIR)?;

	generate_dataset(DATASET_PATH, TOTAL_ROWS)?;
	let summary = filter_dataset(DATASET_PATH, OUTPUT_DIR)?;

	println!("Dataset created: {DATASET_PATH}");
	println!("Rows generated: {}", summary.total_rows);
	println!("Rows parsed: {}", summary.parsed_rows);
	println!("Rows skipped (bad data): {}", summary.skipped_rows);
	println!();
	println!("Created filtered files:");
	println!(
		"  - {}/new_large_verified_files.csv ({})",
		OUTPUT_DIR, summary.new_large_verified
	);
	println!(
		"  - {}/stale_log_files.csv ({})",
		OUTPUT_DIR, summary.stale_logs
	);
	println!(
		"  - {}/failed_or_corrupt_files.csv ({})",
		OUTPUT_DIR, summary.failed_or_corrupt
	);

	Ok(())
}

struct Summary {
	total_rows: usize,
	parsed_rows: usize,
	skipped_rows: usize,
	new_large_verified: usize,
	stale_logs: usize,
	failed_or_corrupt: usize,
}

fn generate_dataset(path: &str, rows: usize) -> Result<(), Box<dyn Error>> {
	let mut writer = csv::Writer::from_path(path)?;
	writer.write_record([
		"id",
		"file_name",
		"owner",
		"region",
		"status",
		"size_mb",
		"last_modified_days_ago",
		"checksum_ok",
		"extension",
	])?;

	let owners = ["team-a", "team-b", "team-c", "team-d"];
	let regions = ["us-east-1", "us-west-2", "eu-west-1", "ap-south-1"];
	let statuses = ["new", "processed", "archived", "corrupt"];
	let extensions = ["csv", "json", "parquet", "log", "bin"];

	for i in 0..rows {
		let id = i + 1;
		let owner = owners[i % owners.len()];
		let region = regions[(i / 3) % regions.len()];
		let status = statuses[(i / 7) % statuses.len()];
		let extension = extensions[(i / 11) % extensions.len()];
		let size_mb = ((i * 37) % 2048) + 1;
		let last_modified_days_ago = (i * 13) % 90;
		let checksum_ok = if i % 17 == 0 { "false" } else { "true" };
		let file_name = format!("file_{id:07}.{extension}");

		writer.write_record([
			id.to_string(),
			file_name,
			owner.to_string(),
			region.to_string(),
			status.to_string(),
			size_mb.to_string(),
			last_modified_days_ago.to_string(),
			checksum_ok.to_string(),
			extension.to_string(),
		])?;
	}

	writer.flush()?;
	Ok(())
}

fn filter_dataset(dataset_path: &str, output_dir: &str) -> Result<Summary, Box<dyn Error>> {
	let mut reader = csv::ReaderBuilder::new()
		.has_headers(true)
		.from_path(dataset_path)?;

	let headers = reader.headers()?.clone();

	let new_large_path = Path::new(output_dir).join("new_large_verified_files.csv");
	let stale_logs_path = Path::new(output_dir).join("stale_log_files.csv");
	let failed_path = Path::new(output_dir).join("failed_or_corrupt_files.csv");

	let mut new_large_writer = csv::Writer::from_path(new_large_path)?;
	let mut stale_logs_writer = csv::Writer::from_path(stale_logs_path)?;
	let mut failed_writer = csv::Writer::from_path(failed_path)?;

	new_large_writer.write_record(&headers)?;
	stale_logs_writer.write_record(&headers)?;
	failed_writer.write_record(&headers)?;

	let mut summary = Summary {
		total_rows: 0,
		parsed_rows: 0,
		skipped_rows: 0,
		new_large_verified: 0,
		stale_logs: 0,
		failed_or_corrupt: 0,
	};

	for result in reader.records() {
		summary.total_rows += 1;
		let record = match result {
			Ok(r) => r,
			Err(_) => {
				summary.skipped_rows += 1;
				continue;
			}
		};

		let status = get_field(&record, 4);
		let size_mb = get_field(&record, 5).parse::<usize>();
		let last_modified_days_ago = get_field(&record, 6).parse::<usize>();
		let checksum_ok = get_field(&record, 7);
		let extension = get_field(&record, 8);

		let (size_mb, last_modified_days_ago) = match (size_mb, last_modified_days_ago) {
			(Ok(s), Ok(d)) => (s, d),
			_ => {
				summary.skipped_rows += 1;
				continue;
			}
		};

		summary.parsed_rows += 1;

		// Condition 1: files that are new, large, and checksum-verified.
		if status == "new" && size_mb >= 700 && checksum_ok == "true" {
			new_large_writer.write_record(&record)?;
			summary.new_large_verified += 1;
		}

		// Condition 2: stale log files (good for cleanup candidates).
		if extension == "log" && last_modified_days_ago >= 30 {
			stale_logs_writer.write_record(&record)?;
			summary.stale_logs += 1;
		}

		// Condition 3: either failed checksum or explicitly marked corrupt.
		if checksum_ok == "false" || status == "corrupt" {
			failed_writer.write_record(&record)?;
			summary.failed_or_corrupt += 1;
		}
	}

	new_large_writer.flush()?;
	stale_logs_writer.flush()?;
	failed_writer.flush()?;

	Ok(summary)
}

fn get_field(record: &csv::StringRecord, idx: usize) -> &str {
	record.get(idx).unwrap_or("")
}


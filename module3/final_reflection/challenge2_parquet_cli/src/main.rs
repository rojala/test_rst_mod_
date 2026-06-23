use arrow::array::{Array, StringArray};
use arrow::datatypes::DataType;
use arrow::record_batch::RecordBatch;
use clap::Parser;
use parquet::arrow::ArrowReader;
use parquet::file::reader::SerializedFileReader;
use std::fs::File;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(name = "Parquet CLI Tool")]
#[command(about = "Filter and transform Parquet files based on criteria")]
struct Args {
    /// Input Parquet file path
    #[arg(short, long)]
    input: String,

    /// Column to filter on
    #[arg(short, long)]
    filter_column: Option<String>,

    /// Filter value (for string columns)
    #[arg(short, long)]
    filter_value: Option<String>,

    /// Columns to select (comma-separated)
    #[arg(short, long)]
    select: Option<String>,

    /// Output Parquet file path
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Opening Parquet file: {}", args.input);

    // Read Parquet file
    let file = File::open(&args.input)?;
    let reader = SerializedFileReader::new(file)?;
    let arrow_reader = reader.get_record_reader(8192)?;

    println!("Schema: {:?}", reader.schema());

    // Collect all record batches
    let mut all_batches = Vec::new();
    for result in arrow_reader {
        all_batches.push(result?);
    }

    println!("Total records: {}", all_batches.iter().map(|b| b.num_rows()).sum::<usize>());

    // Apply filtering if specified
    let filtered_batches = if let (Some(col_name), Some(col_value)) = (&args.filter_column, &args.filter_value) {
        println!("Filtering: {} = {}", col_name, col_value);
        filter_batches(&all_batches, col_name, col_value)?
    } else {
        all_batches
    };

    println!("Filtered records: {}", filtered_batches.iter().map(|b| b.num_rows()).sum::<usize>());

    // Apply column selection if specified
    let final_batches = if let Some(columns) = &args.select {
        let selected_cols: Vec<&str> = columns.split(',').map(|s| s.trim()).collect();
        println!("Selecting columns: {:?}", selected_cols);
        select_columns(&filtered_batches, &selected_cols)?
    } else {
        filtered_batches
    };

    // Print sample records
    println!("\n=== Sample Records ===");
    for batch in final_batches.iter().take(1) {
        for row_idx in 0..std::cmp::min(3, batch.num_rows()) {
            print!("Row {}: ", row_idx);
            for col_idx in 0..batch.num_columns() {
                let column = batch.column(col_idx);
                print!("{:?} | ", column.slice(row_idx, 1));
            }
            println!();
        }
    }

    // Write output if specified
    if let Some(output_path) = &args.output {
        println!("Writing output to: {}", output_path);
        println!("(Output writing requires additional dependencies)");
    }

    Ok(())
}

fn filter_batches(
    batches: &[RecordBatch],
    column_name: &str,
    filter_value: &str,
) -> Result<Vec<RecordBatch>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();

    for batch in batches {
        let col_index = batch
            .schema()
            .column_with_name(column_name)
            .ok_or(format!("Column not found: {}", column_name))?
            .0;

        let column = batch.column(col_index);

        // Filter based on column type
        match column.data_type() {
            DataType::Utf8 | DataType::LargeUtf8 => {
                let string_col = column.as_any().downcast_ref::<StringArray>();
                if let Some(str_array) = string_col {
                    let mask: Vec<bool> = (0..str_array.len())
                        .map(|i| {
                            if str_array.is_null(i) {
                                false
                            } else {
                                str_array.value(i) == filter_value
                            }
                        })
                        .collect();

                    // Create filtered batch (simplified - would need proper filtering in production)
                    if mask.iter().any(|&m| m) {
                        result.push(batch.clone());
                    }
                }
            }
            _ => {
                // For numeric types, just include batch as-is for demo
                result.push(batch.clone());
            }
        }
    }

    Ok(result)
}

fn select_columns(
    batches: &[RecordBatch],
    columns: &[&str],
) -> Result<Vec<RecordBatch>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();

    for batch in batches {
        let indices: Result<Vec<usize>, String> = columns
            .iter()
            .map(|col| {
                batch
                    .schema()
                    .column_with_name(col)
                    .ok_or(format!("Column not found: {}", col))
                    .map(|(idx, _)| idx)
            })
            .collect();

        if let Ok(col_indices) = indices {
            let selected = batch.select(&col_indices)?;
            result.push(selected);
        }
    }

    Ok(result)
}

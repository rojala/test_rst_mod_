use csv::{ReaderBuilder, WriterBuilder};
use std::error::Error;
use std::io::{BufReader, BufWriter, Read, Write};
use std::thread;

#[derive(Debug, Clone)]
pub struct DiscountOptions {
    pub clamp: bool,
    pub discount_percent: f64,
    pub extra_discount_percent: f64,
    pub product_filter: Option<String>,
    pub thread_count: usize,
}

impl Default for DiscountOptions {
    fn default() -> Self {
        Self {
            clamp: false,
            discount_percent: 10.0,
            extra_discount_percent: 0.0,
            product_filter: None,
            thread_count: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcessingSummary {
    pub total_items: usize,
    pub discounted_items: usize,
    pub total_original: f64,
    pub total_final: f64,
    pub total_savings: f64,
}

fn calculate_final_price(
    product_name: &str,
    original_price: f64,
    options: &DiscountOptions,
) -> (f64, bool) {
    let should_discount = match &options.product_filter {
        Some(target) => product_name.eq_ignore_ascii_case(target),
        None => true,
    };

    let total_discount = (options.discount_percent + options.extra_discount_percent) / 100.0;
    let discounted_price = if should_discount {
        original_price * (1.0 - total_discount)
    } else {
        original_price
    };

    let final_price = if options.clamp {
        discounted_price.clamp(0.0, original_price)
    } else {
        discounted_price
    };

    (final_price, should_discount)
}

pub fn process_csv_with_buffers<R: Read, W: Write>(
    reader: R,
    writer: W,
    options: &DiscountOptions,
    reader_buffer_size: usize,
    writer_buffer_size: usize,
) -> Result<ProcessingSummary, Box<dyn Error>> {
    if options.thread_count == 0 {
        return Err("thread_count must be greater than 0".into());
    }
    if reader_buffer_size == 0 || writer_buffer_size == 0 {
        return Err("buffer sizes must be greater than 0".into());
    }

    let buffered_reader = BufReader::with_capacity(reader_buffer_size, reader);
    let buffered_writer = BufWriter::with_capacity(writer_buffer_size, writer);

    let mut rdr = ReaderBuilder::new().from_reader(buffered_reader);
    let mut wtr = WriterBuilder::new().from_writer(buffered_writer);

    wtr.write_record(["Product", "Price"])?;

    let mut input_rows: Vec<(usize, String, f64)> = Vec::new();
    for (index, result) in rdr.records().enumerate() {
        let record = result?;
        let product_name = record.get(0).unwrap_or("").to_string();
        let original_price: f64 = record.get(1).unwrap_or("0").parse()?;
        input_rows.push((index, product_name, original_price));
    }

    let mut processed_rows: Vec<(usize, String, f64, f64, bool)> = Vec::new();

    if options.thread_count <= 1 || input_rows.len() <= 1 {
        for (index, product_name, original_price) in input_rows {
            let (final_price, should_discount) =
                calculate_final_price(&product_name, original_price, options);
            processed_rows.push((index, product_name, original_price, final_price, should_discount));
        }
    } else {
        let worker_count = options.thread_count.min(input_rows.len());
        let chunk_size = input_rows.len().div_ceil(worker_count);
        let mut handles = Vec::new();

        for chunk in input_rows.chunks(chunk_size) {
            let chunk_rows = chunk.to_vec();
            let opts = options.clone();
            let handle = thread::spawn(move || {
                let mut local_rows = Vec::with_capacity(chunk_rows.len());
                for (index, product_name, original_price) in chunk_rows {
                    let (final_price, should_discount) =
                        calculate_final_price(&product_name, original_price, &opts);
                    local_rows.push((index, product_name, original_price, final_price, should_discount));
                }
                local_rows
            });
            handles.push(handle);
        }

        for handle in handles {
            let local_rows = handle
                .join()
                .map_err(|_| "A worker thread panicked while processing CSV rows")?;
            processed_rows.extend(local_rows);
        }

        processed_rows.sort_by_key(|(index, _, _, _, _)| *index);
    }

    let mut total_original = 0.0;
    let mut total_final = 0.0;
    let mut discounted_items = 0usize;
    let mut total_items = 0usize;

    for (_index, product_name, original_price, final_price, should_discount) in processed_rows {
        total_items += 1;
        total_original += original_price;
        total_final += final_price;
        if should_discount {
            discounted_items += 1;
        }
        wtr.write_record([product_name.as_str(), &format!("{final_price:.2}")])?;
    }

    wtr.flush()?;

    Ok(ProcessingSummary {
        total_items,
        discounted_items,
        total_original,
        total_final,
        total_savings: (total_original - total_final).max(0.0),
    })
}
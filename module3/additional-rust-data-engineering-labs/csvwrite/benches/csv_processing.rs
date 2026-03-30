use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use csvwrite::{process_csv_with_buffers, DiscountOptions};
use std::io::Cursor;

fn generate_products_csv(rows: usize) -> Vec<u8> {
    let mut data = String::with_capacity(rows * 20);
    data.push_str("Product,Price\n");
    for i in 0..rows {
        let price = 1.0 + (i % 100) as f64 * 0.25;
        data.push_str(&format!("Product{},{}\n", i, price));
    }
    data.into_bytes()
}

fn run_case(input_data: &[u8], reader_buffer: usize, writer_buffer: usize, thread_count: usize) {
    let options = DiscountOptions {
        thread_count,
        ..DiscountOptions::default()
    };

    let reader = Cursor::new(input_data);
    let writer = Cursor::new(Vec::<u8>::with_capacity(input_data.len()));

    let summary = process_csv_with_buffers(reader, writer, &options, reader_buffer, writer_buffer)
        .expect("benchmark case should process successfully");

    black_box(summary.total_savings);
}

fn bench_buffer_sizes(c: &mut Criterion) {
    let rows = 200_000usize;
    let input_data = generate_products_csv(rows);
    let mut group = c.benchmark_group("csv_buffer_sizes_threads_1");
    group.throughput(Throughput::Elements(rows as u64));

    for &buffer_size in &[4 * 1024usize, 16 * 1024usize, 64 * 1024usize, 256 * 1024usize] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}KB", buffer_size / 1024)),
            &buffer_size,
            |b, &size| {
                b.iter(|| run_case(&input_data, size, size, 1));
            },
        );
    }

    group.finish();
}

fn bench_thread_counts(c: &mut Criterion) {
    let rows = 200_000usize;
    let input_data = generate_products_csv(rows);
    let mut group = c.benchmark_group("csv_threads_buffer_64kb");
    group.throughput(Throughput::Elements(rows as u64));

    for &threads in &[1usize, 2usize, 4usize, 8usize] {
        group.bench_with_input(BenchmarkId::from_parameter(threads), &threads, |b, &t| {
            b.iter(|| run_case(&input_data, 64 * 1024, 64 * 1024, t));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_buffer_sizes, bench_thread_counts);
criterion_main!(benches);

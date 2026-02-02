use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long = "random-delay", default_value_t = 0, help = "Max random sleep per item in ms")]
    random_delay_ms: u64,
    #[arg(long = "max-size", default_value_t = 1_000_000_000, help = "Maximum size to include in tests")]
    max_size: usize,
}
static RNG_STATE: AtomicU64 = AtomicU64::new(0x9E3779B97F4A7C15);

fn compute_random_delay_ms(max_delay_ms: u64) -> u64 {
    if max_delay_ms == 0 {
        return 0;
    }

    let mut x = RNG_STATE.fetch_add(0x9E3779B97F4A7C15, Ordering::Relaxed);
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    let val = x.wrapping_mul(0x2545F4914F6CDD1D);
    val % (max_delay_ms + 1)
}

fn sequential_sum(size: usize, max_delay_ms: u64) -> (i128, Duration) {
    let start = Instant::now();
    let sum: i128 = (1..=size)
        .map(|x| {
            let delay_ms = compute_random_delay_ms(max_delay_ms);
            if delay_ms > 0 {
                thread::sleep(Duration::from_millis(delay_ms));
            }
            let x = x as i128;
            x * x
        })
        .sum();
    let duration = start.elapsed();
    (sum, duration)
}

fn parallel_sum(size: usize, max_delay_ms: u64) -> (i128, Duration) {
    let start = Instant::now();
    let sum: i128 = (1..=size)
        .into_par_iter()
        .map(|x| {
            let delay_ms = compute_random_delay_ms(max_delay_ms);
            if delay_ms > 0 {
                thread::sleep(Duration::from_millis(delay_ms));
            }
            let x = x as i128;
            x * x
        })
        .sum();
    let duration = start.elapsed();
    (sum, duration)
}

fn parallel_sum_with_threads(size: usize, threads: usize, max_delay_ms: u64) -> (i128, Duration) {
    let pool = ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("Failed to build Rayon thread pool");

    pool.install(|| parallel_sum(size, max_delay_ms))
}

fn main() {
    let args = Args::parse();
    let max_delay_ms = args.random_delay_ms;
    let max_size = args.max_size;

    let sizes = [3_usize, 1_000, 100_000, 1_000_000, 1_000_000_000];
    let max_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    let mut thread_counts = vec![1, 2, 4, max_threads];
    thread_counts.sort_unstable();
    thread_counts.dedup();
    thread_counts.retain(|&t| t <= max_threads);

    for size in sizes.into_iter().filter(|&size| size <= max_size) {
        let (sequential_sum, sequential_duration) = sequential_sum(size, max_delay_ms);

        println!("\nSize: {}", size);
        println!("Sleep delay (ms): {}", max_delay_ms);
        println!("Sequential sum: {}", sequential_sum);
        println!("Sequential duration: {:?}", sequential_duration);

        for &threads in &thread_counts {
            let (parallel_sum, parallel_duration) = parallel_sum_with_threads(size, threads, max_delay_ms);
            let speedup = sequential_duration.as_secs_f64() / parallel_duration.as_secs_f64();

            println!("Parallel sum (threads={}): {}", threads, parallel_sum);
            println!("Parallel duration (threads={}): {:?}", threads, parallel_duration);
            println!("Speedup (seq/par, threads={}): {:.3}x", threads, speedup);
        }
    }
}
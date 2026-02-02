use rayon::prelude::*;
use std::time::Instant;

fn heavy_compute(x: u64) -> u64 {
    (0..1000).fold(x, |acc, i| acc.wrapping_mul(31).wrapping_add(i))
}

fn main() {
    let data: Vec<u64> = (0..200_000).collect();

    let start = Instant::now();
    let serial_sum: u64 = data
        .iter()
        .fold(0u64, |acc, &x| acc.wrapping_add(heavy_compute(x)));
    let serial_elapsed = start.elapsed();

    let start = Instant::now();
    let parallel_sum: u64 = data
        .par_iter()
        .map(|&x| heavy_compute(x))
        .reduce(|| 0u64, |acc, x| acc.wrapping_add(x));
    let parallel_elapsed = start.elapsed();

    println!("serial sum   = {serial_sum} in {:?}", serial_elapsed);
    println!("parallel sum = {parallel_sum} in {:?}", parallel_elapsed);
}

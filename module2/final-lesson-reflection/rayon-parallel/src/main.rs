use rayon::prelude::*;

fn main() {
    let data: Vec<u64> = (0..1_000_000).collect();

    let sum: u64 = data
        .par_iter()
        .map(|x| x.wrapping_mul(2))
        .filter(|x| x % 3 == 0)
        .sum();

    println!("parallel sum = {sum}");
}

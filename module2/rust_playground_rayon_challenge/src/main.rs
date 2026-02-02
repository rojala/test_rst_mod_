use rayon::prelude::*;

fn main() {
    let data = vec![1, 2, 3];

    let parallel_sum: i32 = data.par_iter() // Specify the type
        .map(|x| x * x)
        .sum();

    println!("Parallel sum: {}", parallel_sum);
}
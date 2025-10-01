/*
This example code counts the frequency of each number in the vector.
 */
use std::collections::HashMap;

// 1. Use clap  modify the program to accept input from the
// user and then calculate the frequency of each integer.
use clap::{Arg, Command};

fn main_with_clap() -> Vec<(i32, u32)> {
    let matches = Command::new("counting-frequency")
        .version("1.0")
        .author("Author Name")
        .about("Counts the frequency of each integer in a list")
        .arg(
            Arg::new("numbers")
                .help("List of integers to count frequency")
                .required(false)
                .num_args(1..),
        )
        .get_matches();

    let numbers: Vec<i32> = matches
        .get_many::<String>("numbers")
        .unwrap()
        .map(|s| s.parse().unwrap())
        .collect();

    let _result: Vec<(i32, u32)> = logic(numbers);
    _result
}

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let result = logic(numbers);
    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );

    let _cli_result = main_with_clap();
    // Print the results from the CLI
    println!("The frequency of each number from CLI input is: {:?}", _cli_result);
}

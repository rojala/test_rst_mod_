/*
This example code counts the frequency of each number in the vector.
 */
use std::collections::HashMap;

// 1. Use clap  modify the program to accept input from the
// user and then calculate the frequency of each integer.
// 2.  Extend this concept to count the frequency of words in a sentence.

use clap::{Arg, Command};

fn main_with_clap() -> Vec<(String, u32)> {
    let matches = Command::new("counting-frequency")
        .version("1.0")
        .author("Author Name")
        .about("Counts the frequency of each integer/word in a list")
        .arg(
            Arg::new("numbers or words")
                .help("List of integers or words to count frequency")
                .required(false)
                .num_args(1..),
        )
        .get_matches();

    let numbers: Vec<String> = if let Some(nums) = matches.get_many::<String>("numbers or words") {
        nums.map(|s| s.to_string()).collect()
    } else {
        vec![ ]
    };

    let _result: Vec<(String, u32)> = logic_word(numbers);
    _result
}

fn logic_word(numbers_or_words: Vec<String>) -> Vec<(String, u32)> {
    let mut frequencies = HashMap::new();

    for item in numbers_or_words {
        let frequency = frequencies.entry(item).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (item, frequency) in frequencies {
        result.push((item, frequency));
    }

    result
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

// 3. order Vec(i32, u32) by frequency = value not key
fn order_by_frequency_string(vec: Vec<(String, u32)>) -> Vec<(String, u32)> {
    let mut vec = vec;
    vec.sort_by(|a, b| b.1.cmp(&a.1));
    vec
}

// 3. order Vec(String, u32) by frequency = value not key
fn order_by_frequency_int(vec: Vec<(i32, u32)>) -> Vec<(i32, u32)> {
    let mut vec = vec;
    vec.sort_by(|a, b| b.1.cmp(&a.1));
    vec
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let result = logic(numbers);
    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );
    
    // 3. order
    println!(
        "Ordered The frequency of each number in the vector is: {:?}",
        order_by_frequency_int(result)
    );

    let _cli_result = main_with_clap();
    // Print the results from the CLI
    println!(
        "The frequency of each number/word from CLI input is: {:?}",
        _cli_result
    );

    // 3. ordered from CLI
    println!(
        "Ordered frequency of each number/word from CLI input is: {:?}",
        order_by_frequency_string(_cli_result)
    );
}

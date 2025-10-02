use chrono::Datelike;
use std::collections::HashMap;
use clap::{Arg, Command};

fn init_languages() -> HashMap<String, i32> {
    let mut languages = HashMap::new();
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("HTML/CSS".to_string(), 1990);
    languages.insert("Python".to_string(), 1991);
    languages.insert("SQL".to_string(), 1974);
    languages.insert("TypeScript".to_string(), 2012);
    languages.insert("Bash/Shell".to_string(), 1989);
    languages.insert("Java".to_string(), 1995);
    languages.insert("C#".to_string(), 2000);
    languages.insert("C++".to_string(), 1985);
    languages.insert("C".to_string(), 1972);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("PowerShell".to_string(), 2006);
    languages.insert("Go".to_string(), 2007);
    languages.insert("Rust".to_string(), 2010);

    languages
}

fn calculate_weights(years_active: &mut HashMap<String, i32>) -> HashMap<String, i32> {
    // Subtract the creation year from present to get the number of years active.
    for year in years_active.values_mut() {
        // Get current year
        let current_year = chrono::Utc::now().year();
        *year = current_year - *year;
    }

    let min_year = *years_active.values().min().unwrap_or(&0);
    let max_year = *years_active.values().max().unwrap_or(&0);

    let mut weights = HashMap::new();

    for (language, &year) in years_active.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as i32 + 1; // weight between 1 and 100
        weights.insert(language.to_string(), weight);
    }

    weights
}

// 1. Modify the program to accept new languages from the user,
//    along with the year they were created, and then include them in the weight calculation?
//    If argument is given both language and year must be provided.
//    allow multiple languages to be added at once.

fn main_check_cli() -> HashMap<String, i32> {
    let matches = Command::new("language_weight")
        .arg(
            Arg::new("language")
                .help("Programming language name")
                .long("language")
                .short('l')
                .num_args(1..)
                .action(clap::ArgAction::Append),
        )
        .arg(
            Arg::new("year")
                .help("Year the language was created")
                .long("year")
                .short('y')
                .num_args(1..)
                .action(clap::ArgAction::Append),
        )
        .get_matches();

    let languages: Vec<&String> = matches.get_many::<String>("language").unwrap_or_default().collect();
    let years: Vec<&String> = matches.get_many::<String>("year").unwrap_or_default().collect();

    if languages.len() != years.len() {
        eprintln!("Error: The number of --language and --year arguments must match.");
        std::process::exit(1);
    }

    let mut language_years: HashMap<String, i32> = HashMap::new();

    for (lang, yr_str) in languages.iter().zip(years.iter()) {
        match yr_str.parse::<i32>() {
            Ok(year) => {
                language_years.insert(lang.to_string(), year);
            }
            Err(_) => {
                eprintln!("Invalid year format for '{}': {}", lang, yr_str);
                std::process::exit(1);
            }
        }
    }

    println!("CLI collected language-year pairs:");
    for (lang, year) in &language_years {
        println!("{}: {}", lang, year);
    }

    language_years
}

fn main() {
    // Get languages and years from CLI
    let cli_language_years = main_check_cli();

    let mut languages = init_languages();

    // Extend the initial languages with those provided via CLI
    // Check if the language already exists, if so, update the year
    // and give warnig.
    for (lang, year) in &cli_language_years {
        if languages.contains_key(lang) {
            eprintln!("Warning: Language '{}' already exists with year {}. Updating to new year {}.", lang, languages[lang], year);
        }
        languages.insert(lang.to_string(), *year);
    }

    let weights = calculate_weights(&mut languages);

    println!("Language weighing from 1-100 by age (1 is newest and 100 is oldest):");
    for (language, weight) in &weights {
        println!("{}: {}", language, weight);
    }
}

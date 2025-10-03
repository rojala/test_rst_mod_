use chrono::Datelike;
use clap::{Arg, Command};
use std::collections::HashMap;

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

// 3. init growth rates for each language
fn init_growth_rates() -> HashMap<String, f64> {
    let mut growth_rates = HashMap::new();
    growth_rates.insert("JavaScript".to_string(), 0.05);
    growth_rates.insert("HTML/CSS".to_string(), 0.02);
    growth_rates.insert("Python".to_string(), 0.10);
    growth_rates.insert("SQL".to_string(), 0.01);
    growth_rates.insert("TypeScript".to_string(), 0.15);
    growth_rates.insert("Bash/Shell".to_string(), 0.01);
    growth_rates.insert("Java".to_string(), 0.03);
    growth_rates.insert("C#".to_string(), 0.02);
    growth_rates.insert("C++".to_string(), 0.01);
    growth_rates.insert("C".to_string(), 0.005);
    growth_rates.insert("PHP".to_string(), -0.01);
    growth_rates.insert("PowerShell".to_string(), 0.02);
    growth_rates.insert("Go".to_string(), 0.08);
    growth_rates.insert("Rust".to_string(), 0.12);

    growth_rates
}

// 3. init user counts for each language
fn init_user_counts() -> HashMap<String, i32> {
    let mut user_counts = HashMap::new();
    user_counts.insert("JavaScript".to_string(), 12000000);
    user_counts.insert("HTML/CSS".to_string(), 10000000);
    user_counts.insert("Python".to_string(), 8000000);
    user_counts.insert("SQL".to_string(), 6000000);
    user_counts.insert("TypeScript".to_string(), 4000000);
    user_counts.insert("Bash/Shell".to_string(), 3000000);
    user_counts.insert("Java".to_string(), 7000000);
    user_counts.insert("C#".to_string(), 5000000);
    user_counts.insert("C++".to_string(), 4000000);
    user_counts.insert("C".to_string(), 3500000);
    user_counts.insert("PHP".to_string(), 4500000);
    user_counts.insert("PowerShell".to_string(), 2000000);
    user_counts.insert("Go".to_string(), 2500000);
    user_counts.insert("Rust".to_string(), 1500000);
    user_counts
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
/* 

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

    let languages: Vec<&String> = matches
        .get_many::<String>("language")
        .unwrap_or_default()
        .collect();
    let years: Vec<&String> = matches
        .get_many::<String>("year")
        .unwrap_or_default()
        .collect();

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

*/

// 1. Modify the program to accept new languages from the user,
//    along with the year they were created, and then include them in the weight calculation?
//    If argument is given both language and year must be provided.
//    allow multiple languages to be added at once.

fn main_check_cli_updated() -> (HashMap<String, i32>, HashMap<String, i32>, HashMap<String, f64>) {
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
        .arg(
            Arg::new("users")
                .help("Number of users of the language")
                .long("users")
                .short('u')
                .num_args(1..)
                .action(clap::ArgAction::Append),
        )
        .arg(
            Arg::new("growth")
                .help("Growth rate of the language")
                .long("growth")
                .short('g')
                .num_args(1..)
                .action(clap::ArgAction::Append),
        )
        .get_matches();

    let languages: Vec<&String> = matches
        .get_many::<String>("language")
        .unwrap_or_default()
        .collect();
    let years: Vec<&String> = matches
        .get_many::<String>("year")
        .unwrap_or_default()
        .collect();
    let users: Vec<&String> = matches
        .get_many::<String>("users")
        .unwrap_or_default()
        .collect();
    let growths: Vec<&String> = matches
        .get_many::<String>("growth")
        .unwrap_or_default()
        .collect();

    if languages.len() != years.len() || languages.len() != users.len() || languages.len() != growths.len() {
        eprintln!("Error: The number of --language, --year, --users, and --growth arguments must match.");
        std::process::exit(1);
    }

    // Return tuble of hashmaps where first is language-year, second is language-users and third is language-growth

    let mut language_years: HashMap<String, i32> = HashMap::new();
    let mut language_users: HashMap<String, i32> = HashMap::new();
    let mut language_growths: HashMap<String, f64> = HashMap::new();

    for i in 0..languages.len() {
        let lang = languages[i];
        let yr_str = years[i];
        let user_str = users[i];
        let growth_str = growths[i];

        match yr_str.parse::<i32>() {
            Ok(year) => {
                language_years.insert(lang.to_string(), year);
            }
            Err(_) => {
                eprintln!("Invalid year format for '{}': {}", lang, yr_str);
                std::process::exit(1);
            }
        }

        match user_str.parse::<i32>() {
            Ok(users) => {
                language_users.insert(lang.to_string(), users);
            }
            Err(_) => {
                eprintln!("Invalid users format for '{}': {}", lang, user_str);
                std::process::exit(1);
            }
        }

        match growth_str.parse::<f64>() {
            Ok(growth) => {
                language_growths.insert(lang.to_string(), growth);
            }
            Err(_) => {
                eprintln!("Invalid growth format for '{}': {}", lang, growth_str);
                std::process::exit(1);
            }
        }
    }

    println!("CLI collected language-year pairs:");
    for (lang, year) in &language_years {
        println!("{}: {}", lang, year);
    }

    println!("CLI collected language-users pairs:");
    for (lang, users) in &language_users {
        println!("{}: {}", lang, users);
    }

    println!("CLI collected language-growth pairs:");
    for (lang, growth) in &language_growths {
        println!("{}: {}", lang, growth);
    }

    (language_years, language_users, language_growths)
}

// 2. How would you modify the program to sort the languages by their weights?
fn sort_by_weight(weights: &HashMap<String, i32>) -> Vec<(String, i32)> {
    let mut weight_vec: Vec<(String, i32)> = weights.iter().map(|(k, &v)| (k.clone(), v)).collect();
    weight_vec.sort_by(|a, b| a.1.cmp(&b.1));
    weight_vec
}


// 3. Can you extend the weight calculation to take into account other factors,
//    such as the number of users or the growth rate of each language?
fn extend_weight_calculation(weights: &mut HashMap<String, i32>, user_counts: &HashMap<String, i32>, growth_rates: &HashMap<String, f64>) {
    for (language, weight) in weights.iter_mut() {
        if let Some(&user_count) = user_counts.get(language) {
            // Example: Decrease weight by 1 for every 1000 users (capped at 20)
            let user_weight_adjustment = (user_count / 1000).min(20);
            *weight -= user_weight_adjustment;
        }
        if let Some(&growth_rate) = growth_rates.get(language) {
            // Example: Decrease weight by growth rate percentage (capped at 10)
            let growth_weight_adjustment = (growth_rate * 10.0).min(10.0) as i32;
            *weight -= growth_weight_adjustment;
        }
        // Ensure weight stays within bounds
        if *weight < 1 {
            *weight = 1;
        } else if *weight > 100 {
            *weight = 100;
        }
    }
}

fn main() {
    // Get languages and years from CLI
    //let cli_language_years = main_check_cli();
    // 3. Include user counts and growth rates from CLI
    let (cli_language_years, cli_user_counts, cli_growth_rates) = main_check_cli_updated();

    let mut languages = init_languages();

    // 3. Initialize user counts and growth rates
    let user_counts = init_user_counts();
    let growth_rates = init_growth_rates();

    // Extend the initial languages with those provided via CLI
    // Check if the language already exists, if so, update the year
    // and give warnig.
    for (lang, year) in &cli_language_years {
        if languages.contains_key(lang) {
            eprintln!(
                "Warning: Language '{}' already exists with year {}. Updating to new year {}.",
                lang, languages[lang], year
            );
        }
        languages.insert(lang.to_string(), *year);
    }

    // 3. Extend user counts and growth rates with those provided via CLI
    let mut user_counts = user_counts;
    let mut growth_rates = growth_rates;
    for (lang, users) in &cli_user_counts {
        if user_counts.contains_key(lang) {
            eprintln!(
                "Warning: Language '{}' already exists with user count {}. Updating to new user count {}.",
                lang, user_counts[lang], users
            );
        }
        user_counts.insert(lang.to_string(), *users);
    }
    for (lang, growth) in &cli_growth_rates {
        if growth_rates.contains_key(lang) {
            eprintln!(
                "Warning: Language '{}' already exists with growth rate {:.2}. Updating to new growth rate {:.2}.",
                lang, growth_rates[lang], growth
            );
        }
        growth_rates.insert(lang.to_string(), *growth);
    }

    let mut weights = calculate_weights(&mut languages);

    // Print delimiter for clarity
    println!("----------------------------------------");

    // Print original without extra factors
    println!("Language weighing with from 1-100 by age (1 is newest and 100 is oldest):");
    for (language, weight) in &weights {
        println!("{}: {}", language, weight);
    }

    // Print delimiter for clarity
    println!("----------------------------------------");

    // Print original sorted by weight
    println!("\n");
    println!("Sorted language by weighing from 1-100 by age (1 is newest and 100 is oldest):");
    for (language, weight) in sort_by_weight(&weights) {
        println!("{}: {}", language, weight);
    }

    // Print delimiter for clarity
    println!("\n----------------------------------------");
    println!("----------------------------------------\n");
    //3. calculate weights with extended factors
    extend_weight_calculation(&mut weights, &user_counts, &growth_rates);

    println!("Language weighing with user counts and growth rates from 1-100 scale:");
    // Add 1. Print the weights for each language and its year of creation. As well user_counts and growth_rates.
    for (language, weight) in &weights {
        let user_count = user_counts.get(language).unwrap_or(&0);
        let growth_rate = growth_rates.get(language).unwrap_or(&0.0);
        println!("{}: {} (Users: {}, Growth Rate: {:.2}%)", language, weight, user_count, growth_rate);
    }

    println!("----------------------------------------");

    // 2. Sorted printing by weight
    println!("\n");
    println!("Sorted language by weighing from 1-100 scale most weighted first:");
    for (language, weight) in sort_by_weight(&weights).into_iter().rev() {
        let user_count = user_counts.get(&language).unwrap_or(&0);
        let growth_rate = growth_rates.get(&language).unwrap_or(&0.0);
        println!("{}: {} (Users: {}, Growth Rate: {:.2}%)", language, weight, user_count, growth_rate);
    }
    println!("----------------------------------------");
}

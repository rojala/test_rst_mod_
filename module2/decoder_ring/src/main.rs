/*
Attempts to statistically decode a Caesar cipher.
Here's an example of how to use it:

This is a shift 16 message: "Off to the bunker. Every person for themselves"
"Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc"

Command-line input:
cargo run -- --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --guess

File input:
cargo run -- --file encrypted.txt --guess

With optimization flag:
cargo run -- --message "Ypp dy dro lexuob" --guess --optimize chi_squared
cargo run -- --message "Ypp dy dro lexuob" --guess --optimize bigram
cargo run -- --message "Ypp dy dro lexuob" --guess --optimize weighted

Optimization options: basic (default), chi_squared, bigram, weighted
*/

use clap::Parser;
use decoder_ring::print_stats_analysis;
use std::fs;
use std::time::Instant;

/// CLI tool to reverse engineer a Caesar cipher with optimization options
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The message to decrypt (via command-line)
    #[arg(short, long)]
    message: Option<String>,

    /// Path to file containing the encrypted message
    #[arg(short, long)]
    file: Option<String>,

    /// Display statistical information about the message
    #[arg(short, long)]
    stats: bool,

    /// Guess the shift using frequency analysis
    #[arg(short, long)]
    guess: bool,

    /// Optimization strategy: basic, chi_squared, bigram, or weighted
    #[arg(short, long, default_value = "basic")]
    optimize: String,
}

fn main() {
    let args = Args::parse();

    // Get the message from either command-line or file input
    // Both sources are optional, but exactly one must be provided
    let message = match (&args.message, &args.file) {
        // If message is provided via --message flag, use it directly
        (Some(msg), None) => msg.clone(),

        // If file path is provided via --file flag, read the file contents
        (None, Some(path)) => {
            match fs::read_to_string(path) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", path, e);
                    return;
                }
            }
        }

        // Error: both message and file provided
        (Some(_), Some(_)) => {
            eprintln!("Error: Provide either --message or --file, not both");
            return;
        }

        // Error: neither message nor file provided
        (None, None) => {
            eprintln!("Error: Either --message or --file is required");
            return;
        }
    };

    // Validate optimization choice
    let valid_optimizations = ["basic", "chi_squared", "bigram", "weighted"];
    if !valid_optimizations.contains(&args.optimize.as_str()) {
        eprintln!(
            "Error: Invalid optimization '{}'. Choose from: {}",
            args.optimize,
            valid_optimizations.join(", ")
        );
        return;
    }

    // Display statistical analysis if --stats flag is set
    if args.stats {
        print_stats_analysis(&message);
    }

    // Attempt to guess the shift and decrypt if --guess flag is set
    if args.guess {
        println!("\n=== Decryption with {} optimization ===", args.optimize);
        
        // Start timing
        let start_time = Instant::now();

        // Run the optimized guess_shift
        let (depth, best_shift, decrypted, max_score) =
            decoder_ring::guess_shift_optimized(&message, 26, &args.optimize);

        // Stop timing
        let elapsed = start_time.elapsed();

        println!(
            "\nBest shift: {} (out of {}), score: {:.2}",
            best_shift, depth, max_score
        );
        println!("Decrypted message: {}", decrypted);
        println!(
            "Time elapsed: {:.4} seconds ({} milliseconds)",
            elapsed.as_secs_f32(),
            elapsed.as_millis()
        );
    }
}
use sha3::{Sha3_256, Digest};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use walkdir::WalkDir;
use clap::Parser;

#[derive(Parser)]
#[command(name = "duplicate-file-finder")]
#[command(about = "Find duplicate files using SHA-3 cryptographic hashes")]
#[command(version = "1.0")]
struct Args {
    /// Directory to scan for duplicates
    #[arg(short, long, default_value = ".")]
    directory: PathBuf,
    
    /// Minimum file size to check (in bytes)
    #[arg(short, long, default_value = "0")]
    min_size: u64,
    
    /// Show progress while scanning
    #[arg(short, long)]
    verbose: bool,
}

fn hash_file(path: &PathBuf) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha3_256::new();
    let mut buffer = [0u8; 8192];
    
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    
    Ok(format!("{:x}", hasher.finalize()))
}

fn find_duplicates(directory: &PathBuf, min_size: u64, verbose: bool) -> HashMap<String, Vec<PathBuf>> {
    let mut hash_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut files_processed = 0;
    
    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let path = entry.path().to_path_buf();
        
        // Skip small files if requested
        if let Ok(metadata) = entry.metadata() {
            if metadata.len() < min_size {
                continue;
            }
        }
        
        if verbose {
            println!("Processing: {:?}", path);
        }
        
        match hash_file(&path) {
            Ok(hash) => {
                hash_map.entry(hash).or_insert_with(Vec::new).push(path);
                files_processed += 1;
            }
            Err(e) => {
                eprintln!("Error hashing {:?}: {}", path, e);
            }
        }
    }
    
    if verbose {
        println!("\nProcessed {} files", files_processed);
    }
    
    // Filter to only keep hashes with multiple files (duplicates)
    hash_map.into_iter()
        .filter(|(_, files)| files.len() > 1)
        .collect()
}

fn main() {
    let args = Args::parse();
    
    println!("=== Duplicate File Finder ===");
    println!("Scanning directory: {:?}", args.directory.canonicalize().unwrap_or(args.directory.clone()));
    println!("Minimum file size: {} bytes", args.min_size);
    println!();
    
    let duplicates = find_duplicates(&args.directory, args.min_size, args.verbose);
    
    if duplicates.is_empty() {
        println!("✓ No duplicate files found.");
    } else {
        println!("Found {} groups of duplicate files:\n", duplicates.len());
        
        for (i, (hash, files)) in duplicates.iter().enumerate() {
            println!("Group {} - Hash: {}", i + 1, &hash[..16]);
            
            // Calculate total wasted space
            let file_size = std::fs::metadata(&files[0])
                .map(|m| m.len())
                .unwrap_or(0);
            let wasted = file_size * (files.len() as u64 - 1);
            
            println!("  File size: {} bytes ({:.2} KB)", file_size, file_size as f64 / 1024.0);
            println!("  Copies: {}", files.len());
            println!("  Wasted space: {} bytes ({:.2} KB)", wasted, wasted as f64 / 1024.0);
            println!("  Files:");
            
            for file in files {
                println!("    - {:?}", file);
            }
            println!();
        }
        
        // Calculate total statistics
        let total_groups = duplicates.len();
        let total_files: usize = duplicates.values().map(|v| v.len()).sum();
        let total_wasted: u64 = duplicates.iter()
            .map(|(_, files)| {
                let size = std::fs::metadata(&files[0]).map(|m| m.len()).unwrap_or(0);
                size * (files.len() as u64 - 1)
            })
            .sum();
        
        println!("=== Summary ===");
        println!("Duplicate groups: {}", total_groups);
        println!("Total duplicate files: {}", total_files);
        println!("Total wasted space: {} bytes", total_wasted);
        println!("Total wasted space: {:.2} KB", total_wasted as f64 / 1024.0);
        println!("Total wasted space: {:.2} MB", total_wasted as f64 / 1_048_576.0);
    }
}

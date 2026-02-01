use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread;

#[derive(Debug, Clone)]
struct DirStats {
    total_size: u64,
    file_count: usize,
    dir_count: usize,
}

impl DirStats {
    fn new() -> Self {
        DirStats { 
            total_size: 0, 
            file_count: 0,
            dir_count: 0,
        }
    }
}

fn traverse_directory(path: PathBuf, tx: Sender<PathBuf>, stats: Arc<Mutex<DirStats>>) {
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries.flatten() {
            let path = entry.path();
            
            if path.is_file() {
                if let Ok(metadata) = fs::metadata(&path) {
                    let mut stats = stats.lock().unwrap();
                    stats.total_size += metadata.len();
                    stats.file_count += 1;
                }
            } else if path.is_dir() {
                {
                    let mut stats = stats.lock().unwrap();
                    stats.dir_count += 1;
                }
                // Send subdirectory to be processed by another thread
                tx.send(path.clone()).ok();
            }
        }
    }
}

fn main() {
    // Change this to the directory you want to scan
    let root = PathBuf::from(".");
    
    println!("Multi-threaded Directory Traversal");
    println!("Scanning: {:?}\n", root.canonicalize().unwrap_or(root.clone()));
    
    let (tx, rx) = channel();
    let stats = Arc::new(Mutex::new(DirStats::new()));
    
    // Start with root directory
    tx.send(root.clone()).unwrap();
    
    let num_threads = 4;
    let mut handles = vec![];
    
    println!("Starting {} worker threads...\n", num_threads);
    
    for i in 0..num_threads {
        let rx = rx.clone();
        let tx = tx.clone();
        let stats = Arc::clone(&stats);
        
        let handle = thread::spawn(move || {
            let mut processed = 0;
            while let Ok(dir) = rx.recv() {
                traverse_directory(dir, tx.clone(), Arc::clone(&stats));
                processed += 1;
            }
            println!("Thread {} processed {} directories", i, processed);
        });
        handles.push(handle);
    }
    
    drop(tx); // Drop original sender so threads can exit when queue is empty
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_stats = stats.lock().unwrap();
    
    println!("\n=== Results ===");
    println!("Directories: {}", final_stats.dir_count);
    println!("Files: {}", final_stats.file_count);
    println!("Total size: {} bytes", final_stats.total_size);
    println!("Total size: {:.2} KB", final_stats.total_size as f64 / 1_024.0);
    println!("Total size: {:.2} MB", final_stats.total_size as f64 / 1_048_576.0);
    
    if final_stats.file_count > 0 {
        println!("Average file size: {:.2} KB", 
            (final_stats.total_size as f64 / final_stats.file_count as f64) / 1_024.0);
    }
}

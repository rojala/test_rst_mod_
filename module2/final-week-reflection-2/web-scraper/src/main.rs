use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct RateLimiter {
    max_concurrent: usize,
    current: Arc<Mutex<usize>>,
}

impl RateLimiter {
    fn new(max_concurrent: usize) -> Self {
        RateLimiter {
            max_concurrent,
            current: Arc::new(Mutex::new(0)),
        }
    }
    
    fn acquire(&self) {
        loop {
            let mut count = self.current.lock().unwrap();
            if *count < self.max_concurrent {
                *count += 1;
                break;
            }
            drop(count); // Release lock before sleeping
            thread::sleep(Duration::from_millis(100));
        }
    }
    
    fn release(&self) {
        let mut count = self.current.lock().unwrap();
        *count -= 1;
    }
}

fn scrape_url(url: &str, limiter: Arc<RateLimiter>) {
    limiter.acquire();
    
    println!("Starting scrape: {}", url);
    
    // Simulated HTTP request (replace with reqwest in real implementation)
    // For real implementation:
    // let client = reqwest::blocking::Client::new();
    // let response = client.get(url).send()?;
    // let body = response.text()?;
    
    thread::sleep(Duration::from_secs(1)); // Simulate network delay
    
    // Process the scraped content
    println!("Completed scrape: {}", url);
    
    limiter.release();
}

fn main() {
    let urls = vec![
        "https://example.com/page1",
        "https://example.com/page2",
        "https://example.com/page3",
        "https://example.com/page4",
        "https://example.com/page5",
        "https://example.com/page6",
        "https://example.com/page7",
        "https://example.com/page8",
    ];
    
    let max_concurrent = 3; // Politeness: max 3 concurrent requests
    println!("Web Scraper with Rate Limiting");
    println!("Scraping {} URLs with max {} concurrent requests\n", 
        urls.len(), max_concurrent);
    
    let limiter = Arc::new(RateLimiter::new(max_concurrent));
    
    let handles: Vec<_> = urls.into_iter().map(|url| {
        let limiter = Arc::clone(&limiter);
        let url = url.to_string();
        thread::spawn(move || scrape_url(&url, limiter))
    }).collect();
    
    // Wait for all scraping threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("\nAll URLs scraped successfully!");
}

use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::time::Duration;

// Messages that philosophers can send
enum ForkRequest {
    Acquire(usize, Sender<bool>), // philosopher_id, response channel
    Release(usize),
}

fn fork_manager(rx: Receiver<ForkRequest>, num_forks: usize) {
    let mut forks_available = vec![true; num_forks];
    
    while let Ok(msg) = rx.recv() {
        match msg {
            ForkRequest::Acquire(fork_id, response) => {
                if forks_available[fork_id] {
                    forks_available[fork_id] = false;
                    response.send(true).unwrap();
                } else {
                    response.send(false).unwrap();
                }
            }
            ForkRequest::Release(fork_id) => {
                forks_available[fork_id] = true;
            }
        }
    }
}

fn philosopher(name: &str, id: usize, left: usize, right: usize, fork_tx: Sender<ForkRequest>) {
    for meal in 0..5 {
        // Think
        println!("{} is thinking...", name);
        thread::sleep(Duration::from_millis(100));
        
        // Try to acquire forks using channels
        println!("{} is hungry (meal {})", name, meal + 1);
        loop {
            let (tx, rx) = channel();
            fork_tx.send(ForkRequest::Acquire(left, tx.clone())).unwrap();
            if rx.recv().unwrap() {
                println!("{} picked up left fork {}", name, left);
                fork_tx.send(ForkRequest::Acquire(right, tx)).unwrap();
                if rx.recv().unwrap() {
                    println!("{} picked up right fork {}", name, right);
                    break; // Got both forks
                } else {
                    // Release left fork if can't get right
                    println!("{} couldn't get right fork, releasing left", name);
                    fork_tx.send(ForkRequest::Release(left)).unwrap();
                    thread::sleep(Duration::from_millis(10));
                }
            } else {
                thread::sleep(Duration::from_millis(10));
            }
        }
        
        // Eat
        println!("{} is eating (meal {})", name, meal + 1);
        thread::sleep(Duration::from_millis(200));
        
        // Release forks
        fork_tx.send(ForkRequest::Release(left)).unwrap();
        fork_tx.send(ForkRequest::Release(right)).unwrap();
        println!("{} finished eating and put down forks", name);
    }
    
    println!("{} is done dining!", name);
}

fn main() {
    let num_philosophers = 5;
    let num_forks = 5;
    
    println!("Dining Philosophers Problem with Channels");
    println!("{} philosophers, {} forks\n", num_philosophers, num_forks);
    
    let (fork_tx, fork_rx) = channel();
    
    // Start fork manager thread
    let manager_tx = fork_tx.clone();
    thread::spawn(move || {
        fork_manager(fork_rx, num_forks);
    });
    
    let philosophers = vec![
        "Aristotle",
        "Plato",
        "Socrates",
        "Descartes",
        "Kant",
    ];
    
    let handles: Vec<_> = philosophers.into_iter().enumerate().map(|(i, name)| {
        let left = i;
        let right = (i + 1) % num_forks;
        let tx = fork_tx.clone();
        let name = name.to_string();
        
        thread::spawn(move || {
            philosopher(&name, i, left, right, tx);
        })
    }).collect();
    
    // Wait for all philosophers to finish
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("\nAll philosophers finished dining!");
}

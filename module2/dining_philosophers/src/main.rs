/*
* Enhanced dining philosophers problem with dynamic runtime additions.
* Allows adding philosophers or forks at specified times during execution.
*/

use clap::Parser;
use rand::seq::SliceRandom;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(name = "Dining Philosophers")]
#[command(about = "Simulates the dining philosophers concurrency problem", long_about = None)]
struct Args {
    /// Number of philosophers
    #[arg(short, long, default_value_t = 15)]
    philosophers: usize,

    /// Number of forks
    #[arg(short, long, default_value_t = 4)]
    forks: usize,

    /// Add philosophers after N seconds (e.g., "3:2,5:1" adds 2 after 3s, 1 after 5s)
    #[arg(long, value_name = "TIME:COUNT,...")]
    add_philosophers: Option<String>,

    /// Add forks after N seconds (e.g., "3:2,5:1" adds 2 after 3s, 1 after 5s)
    #[arg(long, value_name = "TIME:COUNT,...")]
    add_forks: Option<String>,

    /// Runtime duration in seconds (default: auto-calculated based on scheduled additions)
    #[arg(long)]
    runtime: Option<u64>,

    /// Enable priority system based on hunger level
    #[arg(long)]
    priority: bool,

    /// Display hunger statistics every N seconds (requires --priority)
    #[arg(long, value_name = "SECONDS")]
    stats: Option<u64>,
}

struct Fork {
    id: u32,
    mutex: Mutex<()>,
    priority_queue: Mutex<Vec<(u32, Instant)>>, // (philosopher_id, wait_start_time)
}

#[derive(Clone)]
struct HungerStats {
    id: u32,
    name: String,
    times_eaten: Arc<Mutex<u64>>,
    total_wait_time: Arc<Mutex<Duration>>,
    last_meal_time: Arc<Mutex<Option<Instant>>>,
    current_hunger: Arc<Mutex<Duration>>,
}

struct Philosopher {
    id: u32,
    name: String,
    left_fork: Arc<Fork>,
    right_fork: Arc<Fork>,
    running: Arc<RwLock<bool>>,
    use_priority: bool,
    stats: Option<HungerStats>,
}

impl Philosopher {
    fn new(
        id: u32,
        name: &str,
        left_fork: Arc<Fork>,
        right_fork: Arc<Fork>,
        running: Arc<RwLock<bool>>,
        use_priority: bool,
        stats: Option<HungerStats>,
    ) -> Philosopher {
        Philosopher {
            id,
            name: name.to_string(),
            left_fork,
            right_fork,
            running,
            use_priority,
            stats,
        }
    }

    fn try_acquire_fork_with_priority(&self, fork: &Fork, wait_start: Instant) -> bool {
        // Add self to priority queue
        {
            let mut queue = fork.priority_queue.lock().unwrap();
            queue.push((self.id, wait_start));
            // Sort by wait time (oldest first = highest priority)
            queue.sort_by_key(|(_, time)| *time);
        }

        // Check if we have highest priority
        loop {
            let queue = fork.priority_queue.lock().unwrap();
            if queue.is_empty() || queue[0].0 != self.id {
                drop(queue);
                thread::sleep(Duration::from_millis(10));
                if !*self.running.read().unwrap() {
                    return false;
                }
                continue;
            }
            drop(queue);
            break;
        }

        // Try to acquire the mutex
        if fork.mutex.try_lock().is_ok() {
            // Remove self from priority queue
            let mut queue = fork.priority_queue.lock().unwrap();
            queue.retain(|(id, _)| *id != self.id);
            true
        } else {
            thread::sleep(Duration::from_millis(10));
            false
        }
    }

    fn eat(&self) {
        while *self.running.read().unwrap() {
            let wait_start = Instant::now();

            let (first_fork, second_fork) = if self.id % 2 == 0 {
                (&self.left_fork, &self.right_fork)
            } else {
                (&self.right_fork, &self.left_fork)
            };

            let _first_guard = if self.use_priority {
                loop {
                    if self.try_acquire_fork_with_priority(first_fork, wait_start) {
                        break;
                    }
                    if !*self.running.read().unwrap() {
                        return;
                    }
                }
                first_fork.mutex.lock().ok()
            } else {
                first_fork.mutex.lock().ok()
            };

            println!("{} picked up fork {}.", self.name, first_fork.id);

            let _second_guard = if self.use_priority {
                loop {
                    if self.try_acquire_fork_with_priority(second_fork, wait_start) {
                        break;
                    }
                    if !*self.running.read().unwrap() {
                        return;
                    }
                }
                second_fork.mutex.lock().ok()
            } else {
                second_fork.mutex.lock().ok()
            };

            println!("{} picked up fork {}.", self.name, second_fork.id);

            let wait_time = wait_start.elapsed();

            // Update stats if enabled
            if let Some(ref stats) = self.stats {
                *stats.times_eaten.lock().unwrap() += 1;
                *stats.total_wait_time.lock().unwrap() += wait_time;
                *stats.last_meal_time.lock().unwrap() = Some(Instant::now());
                *stats.current_hunger.lock().unwrap() = Duration::from_secs(0);
            }

            println!(
                "{} is eating. (waited {:?})",
                self.name, wait_time
            );
            thread::sleep(Duration::from_secs(1));
            println!("{} finished eating.", self.name);

            println!("{} put down fork {}.", self.name, first_fork.id);
            println!("{} put down fork {}.", self.name, second_fork.id);

            // Small pause before trying to eat again
            thread::sleep(Duration::from_millis(100));
        }
    }
}

// Parse dynamic addition schedule from string like "3:2,5:1"
fn parse_schedule(input: &str) -> Result<Vec<(u64, usize)>, String> {
    input
        .split(',')
        .map(|pair| {
            let parts: Vec<&str> = pair.trim().split(':').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid format '{}', expected TIME:COUNT", pair));
            }
            let time = parts[0]
                .parse::<u64>()
                .map_err(|_| format!("Invalid time value '{}'", parts[0]))?;
            let count = parts[1]
                .parse::<usize>()
                .map_err(|_| format!("Invalid count value '{}'", parts[1]))?;
            Ok((time, count))
        })
        .collect()
}

fn main() {
    let args = Args::parse();

    println!(
        "Dining Philosophers Problem: {} Philosophers, {} Forks",
        args.philosophers, args.forks
    );

    if args.forks == 0 {
        println!("Error: Need at least 1 fork!");
        return;
    }

    if args.philosophers == 0 {
        println!("Error: Need at least 1 philosopher!");
        return;
    }

    // Parse dynamic addition schedules
    let philosopher_schedule = args
        .add_philosophers
        .as_ref()
        .map(|s| parse_schedule(s))
        .transpose()
        .unwrap_or_else(|e| {
            eprintln!("Error parsing --add-philosophers: {}", e);
            std::process::exit(1);
        })
        .unwrap_or_default();

    let fork_schedule = args
        .add_forks
        .as_ref()
        .map(|s| parse_schedule(s))
        .transpose()
        .unwrap_or_else(|e| {
            eprintln!("Error parsing --add-forks: {}", e);
            std::process::exit(1);
        })
        .unwrap_or_default();

    if !philosopher_schedule.is_empty() {
        println!("Scheduled philosopher additions: {:?}", philosopher_schedule);
    }
    if !fork_schedule.is_empty() {
        println!("Scheduled fork additions: {:?}", fork_schedule);
    }

    // Calculate max times before moving schedules
    let max_philosopher_time = philosopher_schedule
        .iter()
        .map(|(t, _)| *t)
        .max()
        .unwrap_or(0);
    let max_fork_time = fork_schedule.iter().map(|(t, _)| *t).max().unwrap_or(0);

    if args.priority {
        println!("Priority system enabled: Hungrier philosophers get priority!");
    }

    // Shared state
    let forks = Arc::new(RwLock::new(Vec::new()));
    let next_fork_id = Arc::new(Mutex::new(0u32));
    let philosopher_handles = Arc::new(Mutex::new(Vec::new()));
    let running = Arc::new(RwLock::new(true));
    let next_philosopher_id = Arc::new(Mutex::new(0u32));
    let all_stats: Arc<Mutex<Vec<HungerStats>>> = Arc::new(Mutex::new(Vec::new()));

    // Create initial forks
    {
        let mut forks_lock = forks.write().unwrap();
        let mut fork_id_lock = next_fork_id.lock().unwrap();
        for _ in 0..args.forks {
            forks_lock.push(Arc::new(Fork {
                id: *fork_id_lock,
                mutex: Mutex::new(()),
                priority_queue: Mutex::new(Vec::new()),
            }));
            *fork_id_lock += 1;
        }
    }

    // Pool of philosopher names
    let philosopher_names = Arc::new(vec![
        "Jürgen Habermas",
        "Friedrich Engels",
        "Karl Marx",
        "Thomas Piketty",
        "Michel Foucault",
        "Socrates",
        "Plato",
        "Aristotle",
        "Pythagoras",
        "Heraclitus",
        "Democritus",
        "Diogenes",
        "Epicurus",
        "Zeno of Citium",
        "Thales of Miletus",
        "Immanuel Kant",
        "René Descartes",
        "John Locke",
        "David Hume",
        "Jean-Jacques Rousseau",
        "Voltaire",
        "Friedrich Nietzsche",
        "Arthur Schopenhauer",
        "Søren Kierkegaard",
        "Baruch Spinoza",
        "Gottfried Leibniz",
        "Thomas Hobbes",
        "John Stuart Mill",
        "Georg Wilhelm Friedrich Hegel",
        "Bertrand Russell",
        "Ludwig Wittgenstein",
        "Martin Heidegger",
        "Jean-Paul Sartre",
        "Simone de Beauvoir",
        "Albert Camus",
    ]);

    // Randomly select initial philosopher names
    let mut rng = rand::thread_rng();
    let mut available_names: Vec<_> = philosopher_names.iter().copied().collect();
    available_names.shuffle(&mut rng);

    // Create initial philosophers
    for i in 0..args.philosophers {
        let name = available_names[i % available_names.len()];
        
        let mut id_lock = next_philosopher_id.lock().unwrap();
        let id = *id_lock;
        *id_lock += 1;
        drop(id_lock);
        
        let forks_lock = forks.read().unwrap();
        let num_forks = forks_lock.len();
        
        if num_forks == 0 {
            println!("Cannot spawn philosopher {}: no forks available!", name);
            continue;
        }

        let left_fork_idx = (id as usize) % num_forks;
        let right_fork_idx = ((id as usize) + 1) % num_forks;

        let stats = if args.priority {
            let s = HungerStats {
                id,
                name: name.to_string(),
                times_eaten: Arc::new(Mutex::new(0)),
                total_wait_time: Arc::new(Mutex::new(Duration::from_secs(0))),
                last_meal_time: Arc::new(Mutex::new(Some(Instant::now()))),
                current_hunger: Arc::new(Mutex::new(Duration::from_secs(0))),
            };
            all_stats.lock().unwrap().push(s.clone());
            Some(s)
        } else {
            None
        };

        let philosopher = Philosopher::new(
            id,
            name,
            Arc::clone(&forks_lock[left_fork_idx]),
            Arc::clone(&forks_lock[right_fork_idx]),
            Arc::clone(&running),
            args.priority,
            stats,
        );
        
        drop(forks_lock);

        let handle = thread::spawn(move || {
            philosopher.eat();
        });

        philosopher_handles.lock().unwrap().push(handle);
    }

    let start = Instant::now();

    // Spawn thread to handle dynamic fork additions
    if !fork_schedule.is_empty() {
        let forks_clone = Arc::clone(&forks);
        let next_fork_id_clone = Arc::clone(&next_fork_id);
        let start_clone = start;
        
        thread::spawn(move || {
            for (delay_secs, count) in fork_schedule {
                thread::sleep(Duration::from_secs(delay_secs));
                
                let mut forks_lock = forks_clone.write().unwrap();
                let mut fork_id_lock = next_fork_id_clone.lock().unwrap();
                
                println!(
                    "\n>>> Adding {} fork(s) at {:?} <<<",
                    count,
                    Instant::now().duration_since(start_clone)
                );
                
                for _ in 0..count {
                    let fork = Arc::new(Fork {
                        id: *fork_id_lock,
                        mutex: Mutex::new(()),
                        priority_queue: Mutex::new(Vec::new()),
                    });
                    println!("    Fork {} created", fork.id);
                    forks_lock.push(fork);
                    *fork_id_lock += 1;
                }
            }
        });
    }

    // Spawn thread to handle dynamic philosopher additions
    if !philosopher_schedule.is_empty() {
        let forks_clone = Arc::clone(&forks);
        let philosopher_handles_clone = Arc::clone(&philosopher_handles);
        let running_clone = Arc::clone(&running);
        let philosopher_names_clone = Arc::clone(&philosopher_names);
        let next_philosopher_id_clone = Arc::clone(&next_philosopher_id);
        let all_stats_clone = Arc::clone(&all_stats);
        let use_priority = args.priority;
        let start_clone = start;
        
        thread::spawn(move || {
            let mut name_idx = args.philosophers;
            
            for (delay_secs, count) in philosopher_schedule {
                thread::sleep(Duration::from_secs(delay_secs));
                
                println!(
                    "\n>>> Adding {} philosopher(s) at {:?} <<<",
                    count,
                    Instant::now().duration_since(start_clone)
                );
                
                for _ in 0..count {
                    let name = philosopher_names_clone[name_idx % philosopher_names_clone.len()];
                    name_idx += 1;
                    
                    let mut id_lock = next_philosopher_id_clone.lock().unwrap();
                    let id = *id_lock;
                    *id_lock += 1;
                    drop(id_lock);
                    
                    let forks_lock = forks_clone.read().unwrap();
                    let num_forks = forks_lock.len();
                    
                    if num_forks == 0 {
                        println!("    Cannot spawn philosopher {}: no forks available!", name);
                        continue;
                    }

                    let left_fork_idx = (id as usize) % num_forks;
                    let right_fork_idx = ((id as usize) + 1) % num_forks;

                    let stats = if use_priority {
                        let s = HungerStats {
                            id,
                            name: name.to_string(),
                            times_eaten: Arc::new(Mutex::new(0)),
                            total_wait_time: Arc::new(Mutex::new(Duration::from_secs(0))),
                            last_meal_time: Arc::new(Mutex::new(Some(Instant::now()))),
                            current_hunger: Arc::new(Mutex::new(Duration::from_secs(0))),
                        };
                        all_stats_clone.lock().unwrap().push(s.clone());
                        Some(s)
                    } else {
                        None
                    };

                    let philosopher = Philosopher::new(
                        id,
                        name,
                        Arc::clone(&forks_lock[left_fork_idx]),
                        Arc::clone(&forks_lock[right_fork_idx]),
                        Arc::clone(&running_clone),
                        use_priority,
                        stats,
                    );
                    
                    println!("    Philosopher {} ({}) spawned", id, name);
                    drop(forks_lock);

                    let handle = thread::spawn(move || {
                        philosopher.eat();
                    });

                    philosopher_handles_clone.lock().unwrap().push(handle);
                }
            }
        });
    }

    // Hunger level update thread (if priority enabled)
    if args.priority {
        let all_stats_clone = Arc::clone(&all_stats);
        let running_clone = Arc::clone(&running);
        
        thread::spawn(move || {
            while *running_clone.read().unwrap() {
                thread::sleep(Duration::from_millis(100));
                let stats_lock = all_stats_clone.lock().unwrap();
                for stat in stats_lock.iter() {
                    if let Some(last_meal) = *stat.last_meal_time.lock().unwrap() {
                        *stat.current_hunger.lock().unwrap() = last_meal.elapsed();
                    }
                }
            }
        });
    }

    // Stats display thread (if enabled)
    if let Some(interval) = args.stats {
        if args.priority {
            let all_stats_clone = Arc::clone(&all_stats);
            let running_clone = Arc::clone(&running);
            let start_clone = start;
            
            thread::spawn(move || {
                while *running_clone.read().unwrap() {
                    thread::sleep(Duration::from_secs(interval));
                    
                    println!("\n=== HUNGER STATISTICS at {:?} ===", Instant::now().duration_since(start_clone));
                    let stats_lock = all_stats_clone.lock().unwrap();
                    
                    // Create a sorted copy by current hunger level (descending)
                    let mut sorted_stats: Vec<_> = stats_lock.iter().collect();
                    sorted_stats.sort_by_key(|s| std::cmp::Reverse(*s.current_hunger.lock().unwrap()));
                    
                    for stat in sorted_stats.iter().take(10) {  // Show top 10 hungriest
                        let times_eaten = *stat.times_eaten.lock().unwrap();
                        let total_wait = *stat.total_wait_time.lock().unwrap();
                        let current_hunger = *stat.current_hunger.lock().unwrap();
                        let avg_wait = if times_eaten > 0 {
                            total_wait / times_eaten as u32
                        } else {
                            Duration::from_secs(0)
                        };
                        
                        println!(
                            "  {} (ID {}): Hunger={:?}, Eaten={}, Avg Wait={:?}",
                            stat.name, stat.id, current_hunger, times_eaten, avg_wait
                        );
                    }
                    println!("================================\n");
                }
            });
        } else {
            println!("Warning: --stats requires --priority to be enabled");
        }
    }
    
    // Calculate runtime - run for 10 seconds or until all scheduled additions complete
    let runtime = args.runtime.unwrap_or_else(|| {
        std::cmp::max(10, std::cmp::max(max_philosopher_time, max_fork_time) + 5)
    });
    
    println!("\nRunning simulation for {} seconds...\n", runtime);
    thread::sleep(Duration::from_secs(runtime));

    // Signal all philosophers to stop
    *running.write().unwrap() = false;

    // Wait for all philosopher threads to complete
    let handles = std::mem::take(&mut *philosopher_handles.lock().unwrap());
    for handle in handles {
        let _ = handle.join();
    }

    println!("\nTotal time: {:?}", start.elapsed());
    println!(
        "Final state: {} philosophers, {} forks",
        *next_philosopher_id.lock().unwrap(),
        forks.read().unwrap().len()
    );

    // Final stats summary
    if args.priority {
        println!("\n=== FINAL HUNGER STATISTICS ===");
        let stats_lock = all_stats.lock().unwrap();
        let mut sorted_stats: Vec<_> = stats_lock.iter().collect();
        sorted_stats.sort_by_key(|s| std::cmp::Reverse(*s.times_eaten.lock().unwrap()));
        
        for stat in sorted_stats.iter() {
            let times_eaten = *stat.times_eaten.lock().unwrap();
            let total_wait = *stat.total_wait_time.lock().unwrap();
            let avg_wait = if times_eaten > 0 {
                total_wait / times_eaten as u32
            } else {
                Duration::from_secs(0)
            };
            
            println!(
                "  {} (ID {}): Ate {} times, Total wait={:?}, Avg wait={:?}",
                stat.name, stat.id, times_eaten, total_wait, avg_wait
            );
        }
        println!("================================");
    }
}

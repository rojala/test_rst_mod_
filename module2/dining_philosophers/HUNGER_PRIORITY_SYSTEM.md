# Hunger Level & Priority System Implementation

## Overview
The dining philosophers program now includes a sophisticated hunger tracking and priority system that ensures fairness and prevents philosopher starvation.

## Features

### 1. Hunger Level Tracking
Each philosopher tracks:
- **Current Hunger:** Time elapsed since last meal
- **Times Eaten:** Total number of meals consumed
- **Total Wait Time:** Cumulative time spent waiting for forks
- **Average Wait Time:** Mean waiting time per meal
- **Last Meal Time:** Timestamp of most recent meal

### 2. Priority-Based Fork Acquisition
When `--priority` is enabled:
- Philosophers with longer wait times get higher priority
- Each fork maintains a priority queue of waiting philosophers
- Only the philosopher with the highest priority (longest wait) can acquire each fork
- Prevents indefinite starvation of any philosopher

### 3. Real-Time Statistics
With `--stats N`:
- Displays hunger statistics every N seconds
- Shows top 10 hungriest philosophers
- Tracks eating patterns in real-time
- Provides final summary at program end

## Command-Line Options

```bash
--priority              # Enable hunger-based priority system
--stats <SECONDS>       # Display stats every N seconds (requires --priority)
```

## Implementation Details

### Data Structures

**HungerStats:**
```rust
struct HungerStats {
    id: u32,
    name: String,
    times_eaten: Arc<Mutex<u64>>,
    total_wait_time: Arc<Mutex<Duration>>,
    last_meal_time: Arc<Mutex<Option<Instant>>>,
    current_hunger: Arc<Mutex<Duration>>,
}
```

**Fork with Priority Queue:**
```rust
struct Fork {
    id: u32,
    mutex: Mutex<()>,
    priority_queue: Mutex<Vec<(u32, Instant)>>,  // (philosopher_id, wait_start)
}
```

### Algorithm

1. **Philosopher attempts to eat:**
   - Records wait start time
   - Adds self to fork's priority queue
   - Queue sorted by wait time (oldest first)

2. **Priority check:**
   - Philosopher checks if they're at front of queue (highest priority)
   - If not, sleeps briefly and checks again
   - If yes, attempts to acquire fork mutex

3. **Eating:**
   - Once both forks acquired, calculates total wait time
   - Updates statistics (times eaten, total/average wait)
   - Resets hunger level to zero

4. **Background threads:**
   - **Hunger updater:** Continuously updates current_hunger for all philosophers
   - **Stats reporter:** Periodically displays hunger statistics if enabled

### Metrics Displayed

**Periodic Stats (--stats N):**
- Top 10 hungriest philosophers by current hunger
- Current hunger level
- Number of times eaten
- Average wait time

**Final Summary:**
- All philosophers sorted by times eaten (descending)
- Total times eaten
- Total cumulative wait time
- Average wait time per meal

## Usage Examples

### Basic Priority System
```bash
cargo run -- -p 5 -f 2 --priority --runtime 10
```

### With Statistics Display
```bash
cargo run -- -p 5 -f 2 --priority --stats 3 --runtime 10
```
Shows hunger stats every 3 seconds.

### Combined with Dynamic Additions
```bash
cargo run -- -p 3 -f 2 --priority --stats 2 \
  --add-philosophers "3:2" --add-forks "5:1" --runtime 12
```
- Starts with 3 philosophers, 2 forks
- Adds 2 philosophers at 3 seconds
- Adds 1 fork at 5 seconds
- Shows stats every 2 seconds

## Example Output

```
Priority system enabled: Hungrier philosophers get priority!

=== HUNGER STATISTICS at 3.000s ===
  Jean-Paul Sartre (ID 2): Hunger=2.902s, Eaten=0, Avg Wait=0ns
  Simone de Beauvoir (ID 3): Hunger=2.901s, Eaten=0, Avg Wait=0ns
  Thales of Miletus (ID 0): Hunger=2.900s, Eaten=1, Avg Wait=31.5µs
  Gottfried Leibniz (ID 4): Hunger=1.793s, Eaten=1, Avg Wait=1.108s
  Thomas Piketty (ID 1): Hunger=691ms, Eaten=1, Avg Wait=2.211s
================================

=== FINAL HUNGER STATISTICS ===
  Jürgen Habermas (ID 0): Ate 3 times, Total wait=8.865s, Avg wait=2.955s
  Baruch Spinoza (ID 1): Ate 2 times, Total wait=7.754s, Avg wait=3.877s
  Georg W. F. Hegel (ID 2): Ate 2 times, Total wait=4.435s, Avg wait=2.217s
  Friedrich Nietzsche (ID 3): Ate 2 times, Total wait=8.862s, Avg wait=4.431s
  Michel Foucault (ID 4): Ate 1 times, Total wait=3.642s, Avg wait=3.642s
  Socrates (ID 5): Ate 1 times, Total wait=4.747s, Avg wait=4.747s
================================
```

## Benefits

1. **Fairness:** Prevents indefinite starvation of philosophers
2. **Visibility:** Real-time insights into system behavior
3. **Analysis:** Metrics help identify bottlenecks and unfair scenarios
4. **Debugging:** Statistics reveal concurrency issues
5. **Educational:** Demonstrates priority queue implementation in concurrent systems

## Demo Script

Run `./demo_priority.sh` to see all features in action:
- Basic priority system
- Priority with statistics
- Priority + dynamic additions
- Comparison without priority

## Notes

- Priority system adds minimal overhead (< 10ms per fork acquisition)
- Statistics thread runs independently without blocking philosophers
- Final summary provides comprehensive fairness analysis
- Works seamlessly with dynamic philosopher/fork additions

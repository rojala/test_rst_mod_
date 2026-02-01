# Quick Reference - Dining Philosophers

## Basic Usage
```bash
# Default: 15 philosophers, 4 forks
cargo run

# Custom configuration
cargo run -- -p 10 -f 5
```

## Priority System
```bash
# Enable hunger-based priority
cargo run -- --priority

# With real-time statistics (every 3 seconds)
cargo run -- --priority --stats 3
```

## Dynamic Additions
```bash
# Add 2 philosophers after 3 seconds
cargo run -- --add-philosophers "3:2"

# Add 1 fork after 5 seconds
cargo run -- --add-forks "5:1"

# Multiple additions: 2 philosophers at 3s, 1 more at 5s
cargo run -- --add-philosophers "3:2,5:1"
```

## Combined Examples
```bash
# Everything: priority, stats, dynamic additions
cargo run -- -p 3 -f 2 --priority --stats 2 \
  --add-philosophers "3:2" --add-forks "5:1" --runtime 10

# Fairness test: few forks, many philosophers, priority enabled
cargo run -- -p 10 -f 3 --priority --stats 3 --runtime 15

# Starvation prevention demo
cargo run -- -p 6 -f 2 --priority --stats 2 --runtime 10
```

## Demo Scripts
```bash
# Dynamic additions demo
./demo.sh

# Priority system demo
./demo_priority.sh
```

## Key Flags
| Flag | Description |
|------|-------------|
| `-p, --philosophers <N>` | Number of philosophers (default: 15) |
| `-f, --forks <N>` | Number of forks (default: 4) |
| `--add-philosophers <TIME:COUNT,...>` | Add philosophers at specified times |
| `--add-forks <TIME:COUNT,...>` | Add forks at specified times |
| `--priority` | Enable hunger-based priority system |
| `--stats <SECONDS>` | Display hunger statistics every N seconds |
| `--runtime <SECONDS>` | Simulation duration |
| `-h, --help` | Show help message |

## Understanding Output

**Without Priority:**
```
Socrates picked up fork 0.
Socrates picked up fork 1.
Socrates is eating.
Socrates finished eating.
```

**With Priority:**
```
Socrates picked up fork 0.
Socrates picked up fork 1.
Socrates is eating. (waited 2.345s)
Socrates finished eating.

=== HUNGER STATISTICS at 3.000s ===
  Plato (ID 1): Hunger=2.902s, Eaten=0, Avg Wait=0ns
  Aristotle (ID 2): Hunger=2.901s, Eaten=1, Avg Wait=1.234s
  Socrates (ID 0): Hunger=655ms, Eaten=1, Avg Wait=2.345s
================================
```

## Interpreting Hunger Stats
- **Hunger**: Time since last meal (higher = hungrier)
- **Eaten**: Total number of meals consumed
- **Avg Wait**: Average time spent waiting for forks
- **Total Wait**: Cumulative waiting time

## Documentation Files
- `Readme.md` - Full lab instructions and explanations
- `HUNGER_PRIORITY_SYSTEM.md` - Detailed priority system documentation
- `src/main.rs` - Complete source code with comments
- `src/main_original.rs` - Original simple implementation

## Tips
1. Use `--priority` for fairness in resource-constrained scenarios
2. Use `--stats` to monitor real-time behavior
3. Compare runs with/without priority to see the difference
4. Try extreme ratios (many philosophers, few forks) to stress test
5. Combine dynamic additions with priority to see adaptation

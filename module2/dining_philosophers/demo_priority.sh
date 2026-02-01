#!/bin/bash

# Demo script for the hunger tracking and priority system

echo "===================================="
echo "DEMO 1: Basic Priority System"
echo "5 philosophers, 2 forks, priority enabled"
echo "===================================="
cargo run -- -p 5 -f 2 --priority --runtime 8
echo ""
echo "Press Enter to continue..."
read

echo "===================================="
echo "DEMO 2: Priority with Statistics"
echo "5 philosophers, 2 forks, stats every 3 seconds"
echo "===================================="
cargo run -- -p 5 -f 2 --priority --stats 3 --runtime 9
echo ""
echo "Press Enter to continue..."
read

echo "===================================="
echo "DEMO 3: Priority + Dynamic Additions"
echo "Start: 3 philosophers, 2 forks"
echo "Add: 2 philosophers at 3s, 1 fork at 5s"
echo "===================================="
cargo run -- -p 3 -f 2 --priority --stats 3 --add-philosophers "3:2" --add-forks "5:1" --runtime 10
echo ""
echo "Press Enter to continue..."
read

echo "===================================="
echo "DEMO 4: Comparison - Without Priority"
echo "5 philosophers, 2 forks, NO priority"
echo "===================================="
cargo run -- -p 5 -f 2 --runtime 8
echo ""
echo "Demo complete!"

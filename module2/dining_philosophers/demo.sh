#!/bin/bash

# Demonstration script for dynamic philosopher/fork additions

echo "=== Demo 1: Basic dynamic philosopher addition ==="
echo "Starting with 2 philosophers, 2 forks"
echo "Adding 1 philosopher at 2 seconds"
echo ""
timeout 6 cargo run -- -p 2 -f 2 --add-philosophers "2:1" --runtime 5

echo ""
echo "=== Demo 2: Multiple additions ==="
echo "Starting with 2 philosophers, 2 forks"
echo "Adding 1 philosopher at 2s, 2 more at 4s"
echo "Adding 2 forks at 3s"
echo ""
timeout 8 cargo run -- -p 2 -f 2 --add-philosophers "2:1,4:2" --add-forks "3:2" --runtime 6

echo ""
echo "=== Demo 3: Complex scenario ==="
echo "Starting with 3 philosophers, 2 forks"
echo "Adding 2 philosophers at 2s"
echo "Adding 1 fork at 3s, 2 more forks at 5s"
echo ""
timeout 10 cargo run -- -p 3 -f 2 --add-philosophers "2:2" --add-forks "3:1,5:2" --runtime 8

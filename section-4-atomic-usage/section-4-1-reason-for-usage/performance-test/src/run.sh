#!/bin/bash

# delete Cargo's package cache
echo "Removing Cargo package cache..."
rm -rf ~/.cargo/.package-cache

# clean the project
echo "Cleaning project..."
cargo clean

# compile and run the project
echo "Building and running project..."
cargo run

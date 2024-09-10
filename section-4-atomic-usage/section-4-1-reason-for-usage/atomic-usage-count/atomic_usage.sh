#!/bin/bash

# Project URLs array
projects=(
    "https://github.com/tikv/tikv.git"
    "https://github.com/apache/incubator-teaclave-sgx-sdk.git"
    "https://github.com/spacejam/sled.git"
    "https://github.com/servo/servo.git"
    "https://github.com/influxdata/influxdb.git"
    "https://github.com/occlum/occlum.git"
    "https://github.com/ProvableHQ/snarkOS.git"
    "https://github.com/rayon-rs/rayon.git"
    "https://github.com/Amanieu/parking_lot.git"
    "https://github.com/tokio-rs/tokio.git"
    "https://github.com/actix/actix.git"
    "https://github.com/crossbeam-rs/crossbeam.git"
    "https://github.com/matklad/once_cell.git"
)

# Corresponding commit hash array
commits=(
    "3f7c63646ef5ea842f7ce6552826976feda2f609"
    "3c903bdac4e503dd27b9b1f761c4abfc55f2464c"
    "005c023ca94d424d8e630125e4c21320ed160031"
    "5d7ed76b79de359ef1de2bdee83b32bd497d7cd8"
    "bb6a5c0bf6968117251617cda99cb39a5274b6dd"
    "f54eabfa92319490e1b13075b28fef2d3d9129fd"
    "e7d39272d0c008c6d67cf1fdbf0da5de8b5001f7"
    "d1b18e616eec5ce8520aecb31054b180006527a8"
    "0b296160941275d8df757066dd26361d6ae5d455"
    "7b555185ff9186f618b198126ee853980b187698"
    "d0509d350ca9a6b7ec67e00d325518b9128721e3"
    "18afbb6ed2f98e55ae5cc10578e54762232a2437"
    "8f39b775effd387b175993b0091b082c4d60f921"
)

# Path to the counting script
count_script="count.py"

project_dir="project"
mkdir -p "$project_dir"

# Iterate over the project array
for i in "${!projects[@]}"; do
    project_url=${projects[$i]}
    commit_hash=${commits[$i]}

    # Extract project name
    project_name=$(basename -s .git "$project_url")

    # Echo message to indicate cloning
    echo "Cloning $project_name from $project_url..."
    git clone --quiet "$project_url" "project/$project_name"

    # Echo message to indicate checkout
    echo "Checking out commit $commit_hash for $project_name..."
    cd "project/$project_name" || exit
    git checkout --quiet "$commit_hash"

    # Echo message to indicate script execution
    echo "Running $count_script for $project_name..."
    cd ../../
    python3 "$count_script" "./project/$project_name"

    # Echo completion message
    echo "$project_name processing completed."

done

echo "All projects processed successfully."

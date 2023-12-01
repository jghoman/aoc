#!/bin/bash

# Check if the argument count is correct
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <number between 1 and 25>"
    exit 1
fi

# Extract the number and pad it with zero if necessary
number=$(printf "%02d" $1)

# Check if the number is within the valid range
if [ $number -lt 1 ] || [ $number -gt 25 ]; then
    echo "Error: Number must be between 1 and 25."
    exit 1
fi

# Create the new Cargo project
cargo new "day$number"
if [ $? -ne 0 ]; then
    echo "Cargo project creation failed."
    exit 1
fi

# Create a bin directory under src
mkdir "day$number/src/bin"

# Move main.rs to the bin directory as part1.rs
mv "day$number/src/main.rs" "day$number/src/bin/part1.rs"

echo "Project day$number created successfully."

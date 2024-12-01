# advent-of-code
[Advent of code](https://adventofcode.com/) solutions. Most recently in Rust.

## Running Rust solutions
Running tests and solutions is mostly conveniently done within the directory for each solution while editing:
```bash
cd 2022/src/bin/aoc-25

# Run tests:
cargo test --bin $(basename $(pwd))
# Run tests and display output in a readable manner:
RUST_BACKTRACE=1 cargo test --bin $(basename $(pwd)) -- --nocapture --test-threads 1

# Run solution:
cargo run --release --bin $(basename $(pwd)) input.txt
```

## Running Python solutions

1. Install `uv`
2. Navigate to daily directory
2. Run with `./main.py input_file`

## Running C++ solutions
Generally these can be compiled and run as follows:
```bash
cd 2018/10

# Compile:
clang++ --std=c++14 -g main.cpp
# Run:
cat input.txt | ./a.out
```

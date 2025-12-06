# Day 06: Column Operations

Rust solutions for the Advent of Code 2025 Day 06 puzzle.

- Part 1 parses whitespace-separated numbers into columns, applies operations (* or +) from the last line to each column, and sums the results.
- Part 2 reads the grid vertically by character position, concatenating digits at each position into numbers. The operations line spacing determines how these numbers are grouped, then the same operations are applied.

Run the day's tests with `cargo test --release day06`.

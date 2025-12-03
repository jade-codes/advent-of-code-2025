# Day 03: Battery Joltage

Rust solutions for the Advent of Code 2025 Day 03 puzzle.

- Part 1 selects 2 batteries from each sequence to maximize the resulting 2-digit number, using DP to track the best selection at each position.
- Part 2 selects 12 batteries from each sequence to maximize the resulting 12-digit number, using the same DP approach with state `dp[i][j]` representing the maximum number formed by choosing `j` batteries from the first `i` positions.

Run the day's tests with `cargo test --release day03::tests`.

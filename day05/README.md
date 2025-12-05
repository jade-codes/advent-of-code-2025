# Day 05: Range Matching

Rust solutions for the Advent of Code 2025 Day 05 puzzle.

- Part 1 counts how many numbers from the input list fall within the merged ranges. The solution parses ranges, sorts and merges overlapping/adjacent ranges, then uses a queue to efficiently check which numbers are contained.
- Part 2 calculates the total size of all merged ranges by summing `(end - start + 1)` for each merged range.

Run the day's tests with `cargo test --release day05::tests`.

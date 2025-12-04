# Day 02: Invalid IDs

Rust solutions for the Advent of Code 2025 Day 02 puzzle.

- Part 1 finds IDs formed by repeating a sequence exactly twice (e.g., `12341234`), using a multiplication formula to enumerate even-length candidates.
- Part 2 finds IDs formed by repeating any sequence at least twice (e.g., `123123123`), iterating over divisors of the length and deduplicating with a HashSet.

Run the day's tests with `cargo test --release day02`.

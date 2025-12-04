# Day 04: Paper Roll Removal

Rust solutions for the Advent of Code 2025 Day 04 puzzle.

- Part 1 counts all paper rolls (`@`) that have fewer than 4 adjacent paper rolls (considering all 8 directions: orthogonal and diagonal).
- Part 2 simulates iterative removal of accessible paper rolls. A roll can be removed if it has fewer than 4 adjacent paper rolls AND is adjacent to at least one empty space (`.`). After each removal round, newly accessible rolls are checked and removed in subsequent rounds.

Run the day's tests with `cargo test --release day04`.

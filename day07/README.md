# Day 07: Grid Path Navigation

Rust solutions for the Advent of Code 2025 Day 07 puzzle.

- Part 1 navigates down from the 'S' starting position, counting how many '^' branch points are encountered while exploring the grid.
- Part 2 counts all possible distinct paths from 'S' to the bottom of the grid. At each '^' character, the path branches both left and right. Uses memoization to efficiently calculate the total number of paths.

Run the day's tests with `cargo test --release day07`.

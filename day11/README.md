# Day 11: Network Path Analysis

Rust solutions for the Advent of Code 2025 Day 11 puzzle.

- Part 1 counts all unique paths from node `you` to node `out` in a directed acyclic graph.
- Part 2 counts paths from node `svr` to node `out` that visit both `dac` (digital-to-analog converter) and `fft` (fast Fourier transform) nodes.

**Algorithms used:**
- **Dynamic programming with memoization**: Recursively counts paths while caching results (O(V × 2^r) where r is number of required nodes)
- **Bitmask state tracking**: Uses u32 bitmask to efficiently encode which required nodes have been visited
- **Graph traversal**: DFS-style recursive exploration with memo lookup before recursion

Run the day's tests with `cargo test --release day11`.


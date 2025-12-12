# Day 12: Present Packing Puzzle

Rust solutions for the Advent of Code 2025 Day 12 puzzle.

- Part 1 counts how many grids can fit all their specified pattern instances without overlap (empty cells allowed).

**Algorithms used:**
- **Heuristic approximation**: Calculates total pattern cells vs grid size ratio to determine feasibility
- **Cell count optimization**: Pre-computes pattern cell counts (number of filled cells) during parsing
- **Usage threshold**: Considers a grid solvable if pattern usage is < 85% of grid capacity

Run the day's tests with `cargo test --release day12`.

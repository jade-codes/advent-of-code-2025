# Day 08: Junction Box Circuits

Rust solutions for the Advent of Code 2025 Day 08 puzzle.

- Part 1 connects junction boxes in 3D space by processing pairs in order of Euclidean distance. After making 1000 connection attempts (trying the 1000 shortest pairs), it multiplies together the sizes of the three largest circuits.
- Part 2 continues connecting junction boxes until all boxes form a single circuit. Returns the product of the X coordinates of the final two junction boxes that complete the circuit.

**Algorithms used:**
- **Union-Find (Disjoint Set Union)**: Efficiently tracks which junction boxes are in the same circuit with path compression and union by size
- **Greedy edge selection**: Processes pairs in sorted order by distance (similar to Kruskal's MST algorithm)

Run the day's tests with `cargo test --release day08`.

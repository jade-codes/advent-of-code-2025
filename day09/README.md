# Day 09: Polygon Rectangles

Rust solutions for the Advent of Code 2025 Day 09 puzzle.

- Part 1 finds the largest axis-aligned rectangle that can be formed between any two points from the input. Calculates area as `(width + 1) × (height + 1)` to count grid cells inclusively.
- Part 2 finds the largest axis-aligned rectangle that fits completely inside the polygon formed by connecting the input points in order. The rectangle must not cross any polygon edges.

**Algorithms used:**
- **Ray casting algorithm**: Point-in-polygon test using horizontal ray intersection counting (O(n) per point)
- **Line segment intersection**: Counter-clockwise orientation test using cross products to detect if rectangle edges cross polygon boundaries
- **Computational geometry**: Validates rectangles by checking corners are inside and edges don't intersect polygon edges

Run the day's tests with `cargo test --release day09`.

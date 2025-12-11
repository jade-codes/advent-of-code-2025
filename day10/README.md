# Day 10: Button Configuration

Rust solutions for the Advent of Code 2025 Day 10 puzzle.

- Part 1 solves a "lights out" puzzle where buttons toggle indicator lights via XOR operations. Finds the minimum number of button presses to match a target light pattern using brute force over all combinations.
- Part 2 configures joltage counters where each button press increments specific counters by 1. Finds the minimum button presses to reach target values by solving a system of linear equations with integer constraints.

**Algorithms used:**
- **Brute force enumeration**: Part 1 tries all 2^n button combinations using bitwise XOR (O(2^n × n))
- **Gaussian elimination**: Integer-preserving row echelon form to solve linear systems (O(m × n^2))
- **Back-substitution with free variables**: For underdetermined systems, enumerates values for free variables to find optimal solution
- **Bitfield encoding**: Buttons and lights represented as u32 with bits indicating which counters/lights are affected

Run the day's tests with `cargo test --release day10`.


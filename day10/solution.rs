use std::path::Path;
use regex::Regex;

// Add the parent directory to access utils
#[path = "../utils.rs"]
mod utils;


type ParsedLine = (u32, Vec<u32>, Vec<i64>, usize); 


fn parse_line(line: &str) -> ParsedLine {
    let diagrams_re = Regex::new(r"\[([.#]+)\]").unwrap();
    let schematics_re = Regex::new(r"\(([0-9,]+)\)").unwrap();
    let requirements_re = Regex::new(r"\{([\d,]+)\}").unwrap();
    
    let (target, n_lights) = diagrams_re.captures(line)
        .map(|c| {
            let pattern = &c[1];
            let n = pattern.len();
            let target = pattern.chars()
                .enumerate()
                .fold(0u32, |acc, (i, ch)| {
                    if ch == '#' {
                        acc | (1u32 << i)
                    } else {
                        acc
                    }
                });
            (target, n)
        })
        .unwrap_or((0, 0));
    
    let buttons: Vec<u32> = schematics_re.captures_iter(line)
        .map(|c| {
            c[1].split(',')
                .filter_map(|s| s.parse::<usize>().ok())
                .fold(0u32, |acc, bit_pos| acc | (1u32 << bit_pos))
        })
        .collect();
    
    let requirements: Vec<i64> = requirements_re.captures(line)
        .map(|c| c[1].split(',')
            .filter_map(|s| s.parse().ok())
            .collect())
        .unwrap_or_default();
    
    (target, buttons, requirements, n_lights)
}

fn minimal_presses(buttons: &[u32], target: u32, n_lights: usize) -> usize {
    let n_buttons = buttons.len();
    
    let mut min_presses = usize::MAX;
    
    for mask in 0..(1_u64 << n_buttons) {
        let mut state = 0u32;
        let mut presses = 0;
        
        for (i, &button) in buttons.iter().enumerate() {
            if (mask & (1 << i)) != 0 {
                state ^= button;
                presses += 1;
            }
        }
        
        let mask_bits = (1u32 << n_lights) - 1;
        if (state & mask_bits) == (target & mask_bits) {
            min_presses = min_presses.min(presses);
        }
    }
    
    min_presses
}

fn solve_with_free_vars(
    free_vals: &[i64],
    free_vars: &[usize],
    pivot_col: &[usize],
    aug: &[Vec<i64>],
    n_buttons: usize,
) -> Option<Vec<i64>> {
    let mut presses = vec![0i64; n_buttons];
    for (&fvar, &val) in free_vars.iter().zip(free_vals.iter()) {
        presses[fvar] = val;
    }
    
    for (r, &col) in pivot_col.iter().enumerate().rev() {
        if aug[r][col] != 0 {
            let mut rhs = aug[r][n_buttons];
            for c in 0..n_buttons {
                if c != col {
                    rhs -= aug[r][c] * presses[c];
                }
            }
            if rhs % aug[r][col] != 0 {
                return None;
            }
            presses[col] = rhs / aug[r][col];
        }
    }
    
    if presses.iter().all(|&p| p >= 0) {
        Some(presses)
    } else {
        None
    }
}

fn enumerate_combos(
    combo: &mut Vec<i64>,
    idx: usize,
    max_val: i64,
    best: &mut i64,
    solve: &mut impl FnMut(&[i64]) -> Option<Vec<i64>>,
) {
    if idx == combo.len() {
        if let Some(solution) = solve(combo) {
            let total: i64 = solution.iter().sum();
            *best = (*best).min(total);
        }
        return;
    }
    
    for val in 0..=max_val {
        combo[idx] = val;
        enumerate_combos(combo, idx + 1, max_val, best, solve);
    }
}

fn minimal_presses_part2(buttons: &[u32], requirements: &[i64], n_counters: usize) -> i64 {
    let n_buttons = buttons.len();
    
    let mut matrix = vec![vec![0i64; n_buttons]; n_counters];
    for (btn_idx, &button) in buttons.iter().enumerate() {
        for (counter, row) in matrix.iter_mut().enumerate() {
            if (button & (1 << counter)) != 0 {
                row[btn_idx] = 1;
            }
        }
    }

    let mut aug = vec![vec![0i64; n_buttons + 1]; n_counters];
    for i in 0..n_counters {
        for j in 0..n_buttons {
            aug[i][j] = matrix[i][j];
        }
        aug[i][n_buttons] = requirements[i];
    }
    
    let mut pivot_col = Vec::new();
    let mut row = 0;
    for col in 0..n_buttons {
        if let Some(pivot_row) = (row..n_counters).find(|&r| aug[r][col] != 0) {
            aug.swap(row, pivot_row);
            pivot_col.push(col);
            
            for r in (row + 1)..n_counters {
                if aug[r][col] != 0 {
                    let pivot_coef = aug[row][col];
                    let row_coef = aug[r][col];
                    let pivot_row = aug[row].clone(); 
                    
                    for (j, &pivot_val) in pivot_row.iter().enumerate().skip(col) {
                        aug[r][j] = aug[r][j] * pivot_coef - pivot_val * row_coef;
                    }
                }
            }
            row += 1;
        }
    }
    
    let free_vars: Vec<usize> = (0..n_buttons).filter(|&i| !pivot_col.contains(&i)).collect();
    
    let max_free = *requirements.iter().max().unwrap_or(&100);
    let mut best_total = i64::MAX;
    
    let mut combo = vec![0i64; free_vars.len()];
    let mut solve = |free_vals: &[i64]| solve_with_free_vars(free_vals, &free_vars, &pivot_col, &aug, n_buttons);
    
    enumerate_combos(&mut combo, 0, max_free, &mut best_total, &mut solve);
    
    best_total
}

fn part1(_lines: &[String]) -> i64 {
    let all = _lines.iter()
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();  

    let mut total = 0;
    
    for (_target, buttons, _requirements, n_lights) in all.iter() {
        total += minimal_presses(buttons, *_target, *n_lights) as i64;
    }
    
    total
}

fn part2(_lines: &[String]) -> i64 {
    _lines.iter()
        .map(|line| parse_line(line))
        .map(|(_target, buttons, requirements, _n_lights)| {
            minimal_presses_part2(&buttons, &requirements, requirements.len())
        })
        .filter(|&result| result < i64::MAX)
        .sum()
}

pub fn main() {
    let input_path = Path::new(file!()).parent().unwrap().join("input.txt");
    let lines = utils::read_lines(&input_path);

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_simple() {
        // Single button, single light
        let input = vec!["[#] (0) {1}".to_string()];
        assert_eq!(part1(&input), 1);
        
        // Target already at 0, no presses needed
        let input = vec!["[..] (0) (1) {0}".to_string()];
        assert_eq!(part1(&input), 0);
    }
    
    #[test]
    fn test_part1_multiple_buttons() {
        // Two buttons affecting same light
        let input = vec!["[#] (0) (0) {1}".to_string()];
        assert_eq!(part1(&input), 1); // Either button works
        
        // Buttons that cancel out
        let input = vec!["[..] (0,1) (0,1) {0}".to_string()];
        assert_eq!(part1(&input), 0); // Already at target
    }
    
    #[test]
    fn test_part2_simple() {
        // Single button, single counter - need 5 presses
        let input = vec!["[.] (0) {5}".to_string()];
        assert_eq!(part2(&input), 5);
    }
    
    #[test]
    fn test_part2_unique_solution() {
        // Two buttons, two counters - unique solution
        // b0 affects counter 0, b1 affects counter 1
        let input = vec!["[..] (0) (1) {3,4}".to_string()];
        assert_eq!(part2(&input), 7); // 3 + 4 = 7 total presses
    }
    
    #[test]
    fn test_part2_overdetermined() {
        // More equations than variables - should still find solution if consistent
        let input = vec!["[..] (0,1) (0,1) {5,5}".to_string()];
        // Both equations say b0 + b1 = 5, so any split works, minimum is 5
        assert_eq!(part2(&input), 5);
    }

    #[test]
    fn test_part2_examples() {
        let input = vec![
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string(),
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string(),
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string(),
        ];
        assert_eq!(part2(&input), 33);
    }
    
    #[test]
    fn test_machine_12() {
        let input = vec![
            "[##........] (4,5,6,7,9) (0,1,2,3,4,5,6,8,9) (0,2,3,6,7,9) (6,8) (0,4,5,8) (0,4,5,7) (1,8,9) (1,2,3,6,9) {42,226,215,215,36,36,222,32,37,238}".to_string(),
        ];
        let result = part2(&input);
        assert_eq!(result, 262, "Machine 12 should return 262");
    }
}

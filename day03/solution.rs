use std::path::Path;

// Add the parent directory to access utils
#[path = "../utils.rs"]
mod utils;

fn part1(lines: &[String]) -> i64 {
    solve_batteries(lines, 2)
}

fn part2(lines: &[String]) -> i64 {
    solve_batteries(lines, 12)
}

fn solve_batteries(lines: &[String], target_count: usize) -> i64 {
    let mut max_total = 0;

    for line in lines {
        let numbers: Vec<i64> = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();
        
        let n = numbers.len();
        let mut dp: Vec<Vec<Option<Vec<i64>>>> = vec![vec![None; target_count + 1]; n + 1];
        dp[0][0] = Some(vec![]);
        
        for i in 0..n {
            for j in 0..=target_count.min(i + 1) {
                if dp[i][j].is_none() {
                    continue;
                }
                
                let curr_sequence = dp[i][j].as_ref().unwrap().clone();
                
                // Option 1: Don't take this battery
                if i + 1 < n || j == target_count {
                    if dp[i + 1][j].is_none() || is_better(&curr_sequence, dp[i + 1][j].as_ref().unwrap()) {
                        dp[i + 1][j] = Some(curr_sequence.clone());
                    }
                }
                
                // Option 2: Take this battery
                if j < target_count {
                    let mut next_sequence = curr_sequence.clone();
                    next_sequence.push(numbers[i]);
                    if dp[i + 1][j + 1].is_none() || is_better(&next_sequence, dp[i + 1][j + 1].as_ref().unwrap()) {
                        dp[i + 1][j + 1] = Some(next_sequence);
                    }
                }
            }
        }
        
        // Find the best with exactly target_count batteries
        let result_digits = dp[n][target_count].as_ref().unwrap();
        let mut result = 0_i64;
        for &digit in result_digits {
            result = result * 10 + digit;
        }
        max_total += result;
    }
    max_total
}

fn is_better(a: &[i64], b: &[i64]) -> bool {
    for i in 0..a.len().min(b.len()) {
        if a[i] > b[i] {
            return true;
        } else if a[i] < b[i] {
            return false;
        }
    }
    a.len() > b.len()
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
    fn test_part1() {
        let input = vec![
            "987654321111111".to_string(),
            "811111111111119".to_string(),
            "234234234234278".to_string(),
            "818181911112111".to_string(),
        ];
        assert_eq!(part1(&input), 357);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "987654321111111".to_string(),
            "811111111111119".to_string(),
            "234234234234278".to_string(),
            "818181911112111".to_string(),
        ];
        assert_eq!(part2(&input), 3121910778619);
    }

    #[test]
    fn test_single_line() {
        let input = vec!["987654321111791".to_string()];
        assert_eq!(part1(&input), 99);
        assert_eq!(part2(&input), 987654321791);
    }

    #[test]
    fn test_empty_input() {
        let input = vec![];
        assert_eq!(part1(&input), 0);
        assert_eq!(part2(&input), 0);
    }
}

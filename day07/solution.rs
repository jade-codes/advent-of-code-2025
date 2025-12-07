use std::collections::HashMap;
use std::path::Path;

// Add the parent directory to access utils
#[path = "../utils.rs"]
mod utils;

fn find_start_position(grid: &[Vec<char>]) -> Option<usize> {
    if grid.is_empty() {
        return None;
    }
    grid[0].iter().position(|&c| c == 'S')
}

fn move_down(count: &mut i64, grid: &mut [Vec<char>], row: usize, col: usize, visited: &mut HashMap<(usize, usize), bool>) {
    let next_row = row + 1;
    if next_row >= grid.len() {
        return;
    }

    if visited.contains_key(&(next_row, col)) {
        return;
    }

    visited.insert((next_row, col), true);
    
    if grid[next_row][col] == '^' {
        *count += 1;
        if col > 0 {
            grid[next_row][col - 1] = '|';
            move_down(count, grid, next_row, col - 1, visited);
        }
        if col + 1 < grid[next_row].len() {
            grid[next_row][col + 1] = '|';
            move_down(count, grid, next_row, col + 1, visited);
        }
    } else {
        grid[next_row][col] = '|';
        move_down(count, grid, next_row, col, visited);
    }
}

fn part1(grid: &[Vec<char>]) -> i64 {
    if let Some(pos) = find_start_position(grid) {
        let mut count = 0;
        let mut grid_copy = grid.to_vec();
        let mut visited = HashMap::new();

        move_down(&mut count, &mut grid_copy, 0, pos, &mut visited);        
        count
    } else {
        0
    }
}

fn count_paths(grid: &[Vec<char>], row: usize, col: usize, memo: &mut HashMap<(usize, usize), i64>) -> i64 {
    let next_row = row + 1;
    
    if next_row >= grid.len() {
        return 1;
    }

    if let Some(&cached) = memo.get(&(next_row, col)) {
        return cached;
    }
    
    let mut total_paths = 0;
    
    if grid[next_row][col] == '^' {
        if col > 0 {
            total_paths += count_paths(grid, next_row, col - 1, memo);
        }
        if col + 1 < grid[next_row].len() {
            total_paths += count_paths(grid, next_row, col + 1, memo);
        }
    } else {
        total_paths += count_paths(grid, next_row, col, memo);
    }
    
    memo.insert((next_row, col), total_paths);
    total_paths
}

fn part2(grid: &[Vec<char>]) -> i64 {
    if let Some(pos) = find_start_position(grid) {
        let mut memo = HashMap::new();
        count_paths(grid, 0, pos, &mut memo)
    } else {
        0
    }
}

pub fn main() {
    let input_path = Path::new(file!()).parent().unwrap().join("input.txt");
    let grid = utils::read_grid(&input_path);

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_simple() {
        let grid = vec![
            vec!['.', 'S', '.'],
            vec!['.', '.', '.'],
            vec!['.', '^', '.'],
            vec!['.', '.', '.'],
        ];
        assert_eq!(part1(&grid), 1);
    }

    #[test]
    fn test_part1_no_branches() {
        let grid = vec![
            vec!['.', 'S', '.'],
            vec!['.', '.', '.'],
            vec!['.', '.', '.'],
        ];
        assert_eq!(part1(&grid), 0);
    }

    #[test]
    fn test_part2_simple() {
  
        let grid = vec![
            vec!['.', 'S', '.'],
            vec!['.', '.', '.'],
            vec!['.', '^', '.'],
            vec!['.', '.', '.'],
        ];
        assert_eq!(part2(&grid), 2);
    }

    #[test]
    fn test_part2_multiple_branches() {
        let grid = vec![
            vec!['.', '.', 'S', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '^', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '^', '.', '^', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];
        assert_eq!(part2(&grid), 4);
    }

    #[test]
    fn test_empty_grid() {
        let grid = vec![];
        assert_eq!(part1(&grid), 0);
        assert_eq!(part2(&grid), 0);
    }
}

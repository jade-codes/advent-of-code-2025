use std::{path::Path};

// Add the parent directory to access utils
#[path = "../utils.rs"]
mod utils;

fn directions() -> Vec<(i64, i64)> {
    vec![(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)]
}

fn move_rolls(grid: &mut [Vec<char>]) -> i64 {
    
    let mut total = 0;
    let mut positions_to_clear = Vec::new();
    
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == '@' {
                let mut neighbor_count = 0;
                for dir in directions() {
                    let new_row = row_idx as i64 + dir.0;
                    let new_col = col_idx as i64 + dir.1;
                    if new_row >= 0 && new_col >= 0 
                        && (new_row as usize) < grid.len() 
                        && (new_col as usize) < grid[0].len()
                        && grid[new_row as usize][new_col as usize] == '@' {
                        neighbor_count += 1;
                    }
                }
                if neighbor_count < 4 {
                    positions_to_clear.push((row_idx, col_idx));
                    total += 1;
                }
            }
        }
    }
    
    for (row_idx, col_idx) in positions_to_clear {

        grid[row_idx][col_idx] = '.';
    }

    total
}

fn part1(grid: &mut [Vec<char>]) -> i64 {

    
    move_rolls(grid)
}

fn part2(grid: &mut [Vec<char>]) -> i64 {
    let mut total = 0;

    loop {
        let round_total = move_rolls(grid);
        if round_total == 0 {
            break;
        }
        total += round_total;
    }
    total
}

pub fn main() {
    let input_path = Path::new(file!()).parent().unwrap().join("input.txt");
    let mut grid = utils::read_grid(&input_path);

    println!("Part 1: {}", part1(&mut grid));

    let mut grid = utils::read_grid(&input_path);
    println!("Part 2: {}", part2(&mut grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let mut input = vec![
            vec!['.', '.', '@', '@', '.', '@', '@', '@', '@', '.'],
            vec!['@', '@', '@', '.', '@', '.', '@', '.', '@', '@'],
            vec!['@', '@', '@', '@', '@', '.', '@', '.', '@', '@'],
            vec!['@', '.', '@', '@', '@', '@', '.', '.', '@', '.'],
            vec!['@', '@', '.', '@', '@', '@', '@', '.', '@', '@'],
            vec!['.', '@', '@', '@', '@', '@', '@', '@', '.', '@'],
            vec!['.', '@', '.', '@', '.', '@', '.', '@', '@', '@'],
            vec!['@', '.', '@', '@', '@', '.', '@', '@', '@', '@'],
            vec!['.', '@', '@', '@', '@', '@', '@', '@', '@', '.'],
            vec!['@', '.', '@', '.', '@', '@', '@', '.', '@', '.'],
        ];
        assert_eq!(part1(&mut input), 13);
    }

    #[test]
    fn test_part2_example() {
        let mut input = vec![
            vec!['.', '.', '@', '@', '.', '@', '@', '@', '@', '.'],
            vec!['@', '@', '@', '.', '@', '.', '@', '.', '@', '@'],
            vec!['@', '@', '@', '@', '@', '.', '@', '.', '@', '@'],
            vec!['@', '.', '@', '@', '@', '@', '.', '.', '@', '.'],
            vec!['@', '@', '.', '@', '@', '@', '@', '.', '@', '@'],
            vec!['.', '@', '@', '@', '@', '@', '@', '@', '.', '@'],
            vec!['.', '@', '.', '@', '.', '@', '.', '@', '@', '@'],
            vec!['@', '.', '@', '@', '@', '.', '@', '@', '@', '@'],
            vec!['.', '@', '@', '@', '@', '@', '@', '@', '@', '.'],
            vec!['@', '.', '@', '.', '@', '@', '@', '.', '@', '.'],
        ];
        assert_eq!(part2(&mut input), 43);
    }

    #[test]
    fn test_part1_single_roll() {
        let mut input = vec![
            vec!['.', '.', '.'],
            vec!['.', '@', '.'],
            vec!['.', '.', '.'],
        ];
        assert_eq!(part1(&mut input), 1);
    }

    #[test]
    fn test_part1_four_neighbors() {
        let mut input = vec![
            vec!['.', '@', '.'],
            vec!['@', '@', '@'],
            vec!['.', '@', '.'],
        ];
        // Center has 4 neighbors, so it won't be counted
        assert_eq!(part1(&mut input), 4);
    }

    #[test]
    fn test_part2_iterative_removal() {
        let mut input = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '@', '@', '@', '.'],
            vec!['.', '@', '@', '@', '.'],
            vec!['.', '@', '@', '@', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];
        // All rolls should be removable eventually
        assert_eq!(part2(&mut input), 9);
    }
}

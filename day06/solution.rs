use std::path::Path;

// Add the parent directory to access utils
#[path = "../utils.rs"]
mod utils;

fn parse_operations(line: &str) -> Vec<char> {
    line.split_whitespace()
        .filter_map(|s| s.chars().next())
        .collect()
}

fn parse_numbers(lines: &[String]) -> Vec<Vec<i64>> {
    // Parse each line into a vector of numbers, filtering out lines with no numbers
    let rows: Vec<Vec<i64>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect()
        })
        .filter(|row: &Vec<i64>| !row.is_empty())
        .collect();
    
    let num_cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    
    let mut columns = vec![Vec::new(); num_cols];
    for row in rows {
        for (col_idx, &value) in row.iter().enumerate() {
            columns[col_idx].push(value);
        }
    }
    
    columns
}

fn parse_columns_by_vertical_position(lines: &[String]) -> (Vec<Vec<i64>>, Vec<char>) {
    if lines.is_empty() {
        return (vec![], vec![]);
    }
    
    let ops_line = lines.last().unwrap();
    let number_lines = &lines[..lines.len() - 1];
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    
    let mut columns = Vec::new();
    let mut operations = Vec::new();
    let mut current_group = Vec::new();
    let mut current_op = None;
    
    for pos in 0..max_len {
        let number = number_lines.iter()
            .filter_map(|line| line.chars().nth(pos)?.to_digit(10))
            .fold(None, |acc: Option<i64>, digit| {
                Some(acc.unwrap_or(0) * 10 + digit as i64)
            });
        
        let op_char = ops_line.chars().nth(pos);
        let is_operator = op_char.map_or(false, |c| c == '*' || c == '+');
        
        if is_operator {
            if current_op.is_some() {
                columns.push(current_group.clone());
                operations.push(current_op.unwrap());
                current_group.clear();
            }
            current_op = op_char;
            current_group.extend(number);
        } else if current_op.is_some() {
            current_group.extend(number);
        }
    }

    if let Some(op) = current_op {
        if !current_group.is_empty() {
            columns.push(current_group);
            operations.push(op);
        }
    }
    
    (columns, operations)
}

fn apply_operations(columns: &[Vec<i64>], operations: &[char]) -> i64 {
    let mut result = 0;
    for (column, &op) in columns.iter().zip(operations.iter()) {
        let col_result = match op {
            '*' => column.iter().product::<i64>(),
            '+' => column.iter().sum::<i64>(),
            _ => 0,
        };
        result += col_result;
    }
    result
}

fn part1(lines: &[String]) -> i64 {
    let columns = parse_numbers(lines);
    let operations = parse_operations(lines.last().unwrap());
    apply_operations(&columns, &operations)
}

fn part2(lines: &[String]) -> i64 {
    let (columns, operations) = parse_columns_by_vertical_position(lines);
    apply_operations(&columns, &operations)
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
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        assert_eq!(part1(&input), 4277556);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        assert_eq!(part2(&input), 3263827);
    }
}

use std::{collections::VecDeque, path::Path};

// Add the parent directory to access utils
#[path = "../utils.rs"]
mod utils;

fn parse_ranges(iter: &mut std::slice::Iter<String>) -> Vec<(i64, i64)> {
    let mut ranges = Vec::new();
    for line in iter.by_ref() {
        if line.is_empty() { break; }
        let (a, b) = line.split_once('-').unwrap();
        ranges.push((a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()));
    }
    ranges
}

fn merge_ranges(ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut sorted = ranges;
    sorted.sort_by_key(|&(start, _)| start);
    
    let mut merged: Vec<(i64, i64)> = Vec::new();
    for (start, end) in sorted {
        if let Some((_, last_end)) = merged.last_mut() {
            if start <= *last_end + 1 {
                // Overlapping or adjacent, merge them
                *last_end = (*last_end).max(end);
            } else {
                // No overlap, add new range
                merged.push((start, end));
            }
        } else {
            // First range
            merged.push((start, end));
        }
    }
    merged
}

fn parse_numbers(iter: &mut std::slice::Iter<String>) -> VecDeque<i64> {
    let mut numbers: Vec<i64> = iter
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    
    numbers.sort();
    numbers.into_iter().collect()
}

fn part1(lines: &[String]) -> i64 {
    let mut iter = lines.iter();
    
    let ranges = parse_ranges(&mut iter);
    let merged = merge_ranges(ranges);
    let mut numbers = parse_numbers(&mut iter);

    let mut count: i64 = 0;
    for (start, end) in merged {
        while let Some(&num) = numbers.front() {
            if num < start {
                numbers.pop_front();
            } else if num > end {
                break;
            } else {
                count += 1;
                numbers.pop_front();
            }
        }
    }
    
    count
}

fn part2(lines: &[String]) -> i64 {
    let mut iter = lines.iter();
    
    let ranges = parse_ranges(&mut iter);
    let merged = merge_ranges(ranges);
    let _numbers = parse_numbers(&mut iter);

    let mut total_fresh: i64 = 0;
    for (start, end) in merged {
        let range_size = end - start + 1;
        total_fresh += range_size;
    }
    total_fresh
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
    fn test_part1_example() {
        let input = vec![
            "3-5".to_string(),
            "10-14".to_string(),
            "16-20".to_string(),
            "12-18".to_string(),
            "".to_string(),
            "1".to_string(),
            "5".to_string(),
            "8".to_string(),
            "11".to_string(),
            "17".to_string(),
            "32".to_string(),
        ];
        // Numbers 5, 11, and 17 are in the merged ranges
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part2_example() {
        let input = vec![
            "3-5".to_string(),
            "10-14".to_string(),
            "16-20".to_string(),
            "12-18".to_string(),
            "".to_string(),
            "1".to_string(),
            "5".to_string(),
            "8".to_string(),
            "11".to_string(),
            "17".to_string(),
            "32".to_string(),
        ];
        // Merged ranges: 3-5 (3), 10-20 (11)
        // Total fresh = 3 + 11 = 14
        assert_eq!(part2(&input), 14);
    }

    #[test]
    fn test_merge_ranges() {
        let ranges = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        let merged = merge_ranges(ranges);
        assert_eq!(merged, vec![(3, 5), (10, 20)]);
    }

    #[test]
    fn test_merge_adjacent_ranges() {
        let ranges = vec![(1, 3), (4, 6), (7, 9)];
        let merged = merge_ranges(ranges);
        assert_eq!(merged, vec![(1, 9)]);
    }

    #[test]
    fn test_no_overlap() {
        let ranges = vec![(1, 2), (5, 6), (10, 11)];
        let merged = merge_ranges(ranges);
        assert_eq!(merged, vec![(1, 2), (5, 6), (10, 11)]);
    }

    #[test]
    fn test_part1_no_matches() {
        let input = vec![
            "1-5".to_string(),
            "".to_string(),
            "10".to_string(),
            "20".to_string(),
        ];
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part1_all_matches() {
        let input = vec![
            "1-100".to_string(),
            "".to_string(),
            "5".to_string(),
            "10".to_string(),
            "50".to_string(),
        ];
        assert_eq!(part1(&input), 3);
    }
}
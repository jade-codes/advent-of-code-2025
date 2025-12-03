use std::collections::HashSet;
use std::path::Path;

// Add the parent directory to access utils
#[path = "../utils.rs"]
mod utils;

fn part1(line: &str) -> i64 {
    let ranges: Vec<(String, String)> = line
        .split(',')
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .filter_map(|segment| {
            segment
                .split_once('-')
                .map(|(start, end)| (start.trim().to_string(), end.trim().to_string()))
        })
        .collect();

    let mut total_sum: i64 = 0;

    for (start, end) in ranges {
        let start_value = start.parse::<i64>().unwrap();
        let end_value = end.parse::<i64>().unwrap();
        if start_value > end_value {
            continue;
        }

        let min_len = digit_len(start_value);
        let max_len = digit_len(end_value);

        for len in min_len..=max_len {
            if len % 2 != 0 {
                continue;
            }

            let half_len = len / 2;
            let pow_half = 10_i64.pow(half_len as u32);
            let factor = pow_half + 1;
            let half_min = pow_half / 10;
            let half_max = pow_half - 1;

            let lower_bound = half_min.max((start_value + factor - 1) / factor);
            let upper_bound = half_max.min(end_value / factor);

            if lower_bound > upper_bound {
                continue;
            }

            for half in lower_bound..=upper_bound {
                let candidate = half * factor;
                if candidate >= start_value && candidate <= end_value {
                    total_sum += candidate;
                }
            }
        }
    }

    total_sum
}

fn part2(line: &str) -> i64 {
    let ranges: Vec<(String, String)> = line
        .split(',')
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .filter_map(|segment| {
            segment
                .split_once('-')
                .map(|(start, end)| (start.trim().to_string(), end.trim().to_string()))
        })
        .collect();

    let mut total_sum: i64 = 0;

    for (start, end) in ranges {
        let start_value = start.parse::<i64>().unwrap();
        let end_value = end.parse::<i64>().unwrap();
        if start_value > end_value {
            continue;
        }

        let min_len = digit_len(start_value);
        let max_len = digit_len(end_value);
        let mut seen = HashSet::new();

        for len in min_len..=max_len {
            if len < 2 {
                continue;
            }

            let pow_len_minus_one = 10_i64.pow((len - 1) as u32);
            let pow_len = 10_i64.pow(len as u32);

            let range_start = start_value.max(pow_len_minus_one);
            let range_end = end_value.min(pow_len - 1);
            if range_start > range_end {
                continue;
            }

            for block_len in 1..=len / 2 {
                if len % block_len != 0 {
                    continue;
                }

                let repeat_count = len / block_len;
                if repeat_count < 2 {
                    continue;
                }

                let pow_block = 10_i64.pow(block_len as u32);
                let block_lower_limit = 10_i64.pow((block_len - 1) as u32);
                let block_upper_limit = pow_block - 1;

                let mut factor = 0_i64;
                for _ in 0..repeat_count {
                    factor = factor
                        .checked_mul(pow_block)
                        .and_then(|v| v.checked_add(1))
                        .expect("repeat factor overflow");
                }

                let block_min = block_lower_limit.max((range_start + factor - 1) / factor);
                let block_max = block_upper_limit.min(range_end / factor);

                if block_min > block_max {
                    continue;
                }

                for block in block_min..=block_max {
                    if is_repetition(block) {
                        continue;
                    }

                    let candidate = block * factor;

                    if candidate >= range_start && candidate <= range_end && seen.insert(candidate)
                    {
                        total_sum += candidate;
                    }
                }
            }
        }
    }

    total_sum
}

fn digit_len(value: i64) -> usize {
    if value == 0 {
        return 1;
    }

    (value.abs().ilog10() + 1) as usize
}

fn is_repetition(value: i64) -> bool {
    let s = value.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();

    for size in 1..=len / 2 {
        if len % size != 0 {
            continue;
        }

        let pattern = &bytes[..size];
        if bytes.chunks(size).all(|chunk| chunk == pattern) {
            return true;
        }
    }

    false
}

pub fn main() {
    let input_path = Path::new(file!()).parent().unwrap().join("input.txt");
    let line = utils::read_input(&input_path);

    println!("Part 1: {}", part1(&line));
    println!("Part 2: {}", part2(&line));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part1(input), 1_227_775_554);
    }

    #[test]
    fn test_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part2(input), 4_174_379_265);
    }
}

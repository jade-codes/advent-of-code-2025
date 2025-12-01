use std::path::Path;

// Add the parent directory to access utils
#[path = "../utils.rs"]
mod utils;

fn part1(lines: &[String]) -> i64 {
    let mut password: i64 = 0;
    let mut actual_position: i64 = 50;
    let modulo = 100;

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let direction = chars[0];
        let num_str = &line[1..];
        let num: i64 = num_str.parse().unwrap();

        if direction == 'L' {
            actual_position -= num;
        } else {
            actual_position += num;
        }

        actual_position = actual_position.rem_euclid(modulo);

        if actual_position == 0 {
            password += 1;
        }
    }

    password
}


fn part2(lines: &[String]) -> i64 {
    let mut password: i64 = 0;
    let mut actual_position: i64 = 50;

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let direction = chars[0];
        let num_str = &line[1..];
        let num: i64 = num_str.parse().unwrap();
        let modulo = 100;

        let old_pos = actual_position;
        
        if direction == 'L' {
            actual_position -= num;
        } else {
            actual_position += num;
        }
        
        let crosses = if old_pos != actual_position {
            let old = old_pos;
            let new = actual_position;

            if old < new {
                let first = (old.div_euclid(modulo) + 1) * modulo;
                let last = new.div_euclid(modulo) * modulo;
                if first <= new {
                    (last - first) / modulo + 1
                } else {
                    0
                }
            } else {
                let first = if new.rem_euclid(modulo) == 0 {
                    new
                } else {
                    (new.div_euclid(modulo) + 1) * modulo
                };
                let last = ((old - 1).div_euclid(modulo)) * modulo;
                if first <= last {
                    (last - first) / modulo + 1
                } else {
                    0
                }
            }
        } else {
            0
        };
        
        password += crosses;
    }
    
    password
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
            "L68".to_string(),
            "L30".to_string(),
            "R48".to_string(),
            "L5".to_string(),
            "R60".to_string(),
            "L55".to_string(),
            "L1".to_string(),
            "L99".to_string(),
            "R14".to_string(),
            "L82".to_string(),
        ];
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "L68".to_string(),
            "L30".to_string(),
            "R48".to_string(),
            "L5".to_string(),
            "R60".to_string(),
            "L55".to_string(),
            "L1".to_string(),
            "L99".to_string(),
            "R14".to_string(),
            "L82".to_string(),
        ];
        assert_eq!(part2(&input), 6);
    }

    #[test]
    fn test_part2_exact_100() {
        let input = vec![
            "R100".to_string(),
            "L100".to_string(),
            "L100".to_string(),
            "R200".to_string(),
        ];
        assert_eq!(part2(&input), 5);
    }

    #[test]
    fn test_part2_multiple_crossings() {
        let input = vec![
            "R250".to_string(),
            "L500".to_string(),
        ];
        assert_eq!(part2(&input), 8);
    }

    #[test]
    fn test_part2_landing_on_zero() {
        let input = vec![
            "L50".to_string(),
            "R100".to_string(),
            "L200".to_string(),
        ];
        assert_eq!(part2(&input), 4);
    }

    #[test]
    fn test_part2_zero_to_zero() {
        let input = vec![
            "L50".to_string(),
            "R100".to_string(),
            "L100".to_string(),
            "L100".to_string(),
        ];
        assert_eq!(part2(&input), 4);
    }
}

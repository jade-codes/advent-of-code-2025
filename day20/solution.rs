use std::path::Path;

// Add the parent directory to access utils
#[path = "../utils.rs"]
mod utils;

fn part1(_lines: &[String]) -> i64 {
    // TODO: Implement part 1
    0
}

fn part2(_lines: &[String]) -> i64 {
    // TODO: Implement part 2
    0
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
        let input = vec![];
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part2() {
        let input = vec![];
        assert_eq!(part2(&input), 0);
    }
}

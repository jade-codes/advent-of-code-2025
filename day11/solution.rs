use std::collections::HashMap;
use std::path::Path;

// Add the parent directory to access utils
#[path = "../utils.rs"]
mod utils;

fn parse_to_hashmap(input: &[String]) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    
    for line in input {
        if let Some((key, values)) = line.split_once(':') {
            let key = key.trim().to_string();
            
            let values: Vec<String> = values
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            
            map.insert(key, values);
        }
    }
    
    map
}

fn count_paths_to_out(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    required: &[String],
    seen_mask: u32,
    target_mask: u32,
    memo: &mut HashMap<(String, u32), i64>
) -> i64 {
    let current_mask = required.iter().enumerate()
        .find(|(_, req)| req.as_str() == node)
        .map(|(i, _)| seen_mask | (1 << i))
        .unwrap_or(seen_mask);
    
    if node == "out" {
        return if current_mask == target_mask { 1 } else { 0 };
    }
    
    let key = (node.to_string(), current_mask);
    
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }
    
    let total = graph.get(node)
        .map(|neighbors| {
            neighbors.iter()
                .map(|next| {
                    count_paths_to_out(next, graph, required, current_mask, target_mask, memo)
                })
                .sum()
        })
        .unwrap_or(0);
    
    memo.insert(key, total);
    total
}

fn part1(lines: &[String]) -> i64 {
    
    let graph = parse_to_hashmap(lines);
    let required = vec![];
    let mut memo = HashMap::new();
    count_paths_to_out("you", &graph, &required, 0, 0, &mut memo)
}

fn part2(lines: &[String]) -> i64 {
    let graph = parse_to_hashmap(lines);
    let required = vec!["fft".to_string(), "dac".to_string()];
    let target_mask = (1 << required.len()) - 1; 
    let mut memo = HashMap::new();
    count_paths_to_out("svr", &graph, &required, 0, target_mask, &mut memo)
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
            "you: aaa bbb".to_string(),
            "aaa: fft".to_string(),
            "fft: ccc".to_string(),
            "bbb: tty".to_string(),
            "tty: ccc".to_string(),
            "ccc: ddd eee".to_string(),
            "ddd: hub".to_string(),
            "hub: fff".to_string(),
            "eee: dac".to_string(),
            "dac: fff".to_string(),
            "fff: ggg hhh".to_string(),
            "ggg: out".to_string(),
            "hhh: out".to_string(),
        ];
        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "svr: aaa bbb".to_string(),
            "aaa: fft".to_string(),
            "fft: ccc".to_string(),
            "bbb: tty".to_string(),
            "tty: ccc".to_string(),
            "ccc: ddd eee".to_string(),
            "ddd: hub".to_string(),
            "hub: fff".to_string(),
            "eee: dac".to_string(),
            "dac: fff".to_string(),
            "fff: ggg hhh".to_string(),
            "ggg: out".to_string(),
            "hhh: out".to_string(),
        ];
        assert_eq!(part2(&input), 2);
    }
}

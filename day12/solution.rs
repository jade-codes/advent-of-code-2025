use std::path::Path;
use std::collections::HashMap;

// Add the parent directory to access utils
#[path = "../utils.rs"]
mod utils;

#[derive(Debug)]
struct GridSpec {
    width: usize,
    height: usize,
    pattern_counts: Vec<(usize, usize)>
}

fn parse_input(lines: &[String]) -> (HashMap<usize, usize>, Vec<GridSpec>) {
    let mut pattern_cells = HashMap::new();
    let mut grid_specs = Vec::new();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i].trim();
        
        if let Some(pattern_id) = line.strip_suffix(':').and_then(|s| s.parse().ok()) {
            let cell_count: usize = lines[i+1..].iter().take(3)
                .flat_map(|line| line.chars())
                .filter(|&c| c == '#')
                .count();
            pattern_cells.insert(pattern_id, cell_count);
            i += 4;
            continue;
        }
        
        if let Some((dims, counts_str)) = line.split_once(':') {
            if let Some((w, h)) = dims.split_once('x') {
                if let (Ok(width), Ok(height)) = (w.parse(), h.parse()) {
                    let pattern_counts: Vec<_> = counts_str
                        .split_whitespace()
                        .enumerate()
                        .filter_map(|(id, s)| s.parse().ok().filter(|&count| count > 0).map(|count| (id, count)))
                        .collect();
                    
                    grid_specs.push(GridSpec { width, height, pattern_counts });
                }
            }
        }
        
        i += 1;
    }
    
    (pattern_cells, grid_specs)
}

fn solve_grid(
    grid_width: usize,
    grid_height: usize,
    pattern_cells: &HashMap<usize, usize>,
    pattern_counts: &[(usize, usize)],
) -> i64 {
    let num_cells = grid_width * grid_height;
    
    let total_pattern_cells: usize = pattern_counts
        .iter()
        .map(|(pid, count)| pattern_cells[pid] * count)
        .sum();
    
    if total_pattern_cells > num_cells {
        return 0; 
    }
    
    let usage = total_pattern_cells as f64 / num_cells as f64;
    if usage < 0.85 { 1 } else { 0 }
}

fn part1(_lines: &[String]) -> i64 {
    let (pattern_cells, grid_specs) = parse_input(_lines);
    
    grid_specs
        .iter()
        .map(|spec| solve_grid(spec.width, spec.height, &pattern_cells, &spec.pattern_counts))
        .sum()
}

pub fn main() {
    let input_path = Path::new(file!()).parent().unwrap().join("input.txt");
    let lines = utils::read_lines(&input_path);

    println!("Part 1: {}", part1(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input_path = Path::new(file!()).parent().unwrap().join("input.txt");
        let lines = utils::read_lines(&input_path);
        
        let result = part1(&lines);
        assert_eq!(result, 403);
    }

    #[test]
    fn test_part_1_small_example() {
        let lines = vec![
            "0:".to_string(),
            "###".to_string(),
            "##.".to_string(),
            "##.".to_string(),
            "".to_string(),
            "1:".to_string(),
            "###".to_string(),
            "##.".to_string(),
            ".##".to_string(),
            "".to_string(),
            "2:".to_string(),
            ".##".to_string(),
            "###".to_string(),
            "##.".to_string(),
            "".to_string(),
            "3:".to_string(),
            "##.".to_string(),
            "###".to_string(),
            "##.".to_string(),
            "".to_string(),
            "4:".to_string(),
            "###".to_string(),
            "#..".to_string(),
            "###".to_string(),
            "".to_string(),
            "5:".to_string(),
            "###".to_string(),
            ".#.".to_string(),
            "###".to_string(),
            "".to_string(),
            "4x4: 0 0 0 0 2 0".to_string(),
            "12x5: 1 0 1 0 2 2".to_string(),
            "12x5: 1 0 1 0 3 2".to_string(),
        ];
        
        let result = part1(&lines);
        assert_eq!(result, 2);
    }
}

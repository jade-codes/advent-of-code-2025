use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Read input file and return a vector of lines
/// Returns an empty vector if the file doesn't exist or is empty
pub fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);
    reader
        .lines()
        .map_while(Result::ok)
        .collect()
}

/// Read input file and return a 2D grid of characters
#[allow(dead_code)]
pub fn read_grid<P>(filename: P) -> Vec<Vec<char>>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename);
    lines.iter().map(|line| line.chars().collect()).collect()
}

/// Read input file and return a vector of numbers (one per line)
#[allow(dead_code)]
pub fn read_numbers<P>(filename: P) -> Vec<i64>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename);
    lines
        .iter()
        .map(|line| line.parse::<i64>().expect("Unable to parse number"))
        .collect()
}

/// Read input file as a single string
/// Returns an empty string if the file doesn't exist
#[allow(dead_code)]
pub fn read_input<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    std::fs::read_to_string(filename).unwrap_or_default()
}

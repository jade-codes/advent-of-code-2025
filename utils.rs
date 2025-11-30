use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Read input file and return a vector of lines
pub fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("Unable to read line"))
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
#[allow(dead_code)]
pub fn read_input<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    std::fs::read_to_string(filename).expect("Unable to read file")
}

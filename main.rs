use std::time::Instant;

// Include all day modules
#[path = "day01/solution.rs"]
mod day01;
#[path = "day02/solution.rs"]
mod day02;
#[path = "day03/solution.rs"]
mod day03;
#[path = "day04/solution.rs"]
mod day04;
#[path = "day05/solution.rs"]
mod day05;
#[path = "day06/solution.rs"]
mod day06;
#[path = "day07/solution.rs"]
mod day07;
#[path = "day08/solution.rs"]
mod day08;
#[path = "day09/solution.rs"]
mod day09;
#[path = "day10/solution.rs"]
mod day10;
#[path = "day11/solution.rs"]
mod day11;
#[path = "day12/solution.rs"]
mod day12;

fn run_day(day: u8, runner: fn()) {
    println!("\n{:=<50}", "");
    println!("Day {:02}", day);
    println!("{:-<50}", "");
    
    let start = Instant::now();
    runner();
    let duration = start.elapsed();
    
    println!("{:-<50}", "");
    println!("Time: {:?}", duration);
    println!("{:=<50}", "");
}

fn main() {
    println!("\n🎄 Advent of Code 2025 🎄\n");
    
    let total_start = Instant::now();
    
    run_day(1, day01::main);
    run_day(2, day02::main);
    run_day(3, day03::main);
    run_day(4, day04::main);
    run_day(5, day05::main);
    run_day(6, day06::main);
    run_day(7, day07::main);
    run_day(8, day08::main);
    run_day(9, day09::main);
    run_day(10, day10::main);
    run_day(11, day11::main);
    run_day(12, day12::main);
    
    let total_duration = total_start.elapsed();
    
    println!("\n{:=<50}", "");
    println!("Total time: {:?}", total_duration);
    println!("{:=<50}", "");
}

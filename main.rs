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
#[path = "day13/solution.rs"]
mod day13;
#[path = "day14/solution.rs"]
mod day14;
#[path = "day15/solution.rs"]
mod day15;
#[path = "day16/solution.rs"]
mod day16;
#[path = "day17/solution.rs"]
mod day17;
#[path = "day18/solution.rs"]
mod day18;
#[path = "day19/solution.rs"]
mod day19;
#[path = "day20/solution.rs"]
mod day20;
#[path = "day21/solution.rs"]
mod day21;
#[path = "day22/solution.rs"]
mod day22;
#[path = "day23/solution.rs"]
mod day23;
#[path = "day24/solution.rs"]
mod day24;
#[path = "day25/solution.rs"]
mod day25;

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
    run_day(13, day13::main);
    run_day(14, day14::main);
    run_day(15, day15::main);
    run_day(16, day16::main);
    run_day(17, day17::main);
    run_day(18, day18::main);
    run_day(19, day19::main);
    run_day(20, day20::main);
    run_day(21, day21::main);
    run_day(22, day22::main);
    run_day(23, day23::main);
    run_day(24, day24::main);
    run_day(25, day25::main);
    
    let total_duration = total_start.elapsed();
    
    println!("\n{:=<50}", "");
    println!("Total time: {:?}", total_duration);
    println!("{:=<50}", "");
}

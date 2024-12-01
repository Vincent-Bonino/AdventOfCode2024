use std::process::exit;

use clap::Parser;

use aoc24::aoc::Aoc24Solution;
use aoc24::cli::Args;

use aoc24::day00::Day00;

fn main() {
    println!("Advent of Code 2024!\n");
    let args: Args = Args::parse();
    let target_day: usize = args.day;

    if target_day > 25 {
        println!("Are you joking ?");
        exit(1)
    }

    // Create index of solutions
    let mut solutions: Vec<Box<dyn Aoc24Solution>> = vec![
        Box::new(Day00::default()),
    ];

    // Find the right solution
    let mut found: bool = false;

    for sol in solutions.iter_mut() {
        if sol.get_day_number() != args.day {
            continue;
        }

        let part_one: i128 = sol.solve_part_one(args.use_test);
        let part_two: i128 = if part_one > 0 {
            sol.solve_part_two(args.use_test)
        } else {
            -2
        };

        println!("[Day {target_day:0>2}] Part 1: {part_one}");
        println!("[Day {target_day:0>2}] Part 2: {part_two}");

        found = true;
        break;
    }

    if !found {
        println!("No solution for day {target_day:0>2}");
        exit(1)
    }

    exit(0)
}
use std::process::exit;

use clap::Parser;
use colored::Colorize;
#[cfg(feature = "benchmark")]
use instant::Instant;

use aoc24::aoc::Aoc24Solution;
use aoc24::cli::Args;

use aoc24::day00::Day00;
use aoc24::day01::Day01;
use aoc24::day02::Day02;
use aoc24::day03::Day03;
use aoc24::day04::Day04;
use aoc24::day05::Day05;
use aoc24::day06::Day06;
use aoc24::day07::Day07;
use aoc24::day08::Day08;
use aoc24::day09::Day09;
use aoc24::day10::Day10;
use aoc24::day11::Day11;
use aoc24::day12::Day12;
use aoc24::day13::Day13;
use aoc24::day14::Day14;
use aoc24::day15::Day15;
use aoc24::day16::Day16;
use aoc24::day17::Day17;
use aoc24::day18::Day18;
use aoc24::day19::Day19;
use aoc24::day20::Day20;
use aoc24::day22::Day22;
use aoc24::day23::Day23;
use aoc24::day24::Day24;
use aoc24::day25::Day25;

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
        Box::new(Day01::default()),
        Box::new(Day02::default()),
        Box::new(Day03::default()),
        Box::new(Day04::default()),
        Box::new(Day05::default()),
        Box::new(Day06::default()),
        Box::new(Day07::default()),
        Box::new(Day08::default()),
        Box::new(Day09::default()),
        Box::new(Day10::default()),
        Box::new(Day11::default()),
        Box::new(Day12::default()),
        Box::new(Day13::default()),
        Box::new(Day14::default()),
        Box::new(Day15::default()),
        Box::new(Day16::default()),
        Box::new(Day17::default()),
        Box::new(Day18::default()),
        Box::new(Day19::default()),
        Box::new(Day20::default()),
        Box::new(Day22::default()),
        Box::new(Day23::default()),
        Box::new(Day24::default()),
        Box::new(Day25::default()),
    ];

    // Find the right solution
    let mut found: bool = false;

    for sol in solutions.iter_mut() {
        if sol.get_day_number() != args.day {
            continue;
        }

        #[cfg(feature = "benchmark")]
        {
            // Run part 1
            let now = Instant::now();
            let part_one: i128 = sol.solve_part_one(args.use_test);
            let part_01_time = now.elapsed().as_micros();

            println!(
                "[Day {target_day:0>2}] Part 1: {} in {part_01_time} us",
                part_one.to_string().cyan()
            );

            // Run part 2
            if part_one != -1 {
                let now = Instant::now();
                let part_two: i128 = sol.solve_part_two(args.use_test);
                let part_02_time = now.elapsed().as_micros();

                if args.day <= 25 {
                    println!(
                        "[Day {target_day:0>2}] Part 2: {} in {part_02_time} us",
                        part_two.to_string().cyan()
                    );
                } else {
                    println!("Merry Christmas !");
                }
            } else {
                println!("No part two yet");
            };
        }

        #[cfg(not(feature = "benchmark"))]
        {
            let part_one: i128 = sol.solve_part_one(args.use_test);
            println!(
                "[Day {target_day:0>2}] Part 1: {}",
                part_one.to_string().cyan()
            );

            if part_one != -1 {
                if args.day < 25 {
                    let part_two: i128 = sol.solve_part_two(args.use_test);
                    println!(
                        "[Day {target_day:0>2}] Part 2: {}",
                        part_two.to_string().cyan()
                    );
                } else {
                    println!("Merry Christmas !");
                }
            } else {
                println!("No part two yet");
            };
        }

        found = true;
        break;
    }

    if !found {
        println!("No solution for day {target_day:0>2}");
        exit(1)
    }

    exit(0)
}

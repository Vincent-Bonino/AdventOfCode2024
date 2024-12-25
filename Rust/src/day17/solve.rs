use rayon::prelude::*;
use std::collections::VecDeque;

use crate::day17::model::Computer;

pub fn solve_part_one(computer: &Computer) -> i128 {
    let mut computer: Computer = computer.clone();

    // Run the program
    let stdout: Vec<u8> = computer.run();
    let stdout_str: String = Computer::format_stdout(&stdout);

    println!("Computer's output is:");
    println!("{stdout_str}\n");

    // Return dummy value
    1234
}

mod instructions;
mod keypads;
mod model;
mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day21::parse::parse_input;
use crate::day21::solve::solve_part_one;

#[derive(Default)]
pub struct Day21 {
    codes: Vec<String>,
}

impl Aoc24Solution for Day21 {
    fn get_day_number(&self) -> usize {
        21
    }

    // 115004 too high
    // 108670 too high
    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        self.codes = parse_input(&self.get_data(is_test));
        solve_part_one(&self.codes) as i128
    }
}

use crate::toolbox::Direction;
use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

pub enum Instruction {
    Up,
    Right,
    Down,
    Left,
    Press,
}

impl Instruction {
    pub fn from_direction(direction: &Direction) -> Self {
        match direction {
            Direction::N => Instruction::Up,
            Direction::E => Instruction::Right,
            Direction::S => Instruction::Down,
            Direction::W => Instruction::Left,
            _ => unimplemented!(),
        }
    }

    pub fn to_direction(&self) -> Direction {
        match self {
            Instruction::Up => Direction::N,
            Instruction::Right => Direction::E,
            Instruction::Down => Direction::S,
            Instruction::Left => Direction::W,
            _ => unimplemented!(),
        }
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Up => write!(f, "^"),
            Instruction::Right => write!(f, ">"),
            Instruction::Down => write!(f, "v"),
            Instruction::Left => write!(f, "<"),
            Instruction::Press => write!(f, "A"),
        }
    }
}

pub fn instructions_to_code(instructions: &Vec<Instruction>) -> String {
    instructions
        .iter()
        .map(|instr| format!("{instr:?}"))
        .join("")
}

#[allow(dead_code)]
pub fn instructions_to_debug_code(instructions: &Vec<Instruction>) -> String {
    instructions
        .iter()
        .map(|instr| format!("{instr:?}"))
        .join("")
        .split("A")
        .join("A ")
}

#[allow(dead_code)]
pub fn parse_instructions_str(instructions_str: &str) -> Vec<Instruction> {
    instructions_str
        .chars()
        .map(|chr| match chr {
            '^' => Instruction::Up,
            '>' => Instruction::Right,
            'v' => Instruction::Down,
            '<' => Instruction::Left,
            'A' => Instruction::Press,
            _ => unreachable!("Unknown instruction str '{chr}'"),
        })
        .collect()
}

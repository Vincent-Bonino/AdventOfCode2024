use std::collections::HashMap;

use itertools::Itertools;
use once_cell::sync::Lazy;

use crate::day21::instructions::{instructions_to_code, Instruction};
use crate::toolbox::iterators::all_combinaisons_of_two;
use crate::toolbox::{Coordinates, Direction};

static DIRECTION_TILES_ORDER: Lazy<HashMap<Direction, u8>> = Lazy::new(|| {
    HashMap::from([
        (Direction::N, 2),
        (Direction::E, 1),
        (Direction::S, 2),
        (Direction::W, 3),
    ])
});

pub trait KeyPad {
    // Simple getters

    fn get_starting_position(&self) -> &Coordinates;
    fn get_empty_position(&self) -> &Coordinates;
    fn get_symbol_at(&self, coordinates: &Coordinates) -> char;
    fn get_coord_with(&self, symbol: &char) -> &Coordinates;
    fn get_directions(&self, from: char, to: char) -> &Vec<Direction>;

    // Intermediate computation

    fn build_directions_cache(&mut self);

    // Must now sort directions (will become movement instructions).
    // Two criteria:
    //  1. Should be short to input at next step: closer directions to 'A' first
    //  2. Should not go over the empty tile
    fn sort_directions(&self, mut directions: Vec<Direction>, from: &char) -> Vec<Direction> {
        // 1.
        directions.sort_unstable_by_key(|dir| DIRECTION_TILES_ORDER[dir]);

        // 2.
        let mut current_coord: Coordinates = *self.get_coord_with(from);
        for (index, dir) in directions.iter().enumerate() {
            current_coord = current_coord.step(dir);

            if current_coord == *self.get_empty_position() {
                let len: usize = directions.len() - 1;
                directions.swap(index, len);
                break;
            }
        }

        directions
    }

    // Code → instructions & instructions → code

    /// From a given code, build a sequence of instructions to type it.
    fn build_instructions_for_code(&self, code: &str) -> Vec<Instruction> {
        let mut result: Vec<Instruction> = Vec::new();
        for (prev, next) in format!("A{code}").chars().tuple_windows() {
            result.extend(
                self.get_directions(prev, next)
                    .iter()
                    .map(|dir| Instruction::from_direction(dir)),
            );
            result.push(Instruction::Press)
        }
        result
    }

    /// From a given sequence of instructions, build the code typed.
    fn build_code_from_instructions(&self, instructions: &[Instruction]) -> String {
        let mut result: String = String::new();
        let mut pointing_coordinate: Coordinates = *self.get_starting_position();

        for instr in instructions.iter() {
            match instr {
                Instruction::Press => result.push(self.get_symbol_at(&pointing_coordinate)),
                _ => pointing_coordinate = pointing_coordinate.step(&instr.to_direction()),
            }
        }

        result
    }
}

/// Numerical keypad layout:
/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
///     | 0 | A |
///     +---+---+
#[derive(Debug)]
pub struct NumericKeypad {
    symbol_to_position: HashMap<char, Coordinates>,
    position_to_symbol: HashMap<Coordinates, char>,

    empty_tile: Coordinates,

    directions_cache: HashMap<(char, char), Vec<Direction>>,
}

impl NumericKeypad {
    pub fn new() -> Self {
        let symbol_to_position: HashMap<char, Coordinates> = HashMap::from([
            ('0', Coordinates { x: 3, y: 1 }),
            ('1', Coordinates { x: 2, y: 0 }),
            ('2', Coordinates { x: 2, y: 1 }),
            ('3', Coordinates { x: 2, y: 2 }),
            ('4', Coordinates { x: 1, y: 0 }),
            ('5', Coordinates { x: 1, y: 1 }),
            ('6', Coordinates { x: 1, y: 2 }),
            ('7', Coordinates { x: 0, y: 0 }),
            ('8', Coordinates { x: 0, y: 1 }),
            ('9', Coordinates { x: 0, y: 2 }),
            ('A', Coordinates { x: 3, y: 2 }),
        ]);
        let position_to_symbol: HashMap<Coordinates, char> = HashMap::from([
            (Coordinates { x: 3, y: 1 }, '0'),
            (Coordinates { x: 2, y: 0 }, '1'),
            (Coordinates { x: 2, y: 1 }, '2'),
            (Coordinates { x: 2, y: 2 }, '3'),
            (Coordinates { x: 1, y: 0 }, '4'),
            (Coordinates { x: 1, y: 1 }, '5'),
            (Coordinates { x: 1, y: 2 }, '6'),
            (Coordinates { x: 0, y: 0 }, '7'),
            (Coordinates { x: 0, y: 1 }, '8'),
            (Coordinates { x: 0, y: 2 }, '9'),
            (Coordinates { x: 3, y: 2 }, 'A'),
        ]);

        let mut new_self: Self = Self {
            symbol_to_position,
            position_to_symbol,
            empty_tile: Coordinates { x: 3, y: 0 },
            directions_cache: HashMap::new(),
        };
        new_self.build_directions_cache();
        new_self
    }
}

impl KeyPad for NumericKeypad {
    fn get_starting_position(&self) -> &Coordinates {
        &self.symbol_to_position[&'A']
    }

    fn get_empty_position(&self) -> &Coordinates {
        &self.empty_tile
    }

    fn get_symbol_at(&self, coordinates: &Coordinates) -> char {
        self.position_to_symbol[coordinates]
    }

    fn get_coord_with(&self, symbol: &char) -> &Coordinates {
        &self.symbol_to_position[symbol]
    }

    fn get_directions(&self, from: char, to: char) -> &Vec<Direction> {
        &self.directions_cache[&(from, to)]
    }

    fn build_directions_cache(&mut self) {
        for (src, src_coord) in self.symbol_to_position.iter() {
            for (dst, dst_coord) in self.symbol_to_position.iter() {
                self.directions_cache.insert(
                    (*src, *dst),
                    self.sort_directions(src_coord.build_directions_to(dst_coord), src),
                );
            }
        }
    }
}

/// Direction keypad layout:
///     +---+---+
///     | ^ | A |
/// +---+---+---+
/// | < | v | > |
/// +---+---+---+
#[derive(Debug)]
pub struct DirectionalKeypad {
    symbol_to_position: HashMap<char, Coordinates>,
    position_to_symbol: HashMap<Coordinates, char>,

    empty_tile: Coordinates,

    directions_cache: HashMap<(char, char), Vec<Direction>>,
}

impl DirectionalKeypad {
    pub fn new() -> Self {
        let symbol_to_position: HashMap<char, Coordinates> = HashMap::from([
            ('^', Coordinates { x: 0, y: 1 }),
            ('>', Coordinates { x: 1, y: 2 }),
            ('v', Coordinates { x: 1, y: 1 }),
            ('<', Coordinates { x: 1, y: 0 }),
            ('A', Coordinates { x: 0, y: 2 }),
        ]);
        let position_to_symbol: HashMap<Coordinates, char> = HashMap::from([
            (Coordinates { x: 0, y: 1 }, '^'),
            (Coordinates { x: 1, y: 2 }, '>'),
            (Coordinates { x: 1, y: 1 }, 'v'),
            (Coordinates { x: 1, y: 0 }, '<'),
            (Coordinates { x: 0, y: 2 }, 'A'),
        ]);

        let mut directions_cache: HashMap<(char, char), Vec<Direction>> = HashMap::new();
        for (src, src_coord) in symbol_to_position.iter() {
            for (dst, dst_coord) in symbol_to_position.iter() {
                directions_cache.insert((*src, *dst), src_coord.build_directions_to(dst_coord));
            }
        }

        let mut new_self: Self = Self {
            symbol_to_position,
            position_to_symbol,
            empty_tile: Coordinates { x: 0, y: 0 },
            directions_cache,
        };
        new_self.build_directions_cache();
        new_self
    }
}

impl KeyPad for DirectionalKeypad {
    fn get_starting_position(&self) -> &Coordinates {
        &self.symbol_to_position[&'A']
    }

    fn get_empty_position(&self) -> &Coordinates {
        &self.empty_tile
    }

    fn get_symbol_at(&self, coordinates: &Coordinates) -> char {
        self.position_to_symbol[coordinates]
    }

    fn get_coord_with(&self, symbol: &char) -> &Coordinates {
        &self.symbol_to_position[symbol]
    }

    fn get_directions(&self, from: char, to: char) -> &Vec<Direction> {
        &self.directions_cache[&(from, to)]
    }

    fn build_directions_cache(&mut self) {
        for (src, src_coord) in self.symbol_to_position.iter() {
            for (dst, dst_coord) in self.symbol_to_position.iter() {
                self.directions_cache.insert(
                    (*src, *dst),
                    self.sort_directions(src_coord.build_directions_to(dst_coord), src),
                );
            }
        }
    }
}

// pub fn find_best_path_in(
//     keypad: &impl KeyPad,
//     from: &Coordinates,
//     to: &Coordinates,
//     time: i32,
// ) -> Vec<Direction> {
//     // Compute all possible paths
//     let x_diff: i32 = to.x - from.x;
//     let y_diff: i32 = to.y - from.y;
//
//     let x_dir: Direction = if x_diff > 0 {
//         Direction::S
//     } else {
//         Direction::N
//     };
//     let y_dir: Direction = if y_diff > 0 {
//         Direction::E
//     } else {
//         Direction::W
//     };
//
//     let all_possible_paths: Vec<Vec<Direction>> =
//         all_combinaisons_of_two(x_dir, x_diff as usize, y_dir, y_diff as usize);
//
//     // Filter out paths walking on empty tile
//     let all_walkable_paths: Vec<Vec<Direction>> = all_possible_paths
//         .into_iter()
//         .filter(|directions| {
//             let mut current_position: Coordinates = *from;
//             let to_avoid: Coordinates = *keypad.get_empty_position();
//
//             for dir in directions {
//                 current_position = current_position.step(dir);
//                 if current_position == to_avoid {
//                     return false;
//                 }
//             }
//
//             true
//         })
//         .collect();
//
//     // Compute their length after {time} keypads
//     let sorted_walkable_paths: Vec<Vec<Direction>> = all_walkable_paths
//         .into_iter()
//         .sorted_by_key(|directions| {
//             let instructions: Vec<Instruction> = directions
//                 .iter()
//                 .map(|dir| Instruction::from_direction(dir))
//                 .collect();
//             instructions_after_keypads(keypad, &instructions_to_code(&instructions), time)
//         })
//         .collect();
//
//     sorted_walkable_paths.first().unwrap().clone()
// }
//
// pub fn instructions_after_keypads(
//     keypad: &impl KeyPad,
//     base_instruction_str: &str,
//     keypad_number: i32,
// ) -> usize {
//     let mut instructions_string: String = base_instruction_str.to_string();
//
//     for _ in 0..keypad_number {
//         let instructions: Vec<Instruction> =
//             keypad.build_instructions_for_code(base_instruction_str);
//         instructions_string = instructions_to_code(&instructions);
//     }
//
//     instructions_string.len()
// }

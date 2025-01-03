use crate::toolbox::coordinates::Coordinates;
use colored::Colorize;
use faer::sparse::ops::sub;
use itertools::Itertools;
use std::fmt::{format, Debug};

/// Grid of data.
///
/// Orientation:
/// +--------> y  (width)
/// |
/// |
/// V
/// x (depth)
#[derive(Clone, Debug, Default)]
pub struct Grid<T> {
    pub(crate) data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }

    /// Get the depth of the grid (length on x-axis / number of sub-arrays)
    pub fn depth(&self) -> usize {
        self.data.len()
    }

    /// Get the width of the grid (length on y-axis / length of sub-arrays)
    pub fn width(&self) -> usize {
        self.data.first().expect("Empty grid").len()
    }

    pub fn is_in_bounds(&self, coordinates: &Coordinates) -> bool {
        0 <= coordinates.x
            && (coordinates.x as usize) < self.depth()
            && 0 <= coordinates.y
            && (coordinates.y as usize) < self.width()
    }

    /// Replace a value in the data.
    ///
    /// Returns `true` if the change occurred, `false` otherwise.
    pub fn replace(&mut self, coordinates: &Coordinates, value: T) -> bool {
        if !self.is_in_bounds(coordinates) {
            return false;
        }

        self.data[coordinates.x as usize][coordinates.y as usize] = value;
        true
    }

    pub fn get_coordinates_vec(&self) -> Vec<Coordinates> {
        let mut coordinates: Vec<Coordinates> = Vec::with_capacity(self.depth() * self.width());

        for i in 0..self.depth() {
            for j in 0..self.width() {
                coordinates.push(Coordinates {
                    x: i as i32,
                    y: j as i32,
                });
            }
        }

        coordinates
    }

    pub fn get_ref(&self, coordinates: &Coordinates) -> Option<&T> {
        match self.is_in_bounds(coordinates) {
            false => None,
            true => Some(&self.data[coordinates.x as usize][coordinates.y as usize]),
        }
    }

    // Iteration

    pub fn enumerate(&self) -> impl Iterator<Item = (Coordinates, &T)> {
        self.data.iter().enumerate().flat_map(|(x_idx, sub_arr)| {
            sub_arr.iter().enumerate().map(move |(y_idx, elmt)| {
                (
                    Coordinates {
                        x: x_idx as i32,
                        y: y_idx as i32,
                    },
                    elmt,
                )
            })
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().flatten()
    }
}

impl Grid<char> {
    pub fn from_string(value: &str) -> Self {
        Self {
            data: value.lines().map(|line| line.chars().collect()).collect(),
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn fill(value: T, depth: usize, width: usize) -> Self {
        let mut sub_data: Vec<T> = Vec::with_capacity(width);
        for _ in 0..width {
            sub_data.push(value.clone());
        }

        let mut data: Vec<Vec<T>> = Vec::with_capacity(depth);
        for _ in 0..depth {
            data.push(sub_data.clone());
        }

        Self { data }
    }
}

impl<T: Debug> Grid<T> {
    pub fn show_surroundings(&self, coordinates: &Coordinates, depth: usize) {
        let min_x: i32 = coordinates.x - depth as i32;
        let max_x: i32 = coordinates.x + depth as i32;
        let min_y: i32 = coordinates.y - depth as i32;
        let max_y: i32 = coordinates.y + depth as i32;

        let mut result: String = String::new();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                match self.get_ref(&Coordinates { x, y }) {
                    None => result.push(' '),
                    Some(val) => result.push_str(&format!("{val:?}")),
                }
            }
            result.push('\n');
        }

        println!("{result}");
    }

    pub fn show_path(&self, path: &[Coordinates], path_char: char) {
        for x in 0..self.depth() {
            for y in 0..self.width() {
                let pos: Coordinates = Coordinates {
                    x: x as i32,
                    y: y as i32,
                };
                if path.contains(&pos) {
                    print!("{}", &path_char.to_string().green());
                } else {
                    print!("{}", &format!("{:?}", self.get_ref(&pos).unwrap()))
                }
            }

            println!();
        }
    }

    pub fn display(&self) {
        self.display_with_mark(None)
    }

    pub fn display_with_mark(&self, marked_position: Option<Coordinates>) {
        // Build horizontal indexes bar
        let mut horizontal_indexes: String = String::with_capacity(self.width() + 5);
        horizontal_indexes.push_str("     "); // Three spaces used by vertical bar
        for i in 0..self.width() {
            horizontal_indexes.push_str(&format!("{}", i % 10));
        }

        // Now construct the grid

        let mut result: String = String::new();

        // Add upper horizontal indexes bar
        result.push_str(&format!("{horizontal_indexes}\n"));

        // Add elements line by line (and insert vertical bars)
        for x in 0..self.depth() {
            // I will never debug by hand grids larger than 999 !
            // Also note the separation space
            let line_index_str: String = format!(" {:0>3} ", x);

            result.push_str(&line_index_str);

            for y in 0..self.width() {
                let current_coord: Coordinates = Coordinates {
                    x: x as i32,
                    y: y as i32,
                };

                match self.get_ref(&current_coord) {
                    None => result.push(' '),
                    Some(_) if marked_position == Some(current_coord) => result.push('@'),
                    Some(val) => result.push_str(&format!("{val:?}")), // Better if this is 1 char long ....
                }
            }
            result.push_str(&line_index_str);
            result.push('\n');
        }

        result.push_str(&horizontal_indexes);

        println!("{result}");
    }
}

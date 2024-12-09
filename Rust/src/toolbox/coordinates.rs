use crate::toolbox::Direction;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Default, Eq, Hash, PartialEq)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl Coordinates {
    /// Return the difference of coordinates between `self` and `other`.
    /// Not symmetric !
    pub fn get_delta(&self, other: &Self) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }

    pub fn with_delta(&self, delta: (i32, i32), add: bool) -> Self {
        match add {
            true => Coordinates {
                x: self.x + delta.0,
                y: self.y + delta.1,
            },
            false => Coordinates {
                x: self.x - delta.0,
                y: self.y - delta.1,
            },
        }
    }

    pub fn step(&self, direction: &Direction) -> Self {
        let (delta_x, delta_y): (i32, i32) = direction.get_delta();
        Coordinates {
            x: self.x + delta_x,
            y: self.y + delta_y,
        }
    }
}

impl Debug for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coord({},{})", self.x, self.y)
    }
}

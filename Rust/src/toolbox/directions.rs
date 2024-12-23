#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    pub fn from_delta(delta: (i32, i32)) -> Self {
        match delta {
            (-1, 0) => Direction::N,
            (-1, 1) => Direction::NE,
            (0, 1) => Direction::E,
            (1, 1) => Direction::SE,
            (1, 0) => Direction::S,
            (1, -1) => Direction::SW,
            (0, -1) => Direction::W,
            (-1, -1) => Direction::NW,
            _ => panic!("Invalid delta for direction"),
        }
    }

    pub fn get_delta(&self) -> (i32, i32) {
        match self {
            Direction::N => (-1, 0),
            Direction::NE => (-1, 1),
            Direction::E => (0, 1),
            Direction::SE => (1, 1),
            Direction::S => (1, 0),
            Direction::SW => (1, -1),
            Direction::W => (0, -1),
            Direction::NW => (-1, -1),
        }
    }

    pub fn left(&self) -> Self {
        match self {
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
            _ => unimplemented!("No left() for diagonal directions"),
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::NE => Direction::SW,
            Direction::E => Direction::W,
            Direction::SE => Direction::NW,
            Direction::S => Direction::N,
            Direction::SW => Direction::NE,
            Direction::W => Direction::E,
            Direction::NW => Direction::SE,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
            _ => unimplemented!("No right() for diagonal directions"),
        }
    }

    // Neighbours

    pub fn neighbours4() -> Vec<Self> {
        vec![Direction::N, Direction::E, Direction::S, Direction::W]
    }

    pub fn neighbours4_diagonal() -> Vec<Self> {
        vec![Direction::NE, Direction::SE, Direction::SW, Direction::NW]
    }

    pub fn neighbours8() -> Vec<Self> {
        vec![
            Direction::N,
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::S,
            Direction::SW,
            Direction::W,
            Direction::NW,
        ]
    }
}

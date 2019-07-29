extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right
}

impl Direction {
    pub fn apply(&self, x:f64, y:f64) -> (f64, f64) {
        match self {
            Direction::Up    => (x, y - 1.0),
            Direction::Left  => (x - 1.0, y),
            Direction::Down  => (x, y + 1.0),
            Direction::Right => (x + 1.0, y)
        }
    }
}
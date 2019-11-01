pub mod actor;
pub mod components;
pub mod duration;
pub mod status;
pub mod item;
pub mod class;
pub mod spell;
pub mod movement;
pub mod dungeon;

use std::ops::{Add, AddAssign, Sub, SubAssign};

extern crate serde;
use serde::{Serialize, Deserialize};

use crate::gamestate::movement::Direction;

/// A two-dimensional vector for describing locations
#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct LocationVec {
    pub x: f64,
    pub y: f64,
}

impl Add for LocationVec {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl AddAssign for LocationVec {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y
        };
    }
}

impl Sub for LocationVec {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl SubAssign for LocationVec {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y
        };
    }
}

impl From<Direction> for LocationVec {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up    => Self {x: 0.0, y: -1.0},
            Direction::Left  => Self {x: -1.0, y: 0.0},
            Direction::Down  => Self {x: 0.0, y: 1.0},
            Direction::Right => Self {x: 1.0, y: 0.0},
        }
    }
}

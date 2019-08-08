extern crate serde;

use serde::{Serialize, Deserialize};
use crate::gamestate::LocationVec;

#[derive(Eq, PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right
}

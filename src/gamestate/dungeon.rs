extern crate serde;

use serde::{Serialize, Deserialize};


#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum DungeonElement {
    Floor,
    Wall,
    Connector,
    Event,
}
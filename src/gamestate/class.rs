extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Class {
    Warrior
}
extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Duration {
    Infinite,
    Steps(i32),
    Updates(i32)
}
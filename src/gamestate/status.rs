extern crate serde;

use serde::{Serialize, Deserialize};

use crate::gamestate::duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub type_: StatusType,
    pub duration: duration::Duration
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StatusType {
    BaseStatusModifier(BaseStatusModifier),
    Invincible
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseStatusModifier {
    pub attack: i32,
    pub defense: i32,
    pub magic: i32,
    pub resistence: i32
}
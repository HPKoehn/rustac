extern crate serde;

use serde::{Serialize, Deserialize};

use crate::ecs::Entity;
use crate::gamestate::{status, duration};

pub enum Target {
    Entity(Entity),
    //ClosestCreature, // Includes Player; yet to be implemented
    Player,
    Initiator,
    None,
}


pub enum Trigger {
    OnCollision,
    StartOfTurn
}

pub enum EventType {
    NextLevel,
    Teleport(f64,f64),
    ModifyHealth(i32),
    ApplyStatus(status::StatusType, duration::Duration),
    CleanseStatus(status::StatusType),
}

#[derive(Eq, PartialEq, Clone, Debug, Ord, PartialOrd, Serialize, Deserialize)]
pub enum HitboxType {
    Transparent,
    Creature,
    Wall,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hitbox {
    pub x: f64,
    pub y: f64,
    pub type_: HitboxType,
}

impl Hitbox {
    pub fn new_small(type_: HitboxType) -> Hitbox {
        Hitbox {
            x: 0.0,
            y: 0.0,
            type_
        }
    }
}

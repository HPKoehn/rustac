use crate::trigger::trigger;
use std::ops::Range;

#[derive(Eq, PartialEq, Clone, Debug, Ord, PartialOrd)]
pub enum HitboxType {
    Transparent,
    Creature,
    Wall,
}

#[derive(Debug)]
pub struct Hitbox {
    pub x: f64,
    pub y: f64,

    pub type_: HitboxType,
    pub trigger: Option<trigger::Trigger>
}

pub fn passive_wall_hitbox() -> Hitbox {
        Hitbox {
            x: 0.0,
            y: 0.0,
            type_: HitboxType::Wall,
            trigger: None
        }
}

pub fn passive_creature_hitbox() -> Hitbox {
    Hitbox {
        x: 0.0,
        y: 0.0,
        type_: HitboxType::Creature,
        trigger: None
    }
}
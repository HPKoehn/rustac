use crate::trigger::trigger;
use std::ops::Range;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum HitboxType {
    Wall,
    Player,
    Transparent
}


//     2      |       yl
//     1      |       ^
// 2 1 0 1 2  |  xl < x > xr
//     1      |       v
//     2      |       yr

#[derive(Debug)]
pub struct Hitbox {
    pub xl: i32,
    pub xr: i32,
    pub yl: i32,
    pub yr: i32,

    pub type_: HitboxType,
    pub trigger: Option<trigger::Trigger>
}

impl Hitbox {
    pub fn x_range(&self) -> Range<i32> {
        Range {
            start: self.xl, 
            end: self.xr + 1
        }
    }

    pub fn y_range(&self) -> Range<i32> {
        Range {
            start: self.yl,
            end: self.yr + 1
        }
    }
}

pub fn passive_wall_hitbox() -> Hitbox {
        Hitbox {
            xl: 0, xr: 0, yl: 0, yr: 0,
            type_: HitboxType::Wall,
            trigger: None
        }
    }
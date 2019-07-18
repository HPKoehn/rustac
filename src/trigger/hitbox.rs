use crate::trigger::trigger;
use std::ops::Range;

pub enum HitboxType {
    Wall,
    Player,
    Transparent
}

pub struct Hitbox {
    pub xl: i32,
    pub xr: i32,
    pub yl: i32,
    pub yr: i32,

    type_: HitboxType,
    trigger: Option<trigger::Trigger>
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

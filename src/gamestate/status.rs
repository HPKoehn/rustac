use crate::gamestate::duration;

#[derive(Debug)]
pub struct Status {
    pub type_: StatusType,
    pub duration: duration::Duration
}

#[derive(Debug)]
pub enum StatusType {
    BaseStatusModifier(BaseStatusModifier),
    Invinsible
}

#[derive(Debug)]
pub struct BaseStatusModifier {
    pub attack: i32,
    pub defense: i32,
    pub magic: i32,
    pub resistence: i32
}
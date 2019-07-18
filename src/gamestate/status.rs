use crate::gamestate::duration;

pub struct Status {
    pub type_: StatusType,
    pub duration: duration::Duration
}
pub enum StatusType {
    BaseStatusModifier(BaseStatusModifier),
    Invinsible
}

pub struct BaseStatusModifier {
    pub attack: i32,
    pub defense: i32,
    pub magic: i32,
    pub resistence: i32
}
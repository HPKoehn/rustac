use crate::gamestate::{
    status,
    item,
    class,
    spell
};

pub struct HealthComponent {
    pub maximum: i32,
    pub current: i32
}

pub struct BaseStatsComponent {
    pub attack: i32,
    pub defense: i32,
    pub magic: i32,
    pub resistence: i32
}

pub struct StatusComponent {
    pub status: Vec<status::Status>
}

pub struct InventoryComponent {
    pub items: Vec<item::ItemEntry>,
    pub capacity: i32
}

pub struct LocationComponent {
    pub x: i32,
    pub y: i32
}

pub struct NameComponent {
    pub name: String
}

pub struct ClassComponent {
    pub class: class::Class,
    pub level: i32
}

pub struct CasterComponent {
    pub current_mana: i32,
    pub maximum_mana: i32,
    pub spells: spell::Spell
}

pub struct HumanoidComponent {
    left_hand: Option<item::Equipment>,
    right_hand: Option<item::Equipment>,

    head: Option<item::Equipment>,
    body: Option<item::Equipment>,
    hand: Option<item::Equipment>,
    leg:  Option<item::Equipment>,
    feet: Option<item::Equipment>,
}

pub struct BehaviorComponent {
    // todo behavior for monsters etc
}

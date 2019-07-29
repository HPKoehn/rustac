extern crate serde;

use serde::{Serialize, Deserialize};

pub type ItemId = i32;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub item: ItemType,
    pub amount: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ItemType {
    Equipment(Equipment),
    Consumable,
    KeyItem
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Equipment {
    OneHandWeapon,
    TwoHandWeapon,
    Helm,
    ChestArmor,
    Gloves,
    Pants,
    Boots,
}

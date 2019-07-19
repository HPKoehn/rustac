pub type ItemId = i32;

#[derive(Debug)]
pub struct Item {
    pub item: ItemType,
    pub amount: i32
}

#[derive(Debug)]
pub enum ItemType {
    Equipment(Equipment),
    Consumable,
    KeyItem
}

#[derive(Debug)]
pub enum Equipment {
    OneHandWeapon,
    TwoHandWeapon,
    Helm,
    ChestArmor,
    Gloves,
    Pants,
    Boots,
}

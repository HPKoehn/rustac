pub type ItemId = i32;

pub struct Item {
    pub item: ItemType,
    pub amount: i32
}

pub enum ItemType {
    Equipment(Equipment),
    Consumable,
    KeyItem
}

pub enum Equipment {
    OneHandWeapon,
    TwoHandWeapon,
    Helm,
    ChestArmor,
    Gloves,
    Pants,
    Boots,
}

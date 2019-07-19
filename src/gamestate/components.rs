use std::collections::HashMap;

use crate::gamestate::{
    status,
    item,
    class,
    spell,
    direction
};

use crate::trigger::{
    hitbox
};

use crate::render::{
    animation,
    sprite
};

#[derive(Debug)]
pub struct HealthComponent {
    pub maximum: i32,
    pub current: i32
}

#[derive(Debug)]
pub struct BaseStatsComponent {
    pub attack: i32,
    pub defense: i32,
    pub magic: i32,
    pub resistence: i32
}

#[derive(Debug)]
pub struct StatusComponent {
    pub status: Vec<status::Status>
}

#[derive(Debug)]
pub struct InventoryComponent {
    pub items: Vec<item::Item>, //todo change to entity index
    pub capacity: i32
}

#[derive(Debug)]
pub struct LocationComponent {
    pub x: i32,
    pub y: i32,
    pub direction: direction::Direction,
    pub hitbox: Option<hitbox::Hitbox>
}

#[derive(Debug)]
pub struct NameComponent {
    pub name: String
}

#[derive(Debug)]
pub struct ClassComponent {
    pub class: class::Class,
    pub level: i32,
    pub experience: i32
}

#[derive(Debug)]
pub struct CasterComponent {
    pub current_mana: i32,
    pub maximum_mana: i32,
    pub spells: Vec<spell::Spell>
}

#[derive(Debug)]
pub struct HumanoidComponent {
    pub left_hand: Option<item::Equipment>,
    pub right_hand: Option<item::Equipment>,

    pub head: Option<item::Equipment>,
    pub body: Option<item::Equipment>,
    pub hand: Option<item::Equipment>,
    pub leg:  Option<item::Equipment>,
    pub feet: Option<item::Equipment>,
}

#[derive(Debug)]
pub struct NpcBehaviorComponent {
    // todo behavior for monsters etc
}

// what do entities drop if they die
#[derive(Debug)]
pub struct ItemDropComponent {
    pub gold: i32,
    pub item: Option<item::ItemId>
}

#[derive(Debug)]
pub struct PlayerComponent {
    pub stage_level: i32,
    pub gold: i32,

    // used for tracking progress of player
    pub progression_flags: HashMap<String, bool>
}

#[derive(Debug)]
pub struct RenderComponent {
    pub base_sprite: sprite::SpriteId, // change to ressource entity?
    pub animation: Option<animation::AnimationState>,
    pub visible: bool,
    pub render_layer: i32
}
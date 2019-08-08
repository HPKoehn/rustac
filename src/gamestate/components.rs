extern crate serde;

use serde::{Serialize, Deserialize};

use std::collections::HashMap;

use crate::gamestate::{actor, status, item, class, spell, direction, dungeon, LocationVec};

use crate::event;

use crate::render::{
    animation,
    sprite
};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthComponent {
    pub maximum: i32,
    pub current: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseStatsComponent {
    pub attack: i32,
    pub defense: i32,
    pub magic: i32,
    pub resistence: i32
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ActorComponent {
    pub state: actor::ActorState,
    pub turn: u64,
    pub max_actions: u64,
    pub performed_actions : u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusComponent {
    pub status: Vec<status::Status>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryComponent {
    pub items: Vec<item::Item>, //todo change to entity index
    pub capacity: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationComponent {
    pub location: LocationVec,
    pub direction: direction::Direction,
    pub hitbox: Option<event::Hitbox>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameComponent {
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassComponent {
    pub class: class::Class,
    pub level: i32,
    pub experience: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CasterComponent {
    pub current_mana: i32,
    pub maximum_mana: i32,
    pub spells: Vec<spell::Spell>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HumanoidComponent {
    pub left_hand: Option<item::Equipment>,
    pub right_hand: Option<item::Equipment>,

    pub head: Option<item::Equipment>,
    pub body: Option<item::Equipment>,
    pub hand: Option<item::Equipment>,
    pub leg:  Option<item::Equipment>,
    pub feet: Option<item::Equipment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NpcBehaviorComponent {
    // todo behavior for monsters etc
}

// what do entities drop if they die
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemDropComponent {
    pub gold: i32,
    pub item: Option<item::ItemId>
}

// shows belonging to a room
#[derive(Debug, Serialize, Deserialize)]
pub struct DungeonComponent {
    pub type_: dungeon::DungeonElement
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerComponent {
    pub stage_level: i32,
    pub gold: i32,

    // used for tracking progress of player
    pub progression_flags: HashMap<String, bool>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderComponent {
    pub base_sprite: sprite::SpriteId, // change to ressource entity?
    pub base_sprite_size: f64,
    pub animation: Option<animation::AnimationState>,
    pub visible: bool,
    pub render_layer: i32
}

pub struct EventComponent {
    pub target: event::Target,
    pub event_type: event::EventType,
    pub trigger: event::Trigger
}
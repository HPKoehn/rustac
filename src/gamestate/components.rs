extern crate serde;

use serde::{Serialize, Deserialize};

use std::collections::HashMap;

use crate::gamestate::{actor, status, item, class, spell, movement, dungeon, LocationVec};

use crate::event;

use crate::render::{
    animation,
    sprite
};


/// Enables an Entity to take damage, heal and die
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthComponent {
    pub maximum: i32,
    pub current: i32
}

/// Enables an Entity to have changable offensive and defensive capabilities.
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseStatsComponent {
    pub attack: i32,
    pub defense: i32,
    pub magic: i32,
    pub resistence: i32
}

/// Enables an Entity to get a turn and perform actions
#[derive(Debug, Serialize, Deserialize)]
pub struct ActorComponent {
    pub state: actor::ActorState,
    pub turn: u64,
    pub max_actions: u64,
    pub performed_actions : u64,
}

/// Enables an Entity to recieve status effects
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusComponent {
    pub status: Vec<status::Status>
}

/// Enables an entity to have items
#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryComponent {
    pub items: Vec<item::Item>, //todo change to entity index
    pub capacity: i32
}

/// Enables an entity to be in the level and physically interact
/// with other entities with a LocationComponent
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationComponent {
    pub location: LocationVec,
    pub direction: movement::Direction,
    pub move_intent: Option<movement::MoveIntent>,
    pub hitbox: Option<event::Hitbox>
}

/// Enables an entity to have a name
#[derive(Debug, Serialize, Deserialize)]
pub struct NameComponent {
    pub name: String
}

/// Enables an entity to have a class, a level and to gain expirience
#[derive(Debug, Serialize, Deserialize)]
pub struct ClassComponent {
    pub class: class::Class,
    pub level: i32,
    pub experience: i32
}

/// Enables an entity to have and cast spells and abilities
#[derive(Debug, Serialize, Deserialize)]
pub struct CasterComponent {
    pub current_mana: i32,
    pub maximum_mana: i32,
    pub spells: Vec<spell::Spell>
}

/// Enables an entity to equip items like a human
#[derive(Debug, Serialize, Deserialize)]
pub struct HumanoidComponent {
    pub off_hand: Option<item::Equipment>,
    pub main_hand: Option<item::Equipment>,

    pub head: Option<item::Equipment>,
    pub body: Option<item::Equipment>,
    pub hand: Option<item::Equipment>,
    pub leg:  Option<item::Equipment>,
    pub feet: Option<item::Equipment>,
}

/// Enables an entity to have AI behavior
#[derive(Debug, Serialize, Deserialize)]
pub struct NpcBehaviorComponent {
    // todo behavior for monsters etc
}

/// Enables an entity to drop items
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemDropComponent {
    pub gold: i32,
    pub item: Option<item::ItemId>
}

/// Classifies an entity as specific parts of the dungeon environment
#[derive(Debug, Serialize, Deserialize)]
pub struct DungeonComponent {
    pub type_: dungeon::DungeonElement
}

/// Classifies an entity as a player and enables them to carry gold and 
/// save state/progress
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerComponent {
    pub stage_level: i32,
    pub gold: i32,

    // used for tracking progress of player
    pub progression_flags: HashMap<String, bool>
}

/// Enables an entity to have a sprite and be rendered
#[derive(Debug, Serialize, Deserialize)]
pub struct RenderComponent {
    pub base_sprite: sprite::SpriteId, // change to ressource entity?
    pub base_sprite_size: f64,
    pub animation: Option<animation::AnimationState>,
    pub visible: bool,
    pub render_layer: i32
}

/// Enables an entity to trigger events when certain triggers are met
pub struct EventComponent {
    pub target: event::Target,
    pub event_type: event::EventType,
    pub trigger: event::Trigger
}



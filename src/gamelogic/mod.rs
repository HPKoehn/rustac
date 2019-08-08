use crate::ecs;
use crate::gamestate::{actor, direction, status::StatusType};
use std::ops::Add;


pub enum PlayerAction {
    Interact(ecs::Entity),
    Attack(ecs::Entity),
    Move(direction::Direction),
}

pub fn perform_player_action(ecs_: &mut ecs::ECS, player_action: PlayerAction) -> bool {
    if let Some(player) = ecs_.get_player_entity() {
        match player_action {
            PlayerAction::Move(dir) => {
                // get player position
                if !move_entity(ecs_, player, dir) {
                    print!("Player tried to move to a location, but was denied!\n");
                }
            }
            _ => { /*TODO Implement other actions*/ }
        }
    }
    true
}

// moves entity in direction if possible (hitbox check (currently only supports 1x1 hitboxes on location field))
pub fn move_entity(ecs_: &mut ecs::ECS, entity: ecs::Entity, dir: direction::Direction) -> bool {
    // check if move is okay
    if let Some(location_comp) = ecs_.location_component.get(entity) {
        let location = location_comp.location.add(dir.into());

        // check all entities standing at target location (tx,ty)
        let target_entities = ecs_.get_entities_by_location(location);
        // no walking in space (as floor tiles are entities as well)
        if target_entities.is_empty() {
            return false;
        }

        // check hitboxes at target location
        for target_entity in target_entities {
            // they will have a location component as we only get entities with such
            if let Some(target_entity_location_comp) = ecs_.location_component.get(target_entity) {
                // get the hitboxes if they exist
                if let Some(target_hitbox) = &target_entity_location_comp.hitbox {
                    if let Some(source_hitbox) = &location_comp.hitbox {
                        if target_hitbox.type_ >= source_hitbox.type_ {
                            return false;
                        }
                    }
                }
            }
        }
    }

    // now get location as mutable to move
    if let Some(location_comp) = ecs_.location_component.get_mut(entity) {
        location_comp.location += dir.into();
    }
    true
}

pub fn attack(ecs_: &mut ecs::ECS, attacker: ecs::Entity, target: ecs::Entity) -> bool {
    let mut attacker_atk = 0;
    let mut target_def = 0;

    // calculate atk for attacker
    if let Some(basestats_c) = ecs_.basestats_component.get(attacker) {
        attacker_atk = basestats_c.attack;

        // check for basestats status modifications
        if let Some(status_c) = ecs_.status_component.get(attacker) {
            for status in &status_c.status {
                if let StatusType::BaseStatusModifier(modifier) = &status.type_ {
                    attacker_atk += modifier.attack
                }
            }
        }
        //TODO check for equipment

    }

    // calculate def for target
    if let Some(basestats_c) = ecs_.basestats_component.get(target) {
        target_def = basestats_c.defense;

        // check for basestats status modifications
        if let Some(status_c) = ecs_.status_component.get(target) {
            for status in &status_c.status {
                match &status.type_ {
                    StatusType::BaseStatusModifier(modifier) => target_def += modifier.defense,
                    StatusType::Invinsible => { return false; }
                }
            }
        }
        //TODO check for equipment
    }

    // apply damage (do at least 1 damage)
    let damage = std::cmp::max(1, attacker_atk * 2 - target_def);

    if let Some(target_health) = ecs_.health_component.get_mut(target) {
        target_health.current -= damage;
        target_health.current = std::cmp::max(0, target_health.current);
    } else {
        // if no health no attack
        return false;
    }
    true
}


// Force moves an entity to a location
// returns false if entity does not exist or has no location component
pub fn force_move(ecs_: &mut ecs::ECS, entity: ecs::Entity, x: f64, y: f64) -> bool {
    if let Some(location_c) = ecs_.location_component.get_mut(entity) {
        location_c.location.x = x;
        location_c.location.y = y;
        true
    } else {
        false
    }
}

pub fn check_and_perform_end_turn(ecs_: &mut ecs::ECS) {
    let mut actors: Vec<ecs::Entity> = Vec::new();
    let mut all_done = true;
    for entity in ecs_.allocator.live_indices() {
        if let Some(actor_c) = ecs_.actor_component.get(entity) {
            actors.push(entity);
            if actor_c.state != actor::ActorState::DoneActing {
                all_done = false;
                return; // might be removed later if more needs to be done
            }
        }
    }

    if all_done {
        for actor_entity in actors {
            if let Some(actor_c) = ecs_.actor_component.get_mut(actor_entity) {
                actor_c.state = actor::ActorState::WaitingForTurn;
                actor_c.turn += 1;
            }
        }
    }
}
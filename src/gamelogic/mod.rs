use crate::ecs;
use crate::gamestate::{ actor, direction };


pub enum PlayerAction {
    Interact(ecs::Entity),
    Attack(ecs::Entity),
    Move(direction::Direction)
}

pub fn perform_player_action(ecs_: &mut ecs::ECS, player_action: PlayerAction) -> bool {
    if let Some(player) = ecs_.get_player_entity() {

        match player_action {
            PlayerAction::Move(dir) => {
                // get player position
                if !move_entity(ecs_, player, dir) {
                    print!("Player tried to move to a location, but was denied!");
                }

            },
            _ => {/*TODO Implement other actions*/}
        }
    }
    true
}

// moves entity in direction if possible (hitbox check (currently only supports 1x1 hitboxes on location field))
pub fn move_entity(ecs_: &mut ecs::ECS, entity: ecs::Entity, dir: direction::Direction) -> bool {
    // check if move is okay
    if let Some(location) = ecs_.location_component.get(entity) {
        let (tx, ty) = dir.apply(location.x, location.y);

        // check all entities standing at target location (tx,ty)
        let target_entities = ecs_.get_entities_by_location(tx, ty);
        // no walking in space (as floor tiles are entities as well)
        if target_entities.is_empty() {
            return false;
        }

        // check hitboxes at target location
        for target_entity in target_entities {
            // they will have a location component as we only get entities with such
            if let Some(target_entity_location) = ecs_.location_component.get(target_entity) {
                // get the hitboxes if they exist
                if let Some(target_hitbox) = &target_entity_location.hitbox {
                    if let Some(source_hitbox) = &location.hitbox {
                        if target_hitbox.type_ >= source_hitbox.type_ {
                            return false;
                        }
                    }
                }
            }
        }
    }

    // now get location as mutable to move
    if let Some(location) = ecs_.location_component.get_mut(entity) {
        location.apply(dir);
    }
    true
}


// Force moves an entity to a location
// returns false if entity does not exist or has no location component
pub fn force_move(ecs_: &mut ecs::ECS, entity: ecs::Entity, x: f64, y: f64)-> bool {
    if let Some(location) = ecs_.location_component.get_mut(entity) {
        location.x = x;
        location.y = y;
        true
    } else {
        false
    }
}
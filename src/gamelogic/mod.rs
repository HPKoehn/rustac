use crate::ecs;
use crate::gamestate::{actor, movement, status::StatusType, LocationVec};
use crate::ut;
use std::ops::Add;

extern crate rand;
use rand::Rng;

/// Possible actions for the player
pub enum PlayerAction {
    Interact(ecs::Entity),
    Attack,
    Move(movement::Direction),
}

/// Executes a player action
/// 
/// ### Arguments
/// * `ecs_`          - The entity component system to perform on
/// * `player_action` - The player_action to execute
/// 
/// ### Returns
/// True if the action was performed successfully, else false
/// 
pub fn perform_player_action(ecs_: &mut ecs::ECS, player_action: PlayerAction) -> bool {
    if let Some(player) = ecs_.get_player_entity() {
        match player_action {
            PlayerAction::Move(dir) => {
                // get player position
                if !move_entity(ecs_, player, dir) {
                    debug!("Player tried to move to a location, but was denied!");
                    return false;
                }
            }
            PlayerAction::Attack => {
                // find player
                if let Some(location_c) = ecs_.location_component.get(player) {
                    let target_location = location_c.location + LocationVec::from(location_c.direction);
                    let potential_targets = ecs_.get_entities_by_location(target_location);
                    for target in potential_targets {
                        if let Some(_) = ecs_.health_component.get(target) {
                            attack(ecs_, player, target);
                        }
                    }
                }
                else {
                    debug!("Player tried to attack, but has no location!");
                    return false;
                }
            }
            _ => { /*TODO Implement other actions*/ }
        }
    }
    true
}

/// Sets a `MoveIntent` for the entity to move into the given direction by one field. To be able to move into a direction, 
/// there must be no colliding `event::Hitbox` of equal or greater ordering layer than all entities at the target location 
/// and there must be an entity at all (e.g. a floor tile entity)
/// 
/// ### Arguments
/// * `ecs_`   - The entity component system to perform on
/// * `entity` - The entity to move
/// * `dir`    - The direction to move into
/// 
/// ### Returns
/// True if all prerequirements were fullfilled and the `MoveIntent` was set successfully, else false
pub fn move_entity(ecs_: &mut ecs::ECS, entity: ecs::Entity, dir: movement::Direction) -> bool {
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
        // if we move we set the movement component
        location_comp.move_intent = Some(movement::MoveIntent::Vector(dir.into(), movement::DEFAULT_SPEED));
        // set now direction
        location_comp.direction = dir;
    }
    true
}

/// Performs damage calculation for an attacking and definding entity and then applies damage.
/// Status are considered the calcution (also status like `StatusType::Invincible`).
/// **NOTE**: No prerequirements to perfom the attack like range checks are performed here!
/// 
/// ### Arguments
/// * `ecs_`     - The entity component system to perform on
/// * `attacker` - The attacking entity
/// * `target`   - The definding/attacked entity
/// 
/// ### Returns
/// True if the attack was successful and applied at least one damage, else false
/// 
pub fn attack(ecs_: &mut ecs::ECS, attacker: ecs::Entity, target: ecs::Entity) -> bool {
    let mut attacker_atk = 0;
    let mut attacker_mul = 2.0;
    let mut target_def = 0;
    let mut target_mul = 1.0;

    // calculate atk for attacker
    if let Some(basestats_c) = ecs_.basestats_component.get(attacker) {
        attacker_atk = basestats_c.attack;

        // check for basestats status modifications
        if let Some(status_c) = ecs_.status_component.get(attacker) {
            for status in &status_c.status {
                match &status.type_ {
                    StatusType::BaseStatusModifier(modifier) => attacker_atk += modifier.attack,
                    StatusType::BaseStatusMuliplier(multiplier) => attacker_mul += multiplier.attack,
                    _ => {}
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
                    StatusType::BaseStatusMuliplier(multiplier) => target_mul += multiplier.defense,
                    StatusType::Invincible => { return false; }
                }
            }
        }
        //TODO check for equipment
    }

    // attacks vary by 10%  (90% - 110%)
    let dmg_percentile = rand::thread_rng().gen_range(90.0, 110.0) / 100.0;
    // apply damage (do at least 1 damage)
    let damage = std::cmp::max(
        1, 
        ((attacker_atk as f32 * attacker_mul  - target_def as f32 * target_mul) * dmg_percentile) as i32
    );

    info!("{} dealt {} damage to {}", ut::name_or_id(ecs_, attacker), damage, ut::name_or_id(ecs_, target));

    if let Some(target_health) = ecs_.health_component.get_mut(target) {
        target_health.current -= damage;
        target_health.current = std::cmp::max(0, target_health.current);
    } else {
        // if no health no attack
        return false;
    }
    true
}


/// Sets the location for an entity ignoring any movement restrictions (force move)
/// 
/// ### Arguments
/// * `ecs_`   - The entity component system to perform on
/// * `entity` - The entity to move
/// * `x`      - The x coordinat of target location
/// * `y`      - The y coordinat of target location
/// 
/// ### Returns
/// True if the location could be set, else false
pub fn force_move(ecs_: &mut ecs::ECS, entity: ecs::Entity, x: f64, y: f64) -> bool {
    if let Some(location_c) = ecs_.location_component.get_mut(entity) {
        location_c.location.x = x;
        location_c.location.y = y;
        true
    } else {
        false
    }
}

/// Tests if all entities eligible have performed their turns
/// and if so increase the turn count for each actor and ready their
/// action once again
/// 
/// ### Arguments
/// * `ecs_`:  - The entity component system to perform on
/// 
/// ### Returns
/// True if all entities have finished their turn, else false
/// 
pub fn check_and_perform_end_turn(ecs_: &mut ecs::ECS) -> bool {
    let mut actors: Vec<ecs::Entity> = Vec::new();
    let mut all_done = true;
    for entity in ecs_.allocator.live_indices() {
        if let Some(actor_c) = ecs_.actor_component.get(entity) {
            actors.push(entity);
            if actor_c.state != actor::ActorState::DoneActing {
                all_done = false;
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
    all_done
}

/// Advances all `MovementIntents` by one step and updates their `LocationComponent`
/// 
/// ### Arguments
/// * `ecs_` - The entity component system to perform on
/// 
pub fn update_entity_positions(ecs_: &mut ecs::ECS) {
    for entity in ecs_.allocator.live_indices() {
        if let Some(movement_c) = ecs_.location_component.get_mut(entity) {
            let mut at_goal = false;
            if let Some(movement_intent) = &mut movement_c.move_intent {
                movement_c.location = movement_intent.move_from(&movement_c.location);
                at_goal = movement_intent.has_arrived(&movement_c.location);
            }
            if at_goal {
                movement_c.move_intent = None;
            }
        }
    }
}
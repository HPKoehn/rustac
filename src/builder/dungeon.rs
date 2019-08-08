use crate::ecs;
use crate::gamestate::{components, direction, dungeon, LocationVec};
use crate::event::{Hitbox, HitboxType};

pub fn tear_down_level(ecs_: &mut ecs::ECS) {
    for entity in ecs_.allocator.live_indices() {
        if ecs_.dungeon_component.get(entity).is_some() {
            ecs_.allocator.deallocate(entity);
        }
    }
}

pub fn create_floor_tile(ecs_: &mut ecs::ECS, x: f64, y: f64) -> ecs::Entity {
    let entity = ecs_.allocator.allocate();
    ecs_.dungeon_component.set(entity, components::DungeonComponent {
        type_: dungeon::DungeonElement::Floor
    });
    ecs_.location_component.set(entity, components::LocationComponent {
        location: LocationVec { x, y },
        direction: direction::Direction::Down,
        hitbox: None
    });
    ecs_.render_component.set(entity, components::RenderComponent {
        base_sprite: "floor_tile".to_string(),
        base_sprite_size: 1.0,
        animation: None,
        visible: true,
        render_layer: 0
    });
    entity
}

pub fn create_wall_tile(ecs_: &mut ecs::ECS, x: f64, y: f64) -> ecs::Entity {
    let entity = ecs_.allocator.allocate();
    ecs_.dungeon_component.set(entity, components::DungeonComponent {
        type_: dungeon::DungeonElement::Wall
    });
    ecs_.location_component.set(entity, components::LocationComponent {
        location: LocationVec { x, y },
        direction: direction::Direction::Down,
        hitbox: Some(Hitbox::new_small(HitboxType::Wall))
    });
    ecs_.render_component.set(entity, components::RenderComponent {
        base_sprite: "wall_tile".to_string(),
        base_sprite_size: 1.0,
        animation: None,
        visible: true,
        render_layer: 1
    });
    entity
}

pub fn create_connector_tile(ecs_: &mut ecs::ECS, x: f64, y: f64) -> ecs::Entity {
    let entity = ecs_.allocator.allocate();
    ecs_.dungeon_component.set(entity, components::DungeonComponent {
        type_: dungeon::DungeonElement::Connector
    });
    ecs_.location_component.set(entity, components::LocationComponent {
        location: LocationVec { x, y },
        direction: direction::Direction::Down,
        hitbox: None
    });
    entity
}

pub fn create_empty_room(ecs_: &mut ecs::ECS, x: f64, y: f64, xs: u64, ys: u64) {
    //floor
    for i in 0..xs {
        for j in 0..ys {
            create_floor_tile(ecs_, x + i as f64, y + j as f64);
        }
    }

    // wall
    for i in 0..xs {
        create_wall_tile(ecs_, x + i as f64, y);
        create_wall_tile(ecs_, x + i as f64, y + ys as f64 - 1.0);
    }

    // shorter than xs so no double walls at corners
    for j in 1..(ys-1) {
        create_wall_tile(ecs_, x, y + j as f64);
        create_wall_tile(ecs_, x + xs as f64 - 1.0, y + j as f64);
    }
}

pub fn delete_dungeon_entities(ecs_: &mut ecs::ECS, type_: dungeon::DungeonElement, location: LocationVec) {
    for entity in ecs_.get_entities_by_location(location) {
        if let Some(dungeon_c) = ecs_.dungeon_component.get(entity) {
            if dungeon_c.type_ == type_ {
                ecs_.allocator.deallocate(entity);
            }
        }
    }
}
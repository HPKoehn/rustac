use crate::ecs;
use crate::gamestate::{
    components,
    direction,
    dungeon,
};
use crate::trigger::hitbox;

pub fn tear_down_level(ecs_: &mut ecs::ECS) {
    for entity in ecs_.allocator.live_indices() {
        if ecs_.dungeon_component.get(entity).is_some() {
            ecs_.allocator.deallocate(entity);
        }
    }
}

pub fn create_floor_tile(ecs_: &mut ecs::ECS, x: i32, y: i32) -> ecs::Entity {
    let entity = ecs_.allocator.allocate();
    ecs_.dungeon_component.set(entity, components::DungeonComponent {
        type_: dungeon::DungeonElement::Floor
    });
    ecs_.location_component.set(entity, components::LocationComponent {
        x: x,
        y: y,
        direction: direction::Direction::Down,
        hitbox: None
    });
    ecs_.render_component.set(entity, components::RenderComponent {
        base_sprite: "default",
        base_sprite_size: 1,
        animation: None,
        visible: true,
        render_layer: 0
    });
    entity
}

pub fn create_wall_tile(ecs_: &mut ecs::ECS, x: i32, y: i32) -> ecs::Entity {
    let entity = ecs_.allocator.allocate();
    ecs_.dungeon_component.set(entity, components::DungeonComponent {
        type_: dungeon::DungeonElement::Wall
    });
    ecs_.location_component.set(entity, components::LocationComponent {
        x: x,
        y: y,
        direction: direction::Direction::Down,
        hitbox: Some(hitbox::passive_wall_hitbox())
    });
    entity
}

pub fn create_connector_tile(ecs_: &mut ecs::ECS, x: i32, y: i32) -> ecs::Entity {
    let entity = ecs_.allocator.allocate();
    ecs_.dungeon_component.set(entity, components::DungeonComponent {
        type_: dungeon::DungeonElement::Connector
    });
    ecs_.location_component.set(entity, components::LocationComponent {
        x: x,
        y: y,
        direction: direction::Direction::Down,
        hitbox: None
    });
    entity
}
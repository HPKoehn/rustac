use crate::ecs;


/// Returns the name of the entity, if it has no name, the debug string
/// of the entity id is returned instead
pub fn name_or_id(ecs_: &ecs::ECS, entity: ecs::Entity) -> String {
    if let Some(name) = ecs_.name_component.get(entity) {
        name.name.clone()
    } else {
        format!("{:?}", entity)
    }
}
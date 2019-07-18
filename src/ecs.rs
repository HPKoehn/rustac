extern crate recs;
use recs::allocation;

use crate::gamestate::components::*;

pub type Entity = allocation::GenerationalIndex;
pub type EntityMap<T> = allocation::GenerationalIndexArray<T>;

pub struct ECS {
    pub allocator: allocation::GenerationalIndexAllocator,

    pub health_components: EntityMap<HealthComponent>,
    pub basestats_component: EntityMap<BaseStatsComponent>,
    pub status_component: EntityMap<StatusComponent>,
    pub inventory_component: EntityMap<InventoryComponent>,
    pub location_component: EntityMap<LocationComponent>,
    pub name_component: EntityMap<NameComponent>,
    pub class_component: EntityMap<ClassComponent>,
    pub caster_component: EntityMap<CasterComponent>,
    pub humanoid_component: EntityMap<HumanoidComponent>,
    pub npc_behavior_component: EntityMap<NpcBehaviorComponent>,
    pub item_drop_component: EntityMap<ItemDropComponent>,
    pub player_component: EntityMap<PlayerComponent>,
    pub render_component: EntityMap<RenderComponent>
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            allocator: allocation::GenerationalIndexAllocator::new(),

            health_components: EntityMap::new(),
            basestats_component: EntityMap::new(),
            status_component: EntityMap::new(),
            inventory_component: EntityMap::new(),
            location_component: EntityMap::new(),
            name_component: EntityMap::new(),
            class_component: EntityMap::new(),
            caster_component: EntityMap::new(),
            humanoid_component: EntityMap::new(),
            npc_behavior_component: EntityMap::new(),
            item_drop_component: EntityMap::new(),
            player_component: EntityMap::new(),
            render_component: EntityMap::new(),
        }
    }
}


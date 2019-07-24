extern crate recs;
use recs::allocation;

use crate::gamestate::components::*;

pub type Entity = allocation::GenerationalIndex;
pub type EntityMap<T> = allocation::GenerationalIndexArray<T>;

pub struct ECS {
    pub allocator: allocation::GenerationalIndexAllocator,

    pub actor_component: EntityMap<ActorComponent>,
    pub basestats_component: EntityMap<BaseStatsComponent>,
    pub caster_component: EntityMap<CasterComponent>,
    pub class_component: EntityMap<ClassComponent>,
    pub dungeon_component: EntityMap<DungeonComponent>,
    pub health_component: EntityMap<HealthComponent>,
    pub humanoid_component: EntityMap<HumanoidComponent>,
    pub inventory_component: EntityMap<InventoryComponent>,
    pub item_drop_component: EntityMap<ItemDropComponent>,
    pub location_component: EntityMap<LocationComponent>,
    pub name_component: EntityMap<NameComponent>,
    pub npc_behavior_component: EntityMap<NpcBehaviorComponent>,
    pub player_component: EntityMap<PlayerComponent>,
    pub render_component: EntityMap<RenderComponent>,
    pub status_component: EntityMap<StatusComponent>,
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            allocator: allocation::GenerationalIndexAllocator::new(),

            actor_component: EntityMap::new(),
            basestats_component: EntityMap::new(),
            caster_component: EntityMap::new(),
            class_component: EntityMap::new(),
            dungeon_component: EntityMap::new(),
            health_component: EntityMap::new(),
            humanoid_component: EntityMap::new(),
            inventory_component: EntityMap::new(),
            item_drop_component: EntityMap::new(),
            location_component: EntityMap::new(),
            name_component: EntityMap::new(),
            npc_behavior_component: EntityMap::new(),
            player_component: EntityMap::new(),
            render_component: EntityMap::new(),
            status_component: EntityMap::new(),
        }
    }
}






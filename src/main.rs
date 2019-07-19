mod gamestate;
mod trigger;
mod render;
mod ecs;

fn main() {
    println!("Hello, world!");
    let mut my_ecs = crate::ecs::ECS::new();

    let entity = my_ecs.allocator.allocate();
    print!("{:?}", my_ecs.health_components.get(entity));
    my_ecs.health_components.set(entity, crate::gamestate::components::HealthComponent {
        current: 100,
        maximum: 100
    });
    print!("{:?}", my_ecs.health_components.get(entity));
}

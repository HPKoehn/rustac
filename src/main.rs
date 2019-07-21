mod gamestate;
mod trigger;
mod render;
mod builder;
mod ecs;

fn main() {
    println!("Hello, world!");
    let mut my_ecs = crate::ecs::ECS::new();

    let entity = my_ecs.allocator.allocate();
    print!("{:?}", my_ecs.health_component.get(entity));
    my_ecs.health_component.set(entity, crate::gamestate::components::HealthComponent {
        current: 100,
        maximum: 100
    });
    print!("{:?}", my_ecs.health_component.get(entity));
}

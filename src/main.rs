mod gamestate;
mod gamelogic;
mod event;
mod render;
mod builder;
mod ecs;
mod input;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;

use render::{render_game, sprite, RenderConfig};
use gamestate::components;
use event::{Hitbox, HitboxType};
use crate::gamestate::LocationVec;

const UPDATES_PER_SECOND: u64 = 30;

fn main() {

    // setup of main data structures
    let mut ecs_ = ecs::ECS::new();
    let mut render_conf = RenderConfig {
        scale: 100.0,
        window_xs: 1000,
        window_ys: 1000,
        focused_entity: None
    };
    
    // setup of opengl window
    let opengl  = OpenGL::V3_2;
	let mut window: Window = WindowSettings::new(
            "rustac",
            [render_conf.window_xs, render_conf.window_ys]
        )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

	let ref mut gl  = GlGraphics::new(opengl);
    let sprite_textures = sprite::setup_sprite_textures();
	
    // BEGIN test code

    use crate::builder::dungeon;

    dungeon::create_empty_room(&mut ecs_, 0.0, 0.0, 10, 8);
    dungeon::create_empty_room(&mut ecs_, 0.0, 8.0, 8, 10);
    dungeon::delete_dungeon_entities(&mut ecs_, 
                                     gamestate::dungeon::DungeonElement::Wall,
                                     LocationVec { x: 4.0, y: 7.0 });
    dungeon::delete_dungeon_entities(&mut ecs_, 
                                     gamestate::dungeon::DungeonElement::Wall,
                                     LocationVec { x: 4.0, y: 8.0 });

    
    // player
    let player = create_test_dummy_player(&mut ecs_);

    render_conf.focused_entity = Some(player);

    // END test code

	//Main loop

    let mut events = Events::new(EventSettings::new());
    events.set_ups(UPDATES_PER_SECOND);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            render_game(gl, &r, &mut ecs_, &sprite_textures, &render_conf);
        }
        if let Some(p) = e.press_args() {
            input::handle_input(&p, &mut ecs_);
        }

        gamelogic::check_and_perform_end_turn(&mut ecs_);
	}
}
    

fn create_test_dummy_player(ecs_: &mut ecs::ECS) -> ecs::Entity {

    let player = ecs_.allocator.allocate();

    ecs_.actor_component.set(player, components::ActorComponent {
        state: gamestate::actor::ActorState::WaitingForTurn,
        turn: 0,
        max_actions: 1,
        performed_actions: 0
    });

    ecs_.location_component.set(player, components::LocationComponent {
        location: LocationVec {x: 1.0, y: 1.0},
        direction: gamestate::movement::Direction::Down,
        move_intent: None,
        hitbox: Some(Hitbox::new_small(HitboxType::Creature))
    });

    ecs_.render_component.set(player, components::RenderComponent {
        base_sprite: "player".to_string(),
        base_sprite_size: 1.0,
        animation: None,
        visible: true,
        render_layer: 2
    });

    ecs_.player_component.set(player, components::PlayerComponent {
        stage_level: 0,
        gold: 0,
        progression_flags: std::collections::HashMap::new()
    });

    player
}
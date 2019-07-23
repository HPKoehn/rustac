mod gamestate;
mod trigger;
mod render;
mod builder;
mod ecs;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;

use crate::render::{render_game, sprite, RenderConfig};

const UPDATES_PER_SECOND: u64 = 30;

fn main() {

    // setup of main data structures
    let mut ecs_ = ecs::ECS::new();
    let render_conf = RenderConfig {
        scale: 100.0,
        window_xs: 1000,
        window_ys: 1000
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
        
	let mut gl  = GlGraphics::new(opengl);
    let sprite_textures = sprite::setup_sprite_textures();
	
    // BEGIN test code

    use crate::builder::dungeon;
    dungeon::create_floor_tile(&mut ecs_, 0.0, 0.0);
    dungeon::create_floor_tile(&mut ecs_, 2.0, 0.0);
    dungeon::create_floor_tile(&mut ecs_, 1.0, 1.0);
    dungeon::create_floor_tile(&mut ecs_, 1.0, 1.0);

    // END test code

	//Main loop

    let mut events = Events::new(EventSettings::new());
    events.set_ups(UPDATES_PER_SECOND);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            render_game(&mut gl, &r, &mut ecs_, &sprite_textures, &render_conf);
        }
	}
}
pub mod animation;
pub mod sprite;

extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use piston::input::RenderArgs;
use opengl_graphics:: {GlGraphics};
use graphics::{Image, clear, draw_state::DrawState};
use graphics::rectangle::square;

use crate::ecs;



pub struct RenderConfig {
    // determines the ratio between pixels and ingame units
    pub scale: f64,
    pub window_xs: u32,
    pub window_ys: u32,
    // basicly central camera
    pub focused_entity: Option<ecs::Entity>
}

pub fn render_game(gl: &mut GlGraphics, args: &RenderArgs, ecs_: &mut ecs::ECS, tex: &sprite::SpriteTextures, conf: &RenderConfig) {
    //TODO dont ignore render layers
    
    // clear screen
    clear([0.0, 0.0, 0.0, 1.0], gl);

    // middle of screen in ingame grid
    let half_window_x = (conf.window_xs as f64 / 2.0) / conf.scale;
    let half_window_y = (conf.window_ys as f64 / 2.0) / conf.scale;

    // offset for focused entity (camera) (default middle of screen)
    let mut x_camera_position = half_window_x;
    let mut y_camera_position = half_window_y;

    if let Some(focused_entity) = conf.focused_entity {
        if let Some(location) = ecs_.location_component.get(focused_entity) {
            x_camera_position = location.x;
            y_camera_position = location.y;
        }
    }

    // iterate over all entities
    for entity in ecs_.allocator.live_indices() {
        if let Some(r_comp) = ecs_.render_component.get(entity) {
            // if entity is not visible we will not render it
            if !r_comp.visible {
                continue;
            }
            if let Some(location) = ecs_.location_component.get(entity) {
                // check if entity is within cameras vision
                if !(x_camera_position - half_window_x <= location.x && location.x <= x_camera_position + half_window_x) {
                    continue;
                }
                if !(y_camera_position - half_window_y <= location.y && location.y <= y_camera_position + half_window_y) {
                    continue;
                }

                // check if texture actually exists
                if let Some(texture) = tex.get(r_comp.base_sprite) {
                    // we got a location so we will do some math
                    let x = (location.x + x_camera_position) * conf.scale - conf.scale / 2.0;
                    let y = (location.y + y_camera_position) * conf.scale - conf.scale / 2.0;
                    let size = conf.scale * r_comp.base_sprite_size;
                    let image = Image::new().rect(square(x, y, size));
                    gl.draw(args.viewport(), |c, gl| {
                        image.draw(texture, &DrawState::default(), c.transform, gl);
                    });
                } else {
                    print!("Texture not found for {:?}", r_comp.base_sprite);
                }
            }
        }
    }
    
}
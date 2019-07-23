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
    pub window_ys: u32
}

pub fn render_game(gl: &mut GlGraphics, args: &RenderArgs, ecs_: &mut ecs::ECS, tex: &sprite::SpriteTextures, conf: &RenderConfig) {
    //TODO dont ignore render layers
    //TODO prevent offscreen rendering

    clear([0.0, 0.0, 0.0, 1.0], gl);

    // iterate over all entities
    for entity in ecs_.allocator.live_indices() {
        if let Some(r_comp) = ecs_.render_component.get(entity) {
            // if element is not visible we will not render it
            if !r_comp.visible {
                continue;
            }
            // check if texture actually exists
            if let Some(texture) = tex.get(r_comp.base_sprite) {
                // find the place to render the sprite
                if let Some(location) = ecs_.location_component.get(entity) {
                    // we got a location so we will do some math
                    let x = location.x * conf.scale;
                    let y = location.y * conf.scale;
                    let size = conf.scale * r_comp.base_sprite_size;
                    let image = Image::new().rect(square(x, y, size));
                    gl.draw(args.viewport(), |c, gl| {
                        image.draw(texture, &DrawState::default(), c.transform, gl);
                    });
                }

            } else {
                print!("Texture not found for {:?}", r_comp.base_sprite);
            }
        }
    }
    
}
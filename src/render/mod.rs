pub mod animation;
pub mod sprite;

use std::collections::BTreeMap;

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
    let mut focused_entity_position_x = half_window_x;
    let mut focused_entity_position_y = half_window_y;

    if let Some(focused_entity) = conf.focused_entity {
        if let Some(location) = ecs_.location_component.get(focused_entity) {
            focused_entity_position_x = location.x;
            focused_entity_position_y = location.y;
        }
    }

    let x_offset = half_window_x - focused_entity_position_x;
    let y_offset = half_window_y - focused_entity_position_y;

    // create render order
    let mut render_levels: BTreeMap<i32, Vec<ecs::Entity>> = BTreeMap::new();

    for entity in ecs_.allocator.live_indices() {
        if let Some(render_c) = ecs_.render_component.get(entity) {
            if !render_c.visible {
                continue;
            }

            if !render_levels.contains_key(&render_c.render_layer) {
                render_levels.insert(render_c.render_layer, Vec::new());
            }

            render_levels.get_mut(&render_c.render_layer).map(|vector| vector.push(entity));
        }
    }

    // render entities in render order
    for (_render_level, entities) in render_levels {
        println!("render level: {:?}\n entities: {:?}\n\n", _render_level, entities);
        for entity in entities {
            let render_c = ecs_.render_component.get(entity)
                                                .expect("No render component, even though it must have one");
            if let Some(location_c) = ecs_.location_component.get(entity) {
                // we need a location to render the entity
                // check if entity is within cameras vision
                if location_c.x + x_offset < 0.0 || location_c.x + x_offset >= conf.window_xs as f64 {
                    continue;
                }
                if location_c.y + y_offset < 0.0 || location_c.y + y_offset >= conf.window_ys as f64 {
                    continue;
                }

                // check if texture actually exists
                if let Some(texture) = tex.get(render_c.base_sprite) {
                    // we got a location so we will do some math
                    let x = (location_c.x + x_offset) * conf.scale - conf.scale / 2.0;
                    let y = (location_c.y + y_offset) * conf.scale - conf.scale / 2.0;
                    let size = conf.scale * render_c.base_sprite_size;
                    let image = Image::new().rect(square(x, y, size));
                    gl.draw(args.viewport(), |c, gl| {
                        image.draw(texture, &DrawState::default(), c.transform, gl);
                    });
                } else {
                    print!("Texture not found for {:?}", render_c.base_sprite);
                }
            }
        }
    }

}
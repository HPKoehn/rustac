use std::collections::HashMap;
use std::path::Path;

extern crate opengl_graphics;

use opengl_graphics:: {Texture, TextureSettings} ;

pub type SpriteId = String;

pub type SpriteTextures = HashMap<SpriteId, Texture>;

pub fn setup_sprite_textures() -> SpriteTextures {
    let mut sprite_textures = SpriteTextures::new();


    sprite_textures.insert("default".to_string(), load_texture("default.png"));
    sprite_textures.insert("floor_tile".to_string(), load_texture("floor_tile.png"));
    sprite_textures.insert("wall_tile".to_string(), load_texture("wall_tile.png"));
    sprite_textures.insert("player".to_string(), load_texture("player.png"));
    
    sprite_textures
}

fn load_texture(name: &str) -> Texture {
    Texture::from_path(Path::new("./assets/textures/").join(name), &TextureSettings::new()).unwrap()
}
use std::collections::HashMap;
use std::path::Path;

extern crate opengl_graphics;

use opengl_graphics:: {Texture, TextureSettings} ;

pub type SpriteId = &'static str;

pub type SpriteTextures = HashMap<SpriteId, Texture>;

pub fn setup_sprite_textures() -> SpriteTextures {
    let mut sprite_textures = SpriteTextures::new();


    sprite_textures.insert("default", load_texture("default.png"));
    sprite_textures
}

fn load_texture(name: &str) -> Texture {
    return Texture::from_path(Path::new("./assets/textures/").join(name), &TextureSettings::new()).unwrap();
}
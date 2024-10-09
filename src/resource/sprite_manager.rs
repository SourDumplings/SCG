use macroquad::prelude::*;
use std::collections::HashMap;

pub struct SpriteManager
{
    textures: HashMap<String, Texture2D>,
}

impl SpriteManager
{
    pub fn new() -> Self
    {
        Self {
            textures: HashMap::new(),
        }
    }

    pub async fn load_texture(&mut self, name: &str, path: &str)
    {
        let texture = load_texture(path).await.unwrap();
        self.textures.insert(name.to_string(), texture);
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture2D>
    {
        self.textures.get(name)
    }
}

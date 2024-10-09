// src/entity/text.rs
use crate::component::{PositionComponent, TextComponent};
use hecs::World;

pub struct Text;

pub struct TextSpawnParams
{
    pub x: f32,
    pub y: f32,
}

impl Text
{
    pub fn new(world: &mut World, params: TextSpawnParams) -> hecs::Entity
    {
        world.spawn((
            PositionComponent {
                x: params.x,
                y: params.y,
            },
            TextComponent {},
        ))
    }
}

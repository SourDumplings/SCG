use crate::component::{PositionComponent, VelocityComponent};
use hecs::{Entity, World};

pub struct Player;

pub struct PlayerSpawnParams
{
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

impl Player
{
    pub fn new(world: &mut World, params: PlayerSpawnParams) -> Entity
    {
        world.spawn((
            PositionComponent {
                x: params.x,
                y: params.y,
            },
            VelocityComponent {
                x: params.vx,
                y: params.vy,
            },
        ))
    }
}

use crate::component::{PositionComponent, VelocityComponent};
use hecs::World;

pub fn logic_tick_system(world: &mut World, delta_time: f32)
{
    for (_, (pos, vel)) in world
        .query::<(&mut PositionComponent, &VelocityComponent)>()
        .iter()
    {
        pos.x += vel.x * delta_time;
        pos.y += vel.y * delta_time;
    }
}

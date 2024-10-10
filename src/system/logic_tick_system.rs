use crate::component::{PositionComponent, VelocityComponent};
use hecs::World;
use log::trace;

pub fn logic_tick_system(world: &mut World, delta_time: f32)
{
    for (entity, (pos, vel)) in world
        .query::<(&mut PositionComponent, &VelocityComponent)>()
        .iter()
    {
        trace!("Got entity {:?} has position and velocity!", entity);
        pos.x += vel.x * delta_time;
        pos.y += vel.y * delta_time;
    }
}

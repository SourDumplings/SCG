use crate::component::{PositionComponent, VelocityComponent};
use hecs::World;
use macroquad::miniquad::log;

pub fn logic_tick_system(world: &mut World, delta_time: f32)
{
    for (entity, (pos, vel)) in world
        .query::<(&mut PositionComponent, &VelocityComponent)>()
        .iter()
    {
        // TODO：设置日志级别并格式化日志
        log!(
            log::Level::Trace,
            "Got entity {:?} has position and velocity!",
            entity
        );
        pos.x += vel.x * delta_time;
        pos.y += vel.y * delta_time;
    }
}

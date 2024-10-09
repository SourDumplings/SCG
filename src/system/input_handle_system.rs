use crate::component::VelocityComponent;
use crate::resource_manager::ResourceManager;
use hecs::World;
use macroquad::prelude::*;
use miniquad::window::set_window_size;

pub fn input_handle_system(world: &mut World, resource_manager: &mut ResourceManager)
{
    if is_key_pressed(KeyCode::Space)
    {
        for (_, vel) in world.query::<&mut VelocityComponent>().iter()
        {
            vel.x *= 2.0;
            vel.y *= 2.0;
        }
    }

    if is_key_pressed(KeyCode::Key1)
    {
        resource_manager.play_hit_sound();
    }

    if is_key_pressed(KeyCode::Key2)
    {
        set_window_size(1366, 768);
    }
}

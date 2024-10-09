use crate::component::VelocityComponent;
use crate::resource::SoundManager;
use hecs::World;
use macroquad::prelude::*;
use miniquad::window::set_window_size;

pub fn input_handle_system(world: &mut World, sound_manager: &mut SoundManager)
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
        sound_manager.play_sound("Hit", false, 1.0);
    }

    if is_key_pressed(KeyCode::Key2)
    {
        set_window_size(1366, 768);
    }
}

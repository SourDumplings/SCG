use crate::components::{Position, Velocity};
use crate::resource_manager::ResourceManager;
use hecs::World;
use macroquad::prelude::*;
use miniquad::window::set_window_size; // 导入 set_window_size 函数

pub fn handle_input(world: &mut World, resource_manager: &mut ResourceManager)
{
    if is_key_pressed(KeyCode::Space)
    {
        for (_, vel) in world.query::<&mut Velocity>().iter()
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

pub fn update_system(world: &mut World, delta_time: f32)
{
    for (_, (pos, vel)) in world.query::<(&mut Position, &Velocity)>().iter()
    {
        pos.x += vel.x * delta_time;
        pos.y += vel.y * delta_time;
    }
}

pub fn draw_system(world: &World, resource_manager: &ResourceManager)
{
    for (_, pos) in world.query::<&Position>().iter()
    {
        if let Some(texture) = resource_manager.get_texture("mushroom")
        {
            draw_texture(&texture, pos.x, pos.y, WHITE);
        }
        else
        {
            draw_circle(pos.x, pos.y, 10.0, WHITE);
        }
        draw_text(
            &format!("Position: ({:.2}, {:.2})", pos.x, pos.y),
            20.0,
            20.0,
            30.0,
            WHITE,
        );

        resource_manager
            .fonts
            .draw_text("你好，世界！", 20.0, 100.0, 30, WHITE);
    }
}

pub fn draw_fps_and_frame_time(resource_manager: &ResourceManager, frame_time: f32, fps: i32)
{
    let fps_text = format!("FPS: {:.2}", fps);
    let frame_time_text = format!("Frame Time: {:.2} ms", frame_time * 1000.0);

    resource_manager
        .fonts
        .draw_text(&fps_text, 20.0, 50.0, 30, WHITE);
    resource_manager
        .fonts
        .draw_text(&frame_time_text, 20.0, 80.0, 30, WHITE);
}

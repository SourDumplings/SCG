use crate::resource_manager::ResourceManager;
use macroquad::prelude::*;

pub fn fps_draw_system(resource_manager: &ResourceManager, frame_time: f32, fps: i32)
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

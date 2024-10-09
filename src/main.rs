mod components;
mod resource_manager;
mod systems;
mod window;

use components::{Position, Velocity};
use hecs::World;
use macroquad::prelude::*;
use resource_manager::ResourceManager;
use rodio::OutputStream;
use std::time::{Duration, Instant};
use systems::{draw_fps_and_frame_time, draw_system, handle_input, update_system};
use tokio::runtime::Builder;
use window::window_conf; // 导入 Instant 和 Duration

#[macroquad::main(window_conf)]
async fn main()
{
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut world = World::new();
    let mut resource_manager = ResourceManager::new(stream_handle);

    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    rt.block_on(async {
        resource_manager
            .load_texture("mushroom", "res/sprite/mushroom.png")
            .await;
        resource_manager
            .load_sound("Hit", "res/sound/Hit.mp3")
            .await;
        resource_manager
            .load_sound("Bgm", "res/sound/WhenTheMorningComes.mp3")
            .await;
        resource_manager
            .load_font("simhei", "res/font/Jinglei.ttf")
            .await;
    });

    resource_manager.play_sound("Bgm", true, 0.5);

    world.spawn((
        Position { x: 100.0, y: 100.0 },
        Velocity { x: 10.0, y: 10.0 },
    ));

    let mut fps_timer = Instant::now();
    let mut fps = 0;
    let mut frame_time = 0.0;
    let mut last_frame_time = Instant::now();
    let target_fps = 120;

    loop
    {
        let now = Instant::now();
        let delta_time = now.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = now;

        handle_input(&mut world, &mut resource_manager);
        update_system(&mut world, delta_time);

        clear_background(RED);
        draw_system(&world, &resource_manager);

        let time_elapsed = fps_timer.elapsed();
        if time_elapsed >= Duration::from_secs(1)
        {
            fps = get_fps();
            frame_time = get_frame_time();
            fps_timer = Instant::now();
        }

        draw_fps_and_frame_time(&resource_manager, frame_time, fps);

        let frame_duration = Duration::from_secs_f32(1.0 / target_fps as f32);
        let elapsed = now.elapsed();
        if elapsed < frame_duration
        {
            std::thread::sleep(frame_duration - elapsed);
        }

        next_frame().await;
    }
}

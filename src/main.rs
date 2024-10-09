mod component;
mod resource;
mod system;
mod window;

use component::{PositionComponent, VelocityComponent};
use hecs::World;
use macroquad::prelude::*;
use resource::{SoundManager, SpriteManager};
use rodio::OutputStream;
use std::time::{Duration, Instant};
use system::{fps_draw_system, input_handle_system, logic_tick_system, render_system};
use tokio::runtime::Builder;
use window::window_conf; // 导入 Instant 和 Duration

#[macroquad::main(window_conf)]
async fn main()
{
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut world = World::new();
    let mut sprite_manager = SpriteManager::new();
    let mut sound_manager = SoundManager::new(stream_handle);

    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    rt.block_on(async {
        sprite_manager
            .load_texture("mushroom", "res/sprite/mushroom.png")
            .await;
        sound_manager.load_sound("Hit", "res/sound/Hit.mp3").await;
        sound_manager
            .load_sound("Bgm", "res/sound/WhenTheMorningComes.mp3")
            .await;
    });

    sound_manager.play_sound("Bgm", true, 0.5);

    world.spawn((
        PositionComponent { x: 100.0, y: 100.0 },
        VelocityComponent { x: 10.0, y: 10.0 },
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

        input_handle_system(&mut world, &mut sound_manager);
        logic_tick_system(&mut world, delta_time);

        clear_background(RED);
        render_system(&world, &sprite_manager);

        let time_elapsed = fps_timer.elapsed();
        if time_elapsed >= Duration::from_secs(1)
        {
            fps = get_fps();
            frame_time = get_frame_time();
            fps_timer = Instant::now();
        }

        fps_draw_system(frame_time, fps);

        let frame_duration = Duration::from_secs_f32(1.0 / target_fps as f32);
        let elapsed = now.elapsed();
        if elapsed < frame_duration
        {
            std::thread::sleep(frame_duration - elapsed);
        }

        next_frame().await;
    }
}

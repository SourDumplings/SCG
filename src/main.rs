mod component;
mod core;
mod entity;
mod resource;
mod system;

use core::logger::{init_logger, test_log}; // 导入 init_logger 函数
use core::window::window_conf;
use core::world::initialize_world;
use hecs::World;
use macroquad::prelude::*;
use macroquad_text::Fonts;
use resource::{SoundManager, SpriteManager};
use rodio::OutputStream;
use std::env;
use std::path::Path;
use std::time::{Duration, Instant}; // 导入 Instant 和 Duration
use system::{fps_draw_system, input_handle_system, logic_tick_system, render_system};
use tokio::runtime::Builder;

#[macroquad::main(window_conf)]
async fn main()
{
    init_logger();
    test_log();
    // TODO：需要支持动态调整日志级别
    // TODO：需要输出环境信息

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut world = World::new();
    let mut sprite_manager = SpriteManager::new();
    let mut sound_manager = SoundManager::new(stream_handle);
    let mut fonts = Fonts::default();

    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    initialize_resources(&mut sprite_manager, &mut sound_manager, &rt, &mut fonts).await;
    initialize_world(&mut world);

    sound_manager.play_sound("Bgm", true, 0.5);

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
        render_system(&world, &sprite_manager, &fonts);

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

async fn initialize_resources<'a>(
    sprite_manager: &mut SpriteManager,
    sound_manager: &mut SoundManager,
    rt: &tokio::runtime::Runtime,
    fonts: &mut Fonts<'a>,
)
{
    let project_root = env!("CARGO_MANIFEST_DIR");
    let mushroom_texture_path = Path::new(project_root).join("res/sprite/mushroom.png");
    let hit_sound_path = Path::new(project_root).join("res/sound/Hit.mp3");
    let bgm_sound_path = Path::new(project_root).join("res/sound/WhenTheMorningComes.mp3");
    let font_path = Path::new(project_root).join("res/font/Jinglei.ttf");

    rt.block_on(async {
        sprite_manager
            .load_texture("mushroom", mushroom_texture_path.to_str().unwrap())
            .await;
        sound_manager
            .load_sound("Hit", hit_sound_path.to_str().unwrap())
            .await;
        sound_manager
            .load_sound("Bgm", bgm_sound_path.to_str().unwrap())
            .await;
        fonts
            .load_font_from_bytes(
                "simhei",
                &std::fs::read(font_path).expect("Failed to read font file"),
            )
            .unwrap();
    });
}

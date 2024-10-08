use hecs::World;
use macroquad::prelude::*;
use macroquad_text::Fonts;
use miniquad::window::set_window_size;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::fs::File as TokioFile;
use tokio::io::AsyncReadExt;
use tokio::runtime::Builder;

struct Position
{
    x: f32,
    y: f32,
}

struct Velocity
{
    x: f32,
    y: f32,
}

// 资源管理器
struct ResourceManager<'a>
{
    textures: HashMap<String, Arc<Texture2D>>,
    sounds: HashMap<String, Vec<u8>>, // 存储音频文件数据
    fonts: Fonts<'a>,
    stream_handle: OutputStreamHandle,
    sinks: HashMap<String, Arc<Sink>>, // 存储 Sink
}

impl<'a> ResourceManager<'a>
{
    fn new(stream_handle: OutputStreamHandle) -> Self
    {
        Self {
            textures: HashMap::new(),
            sounds: HashMap::new(),
            fonts: Fonts::default(),
            stream_handle,
            sinks: HashMap::new(),
        }
    }

    async fn load_texture(&mut self, name: &str, path: &str)
    {
        if !self.textures.contains_key(name)
        {
            // 使用 tokio 异步读取文件
            let mut file = TokioFile::open(path)
                .await
                .expect("Failed to open texture file");
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .await
                .expect("Failed to read texture file");

            // 使用 macroquad 加载纹理
            let texture = Texture2D::from_file_with_format(&buffer, None);
            texture.set_filter(FilterMode::Nearest); // 设置纹理过滤方式为最近邻
            self.textures.insert(name.to_string(), Arc::new(texture));
        }
    }

    async fn load_sound(&mut self, name: &str, path: &str)
    {
        if !self.sounds.contains_key(name)
        {
            // 异步读取文件
            let mut file = TokioFile::open(path)
                .await
                .expect("Failed to open audio file");
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .await
                .expect("Failed to read audio file");
            self.sounds.insert(name.to_string(), buffer);
        }
    }

    async fn load_font(&mut self, name: &'a str, path: &str)
    {
        let font_data = std::fs::read(path).expect("Failed to read font file");
        self.fonts.load_font_from_bytes(name, &font_data).unwrap();
    }

    fn get_texture(&self, name: &str) -> Option<Arc<Texture2D>>
    {
        self.textures.get(name).cloned()
    }

    fn get_sound(&self, name: &str) -> Option<&Vec<u8>>
    {
        self.sounds.get(name)
    }

    fn play_sound(&mut self, name: &str, looped: bool, volume: f32)
    {
        if let Some(buffer) = self.get_sound(name)
        {
            println!("Playing sound: {}", name); // 调试信息
            let cursor = Cursor::new(buffer.clone());
            let source = Decoder::new(cursor).expect("Failed to decode audio file");
            let sink = Sink::try_new(&self.stream_handle).expect("Failed to create sink");
            if looped
            {
                sink.append(source.repeat_infinite());
            }
            else
            {
                sink.append(source);
            }
            sink.set_volume(volume);
            sink.play();
            self.sinks.insert(name.to_string(), Arc::new(sink));
        }
        else
        {
            println!("Sound not found: {}", name); // 调试信息
        }
    }

    fn play_hit_sound(&mut self)
    {
        if let Some(buffer) = self.get_sound("Hit")
        {
            println!("Playing hit sound: Hit"); // 调试信息
            let cursor = Cursor::new(buffer.clone());
            let source = Decoder::new(cursor).expect("Failed to decode audio file");
            let sink = Sink::try_new(&self.stream_handle).expect("Failed to create sink");
            sink.append(source);
            sink.set_volume(1.0);
            sink.play();
            self.sinks.insert("Hit".to_string(), Arc::new(sink));
        }
        else
        {
            println!("Hit sound not found"); // 调试信息
        }
    }
}

fn window_conf() -> Conf
{
    Conf {
        window_title: "Macroquad + Hecs + ResourceManager".to_owned(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        high_dpi: true,
        sample_count: 1,
        platform: miniquad::conf::Platform {
            swap_interval: Some(0), // 1 表示开启 V-Sync，0 表示关闭 V-Sync
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main()
{
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut world = World::new();
    let mut resource_manager = ResourceManager::new(stream_handle);

    // 创建一个 Tokio 运行时
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    // 异步加载资源
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

    // 播放背景音乐
    resource_manager.play_sound("Bgm", true, 0.5);

    // 创建一个实体并添加 Position 和 Velocity 组件
    world.spawn((
        Position { x: 100.0, y: 100.0 },
        Velocity { x: 10.0, y: 10.0 },
    ));

    // 记录 FPS 计算的时间
    let mut fps_timer = Instant::now();

    // 记录渲染帧的 FPS
    let mut fps = 0;
    let mut frame_time = 0.0;

    // 记录上一帧的时间
    let mut last_frame_time = Instant::now();

    let target_fps = 120;

    loop
    {
        // 计算时间增量（delta time）
        let now = Instant::now();
        let delta_time = now.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = now;

        // 处理输入事件
        handle_input(&mut world, &mut resource_manager);

        // 更新游戏逻辑
        update_system(&mut world, delta_time);

        // 渲染
        clear_background(RED);
        draw_system(&world, &resource_manager);

        // 每秒计算真实的 FPS
        let time_elapsed = fps_timer.elapsed();
        if time_elapsed >= Duration::from_secs(1)
        {
            fps = get_fps();
            frame_time = get_frame_time();
            fps_timer = Instant::now();
        }

        // 显示 FPS 和帧时间
        draw_fps_and_frame_time(&resource_manager, frame_time, fps);

        // 限制 FPS 为 target_fps
        let frame_duration = Duration::from_secs_f32(1.0 / target_fps as f32);
        let elapsed = now.elapsed();
        if elapsed < frame_duration
        {
            std::thread::sleep(frame_duration - elapsed);
        }

        next_frame().await;
    }
}

fn handle_input(world: &mut World, resource_manager: &mut ResourceManager)
{
    // 示例：按下空格键时，所有实体的速度翻倍
    if is_key_pressed(KeyCode::Space)
    {
        for (_, vel) in world.query::<&mut Velocity>().iter()
        {
            vel.x *= 2.0;
            vel.y *= 2.0;
        }
    }

    // 按下数字 1 键时播放 "Hit" 声音
    if is_key_pressed(KeyCode::Key1)
    {
        resource_manager.play_hit_sound();
    }

    // 按下数字 2 键时切换窗口大小为 1366 768
    if is_key_pressed(KeyCode::Key2)
    {
        set_window_size(1366, 768);
    }
}

fn update_system(world: &mut World, delta_time: f32)
{
    for (_, (pos, vel)) in world.query::<(&mut Position, &Velocity)>().iter()
    {
        pos.x += vel.x * delta_time;
        pos.y += vel.y * delta_time;
    }
}

fn draw_system(world: &World, resource_manager: &ResourceManager)
{
    for (_, pos) in world.query::<&Position>().iter()
    {
        if let Some(texture) = resource_manager.get_texture("mushroom")
        {
            // 使用整数像素位置
            draw_texture(&texture, pos.x.round(), pos.y.round(), WHITE);
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

        // 渲染中文字体
        resource_manager
            .fonts
            .draw_text("你好，世界！", 20.0, 100.0, 30, WHITE);
    }
}

fn draw_fps_and_frame_time(resource_manager: &ResourceManager, frame_time: f32, fps: i32)
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

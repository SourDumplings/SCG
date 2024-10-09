use macroquad::prelude::*;
use macroquad_text::Fonts;
use rodio::{Decoder, OutputStreamHandle, Sink, Source}; // 导入 Source trait
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncReadExt;

pub struct ResourceManager<'a>
{
    pub textures: HashMap<String, Arc<Texture2D>>,
    pub sounds: HashMap<String, Vec<u8>>,
    pub fonts: Fonts<'a>, // 将 fonts 字段设为公有
    stream_handle: OutputStreamHandle,
    sinks: HashMap<String, Arc<Sink>>,
}

impl<'a> ResourceManager<'a>
{
    pub fn new(stream_handle: OutputStreamHandle) -> Self
    {
        Self {
            textures: HashMap::new(),
            sounds: HashMap::new(),
            fonts: Fonts::default(),
            stream_handle,
            sinks: HashMap::new(),
        }
    }

    pub async fn load_texture(&mut self, name: &str, path: &str)
    {
        if !self.textures.contains_key(name)
        {
            let mut file = TokioFile::open(path)
                .await
                .expect("Failed to open texture file");
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .await
                .expect("Failed to read texture file");

            let texture = Texture2D::from_file_with_format(&buffer, None);
            texture.set_filter(FilterMode::Nearest);
            self.textures.insert(name.to_string(), Arc::new(texture));
        }
    }

    pub async fn load_sound(&mut self, name: &str, path: &str)
    {
        if !self.sounds.contains_key(name)
        {
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

    pub async fn load_font(&mut self, name: &'a str, path: &str)
    {
        let font_data = std::fs::read(path).expect("Failed to read font file");
        self.fonts.load_font_from_bytes(name, &font_data).unwrap();
    }

    pub fn get_texture(&self, name: &str) -> Option<Arc<Texture2D>>
    {
        self.textures.get(name).cloned()
    }

    pub fn get_sound(&self, name: &str) -> Option<&Vec<u8>>
    {
        self.sounds.get(name)
    }

    pub fn play_sound(&mut self, name: &str, looped: bool, volume: f32)
    {
        if let Some(buffer) = self.get_sound(name)
        {
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
    }

    pub fn play_hit_sound(&mut self)
    {
        if let Some(buffer) = self.get_sound("Hit")
        {
            let cursor = Cursor::new(buffer.clone());
            let source = Decoder::new(cursor).expect("Failed to decode audio file");
            let sink = Sink::try_new(&self.stream_handle).expect("Failed to create sink");
            sink.append(source);
            sink.set_volume(1.0);
            sink.play();
            self.sinks.insert("Hit".to_string(), Arc::new(sink));
        }
    }
}

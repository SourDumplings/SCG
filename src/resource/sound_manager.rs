use rodio::{Decoder, OutputStreamHandle, Sink, Source};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub struct SoundManager
{
    sounds: HashMap<String, Vec<u8>>,
    stream_handle: OutputStreamHandle,
}

impl SoundManager
{
    pub fn new(stream_handle: OutputStreamHandle) -> Self
    {
        Self {
            sounds: HashMap::new(),
            stream_handle,
        }
    }

    pub async fn load_sound(&mut self, name: &str, path: &str)
    {
        let file = File::open(path).expect("Failed to open sound file");
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Failed to read sound file");
        self.sounds.insert(name.to_string(), buffer);
    }

    pub fn play_sound(&self, name: &str, looped: bool, volume: f32)
    {
        if let Some(sound_data) = self.sounds.get(name)
        {
            let cursor = std::io::Cursor::new(sound_data.clone());
            let source = Decoder::new(cursor).expect("Failed to decode sound");
            let sink = Sink::try_new(&self.stream_handle).expect("Failed to create sink");
            sink.set_volume(volume);
            if looped
            {
                sink.append(source.repeat_infinite());
            }
            else
            {
                sink.append(source);
            }
            sink.detach(); // Detach the sink to play the sound asynchronously
        }
        else
        {
            eprintln!("Sound not found: {}", name);
        }
    }
}

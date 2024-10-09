use rodio::{Decoder, OutputStreamHandle, Sink, Source};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub struct SoundManager
{
    sounds: HashMap<String, Sink>,
    sound_paths: HashMap<String, String>,
    stream_handle: OutputStreamHandle,
}

impl SoundManager
{
    pub fn new(stream_handle: OutputStreamHandle) -> Self
    {
        Self {
            sounds: HashMap::new(),
            sound_paths: HashMap::new(),
            stream_handle,
        }
    }

    pub async fn load_sound(&mut self, name: &str, path: &str)
    {
        let file = File::open(path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.append(source);
        self.sounds.insert(name.to_string(), sink);
        self.sound_paths.insert(name.to_string(), path.to_string());
    }

    pub fn play_sound(&self, name: &str, looped: bool, volume: f32)
    {
        if let Some(sink) = self.sounds.get(name)
        {
            sink.set_volume(volume);
            if looped
            {
                if let Some(path) = self.sound_paths.get(name)
                {
                    let file = File::open(path).unwrap();
                    let source = Decoder::new(BufReader::new(file))
                        .unwrap()
                        .repeat_infinite();
                    sink.append(source);
                }
            }
            sink.play();
        }
    }
}

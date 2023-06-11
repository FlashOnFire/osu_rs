use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, OutputStream, Source};
use rodio::{OutputStreamHandle, Sink};

#[allow(dead_code)]
pub struct MusicManager {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
}

impl MusicManager {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();

        Self {
            sink: Sink::try_new(&stream_handle).unwrap(),
            stream,
            stream_handle,
        }
    }

    pub fn play_file(&mut self, filename: &str) {
        let file = BufReader::new(File::open(format!("assets/{}", filename)).unwrap());
        let source = Decoder::new(file).unwrap();

        self.sink.append(source.repeat_infinite());
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.sink.set_volume(volume);
    }

    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }
}

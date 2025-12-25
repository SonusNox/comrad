use rodio::{Decoder, OutputStream, Sink};

use std::{
    error::Error,
    fs::File,
    time::Duration
};

use crate::utils::data;

impl Playback {
    //** Misc. **//
    pub fn clear_playlist(&mut self) {
        self.sink = None;
    }

    pub fn new() -> Self{
        let (sink, stream) = data::get_stream();

        Self {
            stream: Some(stream),
            sink: Some(sink)
        }
    }

    pub fn try_seek(&self, elapsed_time: Duration) {
        if let Some(sink) = &self.sink { let _ = sink.try_seek(elapsed_time); };
    }

    //** Playback **//
    pub fn pause(&mut self) {
        if let Some(sink) = &self.sink {
            sink.stop();

            if let Some(stream) = &self.stream {
                self.sink = Some(Sink::connect_new(stream.mixer()));
            };
        };
    }

    pub fn play(&mut self, source: String) -> Result<(), Box<dyn Error>> {
        let file = File::open(source)?;
        let decoded = Decoder::try_from(file)?;

        if let Some(sink) = &self.sink {
            sink.append(decoded);
            sink.play();
        };

        Ok(())
    }

    //** Setters **//
    pub fn set_volume(&mut self, val: f32) {
        if let Some(sink) = &self.sink { sink.set_volume(val); };
    }
}

pub struct Playback {
    // Playback
    stream: Option<OutputStream>,
    sink: Option<Sink>
}
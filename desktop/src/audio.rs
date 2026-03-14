use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};

pub struct AudioDevice {
    _stream: Stream,
    playing: Arc<Mutex<bool>>,
}

impl AudioDevice {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output audio device");
        let config = device
            .default_output_config()
            .expect("default output config");
        let sample_rate = config.sample_rate().0 as f32;
        let channels = config.channels() as usize;

        let playing = Arc::new(Mutex::new(false));
        let playing_clone = playing.clone();

        let mut phase: f32 = 0.0;
        let phase_inc = 440.0 / sample_rate;

        let stream_config: StreamConfig = config.into();
        let stream = device
            .build_output_stream(
                &stream_config,
                move |data: &mut [f32], _| {
                    let is_playing = *playing_clone.lock().unwrap();
                    for frame in data.chunks_mut(channels) {
                        let sample = if is_playing {
                            0.25 * (2.0 * PI * phase).sin()
                        } else {
                            0.0
                        };
                        if is_playing {
                            phase = (phase + phase_inc) % 1.0;
                        }
                        for s in frame.iter_mut() {
                            *s = sample;
                        }
                    }
                },
                |err| eprintln!("audio stream error: {err}"),
                None,
            )
            .expect("build output stream");

        stream.play().expect("start audio stream");

        Self {
            _stream: stream,
            playing,
        }
    }

    pub fn play(&mut self) {
        *self.playing.lock().unwrap() = true;
    }

    pub fn pause(&mut self) {
        *self.playing.lock().unwrap() = false;
    }
}

use std::f32::consts::PI;

use sdl2::{
    audio::{AudioCallback, AudioSpecDesired, AudioStatus},
    Sdl,
};

struct SineWave {
    pub phase_inc: f32,
    pub phase: f32,
    pub volume: f32,
}

impl AudioCallback for SineWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = self.volume * (2.0 * PI * self.phase).sin();
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct AudioDevice {
    device: sdl2::audio::AudioDevice<SineWave>,
}

impl AudioDevice {
    pub fn new(sdl_context: &Sdl) -> Self {
        let audio_subsystem = sdl_context.audio().expect("audio subsystem for sdl2");
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };
        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| SineWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            })
            .expect("audio device");

        Self { device }
    }

    pub fn play(&mut self) {
        if self.device.status() != AudioStatus::Playing {
            self.device.resume();
        }
    }

    pub fn pause(&mut self) {
        if self.device.status() == AudioStatus::Playing {
            self.device.pause();
        }
    }
}

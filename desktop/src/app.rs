use crate::audio::AudioDevice;
use crate::input::map_keycode;
use crate::window::WindowState;
use chip8_emulator::Emulator;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowId;

pub struct App {
    emulator: Emulator,
    audio: AudioDevice,
    state: Option<WindowState>,
    last_tick: Instant,
}

impl App {
    pub fn new(rom: &[u8]) -> Self {
        let mut emulator = Emulator::new();
        emulator.reset();
        emulator.load_rom(rom);

        Self {
            emulator,
            audio: AudioDevice::new(),
            state: None,
            last_tick: Instant::now(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.state = Some(WindowState::new(event_loop));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state,
                        ..
                    },
                ..
            } => {
                if key == KeyCode::Escape {
                    event_loop.exit();
                    return;
                }
                if let Some(chip8_key) = map_keycode(key) {
                    match state {
                        ElementState::Pressed => self.emulator.keypad.key_down(chip8_key),
                        ElementState::Released => self.emulator.keypad.key_up(chip8_key),
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(state) = &mut self.state {
                    state.render(self.emulator.display.get_buffer());
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let frame_duration = Duration::from_nanos(1_000_000_000 / 60);
        let next_tick = self.last_tick + frame_duration;
        let now = Instant::now();

        if now >= next_tick {
            self.last_tick = now;

            for _ in 0..10 {
                self.emulator.execute_instruction_cycle();
            }
            self.emulator.decrement_timers();

            if self.emulator.is_sound_playing() {
                self.audio.play();
            } else {
                self.audio.pause();
            }

            if let Some(state) = &self.state {
                state.window.request_redraw();
            }
        }

        event_loop.set_control_flow(ControlFlow::WaitUntil(self.last_tick + frame_duration));
    }
}

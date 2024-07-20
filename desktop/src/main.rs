mod audio;
mod display;

use audio::AudioDevice;
use chip8_emulator::Emulator;
use display::DisplayDevice;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::fs;
use std::path::Path;
use std::time::Duration;

fn get_rom_file_path() -> String {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <rom_file_path>", args[0]);
        std::process::exit(1);
    }
    let rom_file_path = args[1].clone();

    if !Path::new(&rom_file_path).is_file() {
        eprintln!("The rom_file_path should be a valid file.");
        std::process::exit(1);
    }

    rom_file_path
}

fn map_keycode(keycode: Keycode) -> Option<u8> {
    match keycode {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xc),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xd),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xe),
        Keycode::Z => Some(0xa),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xb),
        Keycode::V => Some(0xf),
        _ => None,
    }
}

fn main() {
    let rom_file_path = get_rom_file_path();
    let rom = fs::read(rom_file_path).expect("open rom file");

    let mut emulator = Emulator::new();
    emulator.reset();
    emulator.load_rom(&rom);

    let sdl_context = sdl2::init().expect("sdl2 init");

    let mut display_device = DisplayDevice::new(&sdl_context);
    let mut audio_device = AudioDevice::new(&sdl_context);

    let mut event_pump = sdl_context.event_pump().expect("sdl2 event pump");

    'game: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = map_keycode(keycode) {
                        emulator.keypad.key_down(key);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = map_keycode(keycode) {
                        emulator.keypad.key_up(key);
                    }
                }
                _ => {}
            }
        }

        for _ in 0..10 {
            emulator.execute_instruction_cycle();
        }
        emulator.decrement_timers();

        display_device.render(emulator.display.get_buffer());

        if emulator.is_sound_playing() {
            audio_device.play();
        } else {
            audio_device.pause();
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

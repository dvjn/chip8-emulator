mod app;
mod audio;
mod display;
mod input;
mod window;

use std::fs;
use std::path::Path;
use winit::event_loop::EventLoop;

fn get_rom_file_path() -> String {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <rom_file_path>", args[0]);
        std::process::exit(1);
    }
    let path = args[1].clone();
    if !Path::new(&path).is_file() {
        eprintln!("The rom_file_path should be a valid file.");
        std::process::exit(1);
    }
    path
}

fn main() {
    let rom_file_path = get_rom_file_path();
    let rom = fs::read(rom_file_path).expect("open rom file");

    let event_loop = EventLoop::new().expect("create event loop");
    let mut app = app::App::new(&rom);
    event_loop.run_app(&mut app).expect("run app");
}

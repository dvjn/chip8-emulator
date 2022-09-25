mod utils;

use chip8_emulator_lib::cpu::Cpu;
use js_sys::Array;
use std::sync::Mutex;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();
}

static CPU: Mutex<Cpu> = Mutex::new(Cpu::new());

#[wasm_bindgen]
pub fn load_rom(rom: &[u8]) {
    CPU.lock().unwrap().load_rom(rom);
}

#[wasm_bindgen]
pub fn reset() {
    CPU.lock().unwrap().reset();
}

#[wasm_bindgen]
pub fn execute_instruction_cycle() {
    CPU.lock().unwrap().execute_instruction_cycle();
}

#[wasm_bindgen]
pub fn decrement_timers() {
    CPU.lock().unwrap().decrement_timers();
}

#[wasm_bindgen]
pub fn get_display_buffer() -> Array {
    let cpu = CPU.lock().unwrap();
    let display_buffer = cpu.display.get_buffer();
    let js_buffer = Array::new_with_length(display_buffer.len() as u32);

    display_buffer
        .iter()
        .enumerate()
        .for_each(|(i, value)| js_buffer.set(i as u32, JsValue::from_bool(*value)));

    js_buffer
}

#[wasm_bindgen]
pub fn is_sound_playing() -> JsValue {
    let cpu = CPU.lock().unwrap();
    JsValue::from_bool(cpu.is_sound_playing())
}

#[wasm_bindgen]
pub fn set_key_down(key: u8) {
    CPU.lock().unwrap().keypad.key_down(key);
}

#[wasm_bindgen]
pub fn set_key_up(key: u8) {
    CPU.lock().unwrap().keypad.key_up(key);
}

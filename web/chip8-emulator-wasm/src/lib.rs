#![allow(clippy::new_without_default)]

mod utils;

use chip8_emulator_lib::cpu::Cpu;
use js_sys::Array;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();
}

#[wasm_bindgen]
pub struct Emulator {
    cpu: Cpu,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Emulator { cpu: Cpu::new() }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.cpu.load_rom(rom);
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn execute_instruction_cycle(&mut self) {
        self.cpu.execute_instruction_cycle();
    }

    pub fn decrement_timers(&mut self) {
        self.cpu.decrement_timers();
    }

    pub fn get_display_buffer(&mut self) -> Array {
        let display_buffer = self.cpu.display.get_buffer();
        let js_buffer = Array::new_with_length(display_buffer.len() as u32);

        display_buffer
            .iter()
            .enumerate()
            .for_each(|(i, value)| js_buffer.set(i as u32, JsValue::from_bool(*value)));

        js_buffer
    }

    pub fn is_sound_playing(&mut self) -> JsValue {
        JsValue::from_bool(self.cpu.is_sound_playing())
    }

    pub fn set_key_down(&mut self, key: u8) {
        self.cpu.keypad.key_down(key);
    }

    pub fn set_key_up(&mut self, key: u8) {
        self.cpu.keypad.key_up(key);
    }
}

pub const SCALE: u32 = 15;
pub const CHIP8_WIDTH: u32 = 64;
pub const CHIP8_HEIGHT: u32 = 32;
pub const WINDOW_WIDTH: u32 = CHIP8_WIDTH * SCALE;
pub const WINDOW_HEIGHT: u32 = CHIP8_HEIGHT * SCALE;

pub fn render(frame: &mut [u32], buffer: &[bool; 2048]) {
    for (i, pixel) in frame.iter_mut().enumerate() {
        let x = (i as u32) % WINDOW_WIDTH;
        let y = (i as u32) / WINDOW_WIDTH;
        let chip8_x = x / SCALE;
        let chip8_y = y / SCALE;
        let on = buffer[(chip8_y * CHIP8_WIDTH + chip8_x) as usize];
        *pixel = if on { 0x00ffffff } else { 0x00000000 };
    }
}

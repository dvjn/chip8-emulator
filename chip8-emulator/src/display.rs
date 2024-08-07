pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_PIXELS: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

#[derive(Debug)]
pub struct Display {
    buffer: [bool; DISPLAY_PIXELS],
}

impl Display {
    pub const fn new() -> Self {
        Self {
            buffer: [false; DISPLAY_PIXELS],
        }
    }

    pub fn get_buffer(&self) -> &[bool; DISPLAY_PIXELS] {
        &self.buffer
    }

    pub fn cls(&mut self) {
        self.buffer = [false; DISPLAY_PIXELS];
    }

    fn set_pixel(&mut self, x: usize, y: usize, pixel: bool) {
        self.buffer[x + y * DISPLAY_WIDTH] = pixel;
    }

    fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.buffer[x + y * DISPLAY_WIDTH]
    }

    fn xor_pixel(&mut self, x: usize, y: usize, value: bool) -> bool {
        let current_value = self.get_pixel(x, y);
        let new_value = current_value ^ value;

        self.set_pixel(x, y, new_value);

        current_value && value
    }

    pub fn draw(&mut self, x: usize, y: usize, sprites: &[u8]) -> bool {
        let mut collision = false;

        let rows = (0..sprites.len())
            .map(|row| (y + row).rem_euclid(DISPLAY_HEIGHT))
            .collect::<Vec<usize>>();
        let cols = (0..8)
            .map(|col| (x + col).rem_euclid(DISPLAY_WIDTH))
            .collect::<Vec<usize>>();

        for (j, &y) in rows.iter().enumerate() {
            for (i, &x) in cols.iter().enumerate() {
                let value = (sprites[j] >> (7 - i)) & 0x01 > 0;

                collision |= self.xor_pixel(x, y, value);
            }
        }

        collision
    }
}

pub const FONT_SPRITES: [u8; 5 * 16] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // c
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_digit_0() {
        let mut display = Display::new();

        let collision = display.draw(0, 0, &FONT_SPRITES[0..5]);

        assert!(!collision);

        assert!(display.buffer[0]);
        assert!(display.buffer[1]);
        assert!(display.buffer[2]);
        assert!(display.buffer[3]);

        assert!(display.buffer[DISPLAY_WIDTH]);
        assert!(display.buffer[3 + DISPLAY_WIDTH]);

        assert!(display.buffer[DISPLAY_WIDTH * 2]);
        assert!(display.buffer[3 + DISPLAY_WIDTH * 2]);

        assert!(display.buffer[DISPLAY_WIDTH * 3]);
        assert!(display.buffer[3 + DISPLAY_WIDTH * 3]);

        assert!(display.buffer[DISPLAY_WIDTH * 4]);
        assert!(display.buffer[1 + DISPLAY_WIDTH * 4]);
        assert!(display.buffer[2 + DISPLAY_WIDTH * 4]);
        assert!(display.buffer[3 + DISPLAY_WIDTH * 4]);
    }

    #[test]
    fn display_digit_0_wrapped() {
        let mut display = Display::new();

        let collision = display.draw(DISPLAY_WIDTH - 2, DISPLAY_HEIGHT - 2, &FONT_SPRITES[0..5]);

        assert!(!collision);

        assert!(display.buffer[DISPLAY_WIDTH - 2 + DISPLAY_WIDTH * (DISPLAY_HEIGHT - 2)]);
        assert!(display.buffer[DISPLAY_WIDTH - 1 + DISPLAY_WIDTH * (DISPLAY_HEIGHT - 2)]);
        assert!(display.buffer[DISPLAY_WIDTH * (DISPLAY_HEIGHT - 2)]);
        assert!(display.buffer[1 + DISPLAY_WIDTH * (DISPLAY_HEIGHT - 2)]);

        assert!(display.buffer[DISPLAY_WIDTH - 2 + DISPLAY_WIDTH * (DISPLAY_HEIGHT - 1)]);
        assert!(display.buffer[1 + DISPLAY_WIDTH * (DISPLAY_HEIGHT - 1)]);

        assert!(display.buffer[DISPLAY_WIDTH - 2]);
        assert!(display.buffer[1]);

        assert!(display.buffer[DISPLAY_WIDTH - 2 + DISPLAY_WIDTH]);
        assert!(display.buffer[1 + DISPLAY_WIDTH]);

        assert!(display.buffer[DISPLAY_WIDTH - 2 + DISPLAY_WIDTH * 2]);
        assert!(display.buffer[DISPLAY_WIDTH - 1 + DISPLAY_WIDTH * 2]);
        assert!(display.buffer[DISPLAY_WIDTH * 2]);
        assert!(display.buffer[1 + DISPLAY_WIDTH * 2]);
    }
}

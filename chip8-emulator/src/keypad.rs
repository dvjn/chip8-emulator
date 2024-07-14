#[derive(Debug)]
pub struct Keypad {
    keys: [bool; 16],
}

impl Keypad {
    pub const fn new() -> Self {
        Self { keys: [false; 16] }
    }

    pub fn clear(&mut self) {
        self.keys = [false; 16];
    }

    pub fn get_key(&self, key: u8) -> bool {
        self.keys
            .get(key as usize)
            .expect("key should be between 0 and 15")
            .to_owned()
    }

    pub fn key_down(&mut self, key: u8) {
        let key = self
            .keys
            .get_mut(key as usize)
            .expect("key should be between 0 and 15");

        *key = true;
    }

    pub fn key_up(&mut self, key: u8) {
        let key = self
            .keys
            .get_mut(key as usize)
            .expect("key should be between 0 and 15");

        *key = false;
    }
}

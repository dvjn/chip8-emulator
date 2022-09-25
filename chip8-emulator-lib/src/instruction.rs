#[derive(Debug)]
pub struct Instruction(pub u8, pub u8, pub u8, pub u8);

impl Instruction {
    pub fn from_opcode(opcode: u16) -> Self {
        Self(
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        )
    }

    pub fn read(memory: &[u8; 4096], location: u16) -> Self {
        let first_byte = memory[location as usize];
        let second_byte = memory[location as usize + 1];

        Self(
            (first_byte & 0xF0) >> 4,
            first_byte & 0x0F,
            (second_byte & 0xF0) >> 4,
            second_byte & 0x0F,
        )
    }

    pub fn nnn(&self) -> u16 {
        (self.1 as u16) << 8 | (self.2 as u16) << 4 | (self.3 as u16)
    }

    pub fn n(&self) -> u8 {
        self.3
    }

    pub fn x(&self) -> u8 {
        self.1
    }

    pub fn y(&self) -> u8 {
        self.2
    }

    pub fn kk(&self) -> u8 {
        self.2 << 4 | self.3
    }
}

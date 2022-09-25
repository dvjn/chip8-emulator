use rand::Rng;

use crate::{
    display::{Display, FONT_SPRITES},
    instruction::Instruction,
    keypad::Keypad,
};

#[derive(Debug)]
pub struct Cpu {
    memory: [u8; 4096],
    v_registers: [u8; 16],
    i_register: u16,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],

    pub display: Display,
    pub keypad: Keypad,
}

impl Cpu {
    pub const fn new() -> Cpu {
        Cpu {
            memory: [0; 4096],
            v_registers: [0; 16],
            i_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0,
            stack_pointer: 0,
            stack: [0; 16],

            display: Display::new(),
            keypad: Keypad::new(),
        }
    }

    pub fn reset(&mut self) {
        self.memory = [0; 4096];
        self.v_registers = [0; 16];
        self.i_register = 0;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.stack_pointer = 0;
        self.stack = [0; 16];

        self.memory[0..80].copy_from_slice(&FONT_SPRITES);
        self.program_counter = 0x200;

        self.display.cls();
        self.keypad.clear();
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory[0x200..0x200 + rom.len()].copy_from_slice(&rom);
    }

    pub fn decrement_timers(&mut self) {
        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);
    }

    pub fn debug_info(&self) -> String {
        format!(
            "v: {:?}, i: {:?}, sp: {:?}, stack: {:?}, dt: {:?}, pc: {:?}, instruction: {:?}",
            self.v_registers,
            self.i_register,
            self.stack_pointer,
            self.stack,
            self.delay_timer,
            self.program_counter,
            Instruction::read(&self.memory, self.program_counter)
        )
    }

    pub fn is_sound_playing(&self) -> bool {
        self.sound_timer > 0
    }

    pub fn execute_instruction_cycle(&mut self) {
        let instruction = Instruction::read(&self.memory, self.program_counter);
        self.execute_instruction(instruction);
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        self.program_counter += 2;

        match instruction {
            Instruction(0x0, 0x0, 0xE, 0x0) => {
                // 00E0 - CLS
                self.display.cls()
            }
            Instruction(0x0, 0x0, 0xE, 0xE) => {
                // 00EE - RET
                self.stack_pointer -= 1;
                self.program_counter = self.stack[self.stack_pointer as usize];
            }
            Instruction(0x0, _, _, _) => {
                // 0nnn - SYS addr
                ()
            }
            Instruction(0x1, _, _, _) => {
                // 1nnn - JP addr
                self.program_counter = instruction.nnn();
            }
            Instruction(0x2, _, _, _) => {
                // 2nnn - CALL addr
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer += 1;
                self.program_counter = instruction.nnn();
            }
            Instruction(0x3, _, _, _) => {
                // 3xkk - SE Vx, byte
                let vx = self.v_registers[instruction.x() as usize];
                let kk = instruction.kk();

                if vx == kk {
                    self.program_counter += 2;
                }
            }
            Instruction(0x4, _, _, _) => {
                // 4xkk - SNE Vx, byte
                let vx = self.v_registers[instruction.x() as usize];
                let kk = instruction.kk();

                if vx != kk {
                    self.program_counter += 2;
                }
            }
            Instruction(0x5, _, _, 0x0) => {
                // 5xy0 - SE Vx, Vy
                let vx = self.v_registers[instruction.x() as usize];
                let vy = self.v_registers[instruction.y() as usize];

                if vx == vy {
                    self.program_counter += 2;
                }
            }
            Instruction(0x6, _, _, _) => {
                // 6xkk - LD Vx, byte
                self.v_registers[instruction.x() as usize] = instruction.kk();
            }
            Instruction(0x7, _, _, _) => {
                // 7xkk - ADD Vx, byte
                self.v_registers[instruction.x() as usize] =
                    self.v_registers[instruction.x() as usize].wrapping_add(instruction.kk());
            }
            Instruction(0x8, _, _, 0x0) => {
                // 8xy0 - LD Vx, Vy
                self.v_registers[instruction.x() as usize] =
                    self.v_registers[instruction.y() as usize];
            }
            Instruction(0x8, _, _, 0x1) => {
                // 8xy1 - OR Vx, Vy
                self.v_registers[instruction.x() as usize] = self.v_registers
                    [instruction.x() as usize]
                    | self.v_registers[instruction.y() as usize];
            }
            Instruction(0x8, _, _, 0x2) => {
                // 8xy2 - AND Vx, Vy
                self.v_registers[instruction.x() as usize] = self.v_registers
                    [instruction.x() as usize]
                    & self.v_registers[instruction.y() as usize];
            }
            Instruction(0x8, _, _, 0x3) => {
                // 8xy3 - XOR Vx, Vy
                self.v_registers[instruction.x() as usize] = self.v_registers
                    [instruction.x() as usize]
                    ^ self.v_registers[instruction.y() as usize];
            }
            Instruction(0x8, _, _, 0x4) => {
                // 8xy4 - ADD Vx, Vy
                let (vx, overflow) = self.v_registers[instruction.x() as usize]
                    .overflowing_add(self.v_registers[instruction.y() as usize]);

                self.v_registers[instruction.x() as usize] = vx;
                self.v_registers[0xF] = if overflow { 0x1 } else { 0x0 };
            }
            Instruction(0x8, _, _, 0x5) => {
                // 8xy5 - SUB Vx, Vy
                let (vx, overflow) = self.v_registers[instruction.x() as usize]
                    .overflowing_sub(self.v_registers[instruction.y() as usize]);

                self.v_registers[instruction.x() as usize] = vx;
                self.v_registers[0xF] = if overflow { 0x0 } else { 0x1 };
            }
            Instruction(0x8, _, _, 0x6) => {
                // 8xy6 - SHR Vx {, Vy}

                let vx = self.v_registers[instruction.x() as usize];
                let (vx, overflow) = (vx / 2, vx % 2);

                self.v_registers[instruction.x() as usize] = vx;
                self.v_registers[0xF] = overflow;
            }
            Instruction(0x8, _, _, 0x7) => {
                // 8xy7 - SUBN Vx, Vy
                let (vx, overflow) = self.v_registers[instruction.y() as usize]
                    .overflowing_sub(self.v_registers[instruction.x() as usize]);

                self.v_registers[instruction.x() as usize] = vx;
                self.v_registers[0xF] = if overflow { 0x0 } else { 0x1 };
            }
            Instruction(0x8, _, _, 0xE) => {
                // 8xyE - SHL Vx {, Vy}
                let (vx, overflow) = self.v_registers[instruction.x() as usize].overflowing_mul(2);

                self.v_registers[instruction.x() as usize] = vx;
                self.v_registers[0xF] = if overflow { 0x1 } else { 0x0 };
            }
            Instruction(0x9, _, _, 0x0) => {
                // 9xy0 - SNE Vx, Vy
                let vx = self.v_registers[instruction.x() as usize];
                let vy = self.v_registers[instruction.y() as usize];

                if vx != vy {
                    self.program_counter += 2;
                }
            }
            Instruction(0xA, _, _, _) => {
                // Annn - LD I, addr
                self.i_register = instruction.nnn();
            }
            Instruction(0xB, _, _, _) => {
                // Bnnn - JP V0, addr
                self.program_counter = instruction.nnn().wrapping_add(self.v_registers[0x0] as u16);
            }
            Instruction(0xC, _, _, _) => {
                // Cxkk - RND Vx, byte
                self.v_registers[instruction.x() as usize] =
                    rand::thread_rng().gen::<u8>() & instruction.kk();
            }
            Instruction(0xD, _, _, _) => {
                // Dxyn - DRW Vx, Vy, nibble
                let x = instruction.x() as usize;
                let y = instruction.y() as usize;
                let mem_start = self.i_register as usize;
                let mem_end = mem_start + instruction.n() as usize;

                let collision = self.display.draw(
                    self.v_registers[x] as usize,
                    self.v_registers[y] as usize,
                    &self.memory[mem_start..mem_end],
                );

                self.v_registers[0xF] = if collision { 0x1 } else { 0x0 };
            }
            Instruction(0xE, _, 0x9, 0xE) => {
                // Ex9E - SKP Vx
                let key_pressed = self
                    .keypad
                    .get_key(self.v_registers[instruction.x() as usize]);
                if key_pressed {
                    self.program_counter += 2;
                }
            }
            Instruction(0xE, _, 0xA, 0x1) => {
                // ExA1 - SKNP Vx
                let key_pressed = self
                    .keypad
                    .get_key(self.v_registers[instruction.x() as usize]);
                if !key_pressed {
                    self.program_counter += 2;
                }
            }
            Instruction(0xF, _, 0x0, 0x7) => {
                // Fx07 - LD Vx, DT
                self.v_registers[instruction.x() as usize] = self.delay_timer;
            }
            Instruction(0xF, _, 0x0, 0xA) => {
                // Fx0A - LD Vx, K
                self.program_counter -= 2;

                for key in 0x0..=0xF {
                    if self.keypad.get_key(key) {
                        self.v_registers[instruction.x() as usize] = key;
                        self.program_counter += 2;
                    }
                }
            }
            Instruction(0xF, _, 0x1, 0x5) => {
                // Fx15 - LD DT, Vx
                self.delay_timer = self.v_registers[instruction.x() as usize];
            }
            Instruction(0xF, _, 0x1, 0x8) => {
                // Fx18 - LD ST, Vx
                self.sound_timer = self.v_registers[instruction.x() as usize];
            }
            Instruction(0xF, _, 0x1, 0xE) => {
                // Fx1E - ADD I, Vx
                self.i_register = self
                    .i_register
                    .wrapping_add(self.v_registers[instruction.x() as usize] as u16);
            }
            Instruction(0xF, _, 0x2, 0x9) => {
                // Fx29 - LD F, Vx

                // Font sprites kept at 0..80
                // 0 => 0..5, 1 => 5..10, 2 => 10..15, and so on.
                self.i_register = self.v_registers[instruction.x() as usize] as u16 * 5;
            }
            Instruction(0xF, _, 0x3, 0x3) => {
                // Fx33 - LD B, Vx
                let vx = self.v_registers[instruction.x() as usize];

                self.memory[self.i_register as usize] = vx.div_euclid(100).rem_euclid(10);
                self.memory[self.i_register as usize + 1] = vx.div_euclid(10).rem_euclid(10);
                self.memory[self.i_register as usize + 2] = vx.rem_euclid(10);
            }
            Instruction(0xF, _, 0x5, 0x5) => {
                // Fx55 - LD [I], Vx
                let x = instruction.x() as usize;
                let mem_start = self.i_register as usize;
                let mem_end = mem_start + x;

                self.memory[mem_start..=mem_end].copy_from_slice(&self.v_registers[0..=x]);
            }
            Instruction(0xF, _, 0x6, 0x5) => {
                // Fx65 - LD Vx, [I]
                let x = instruction.x() as usize;
                let mem_start = self.i_register as usize;
                let mem_end = mem_start + x;

                self.v_registers[0..=x].copy_from_slice(&self.memory[mem_start..=mem_end]);
            }
            _ => {
                // Invalid Instruction
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{display::Display, instruction::Instruction};

    use super::Cpu;

    #[test]
    fn opcode_jp() {
        let mut cpu = Cpu::new();
        cpu.reset();
        cpu.execute_instruction(Instruction::from_opcode(0x1A2A));
        assert_eq!(
            cpu.program_counter, 0x0A2A,
            "the program counter is updated"
        );
    }

    #[test]
    fn opcode_call() {
        let mut cpu = Cpu::new();
        cpu.reset();
        let addr = 0x23;
        cpu.program_counter = addr;

        cpu.execute_instruction(Instruction::from_opcode(0x2ABC));

        assert_eq!(
            cpu.program_counter, 0x0ABC,
            "the program counter is updated to the new address"
        );
        assert_eq!(cpu.stack_pointer, 1, "the stack pointer is incremented");
        assert_eq!(
            cpu.stack[0],
            addr + 2,
            "the stack stores the previous address"
        );
    }

    #[test]
    fn opcode_se_vx_byte() {
        let mut cpu = Cpu::new();
        cpu.v_registers[1] = 0xFE;

        // vx == kk
        cpu.execute_instruction(Instruction::from_opcode(0x31FE));
        assert_eq!(cpu.program_counter, 4, "the stack pointer skips");

        // vx != kk
        cpu.execute_instruction(Instruction::from_opcode(0x31FA));
        assert_eq!(cpu.program_counter, 6, "the stack pointer is incremented");
    }

    #[test]
    fn opcode_sne_vx_byte() {
        let mut cpu = Cpu::new();
        cpu.v_registers[1] = 0xFE;

        // vx == kk
        cpu.execute_instruction(Instruction::from_opcode(0x41FE));
        assert_eq!(cpu.program_counter, 2, "the stack pointer is incremented");

        // vx != kk
        cpu.execute_instruction(Instruction::from_opcode(0x41FA));
        assert_eq!(cpu.program_counter, 6, "the stack pointer skips");
    }

    #[test]
    fn opcode_se_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v_registers[1] = 1;
        cpu.v_registers[2] = 3;
        cpu.v_registers[3] = 3;

        // vx == vy
        cpu.execute_instruction(Instruction::from_opcode(0x5230));
        assert_eq!(cpu.program_counter, 4, "the stack pointer skips");

        // vx != vy
        cpu.execute_instruction(Instruction::from_opcode(0x5130));
        assert_eq!(cpu.program_counter, 6, "the stack pointer is incremented");
    }

    #[test]
    fn opcode_sne_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v_registers[1] = 1;
        cpu.v_registers[2] = 3;
        cpu.v_registers[3] = 3;

        // vx == vy
        cpu.execute_instruction(Instruction::from_opcode(0x9230));
        assert_eq!(cpu.program_counter, 2, "the stack pointer is incremented");

        // vx != vy
        cpu.execute_instruction(Instruction::from_opcode(0x9130));
        assert_eq!(cpu.program_counter, 6, "the stack pointer skips");
    }

    #[test]
    fn opcode_add_vx_kkk() {
        let mut cpu = Cpu::new();
        cpu.v_registers[1] = 3;

        cpu.execute_instruction(Instruction::from_opcode(0x7101));
        assert_eq!(cpu.v_registers[1], 4, "Vx was incremented by one");
    }

    #[test]
    fn opcode_ld_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v_registers[1] = 3;
        cpu.v_registers[0] = 0;

        cpu.execute_instruction(Instruction::from_opcode(0x8010));
        assert_eq!(cpu.v_registers[0], 3, "Vx was loaded with vy");
    }

    #[test]
    fn opcode_or_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v_registers[2] = 0b01101100;
        cpu.v_registers[3] = 0b11001110;

        cpu.execute_instruction(Instruction::from_opcode(0x8231));
        assert_eq!(
            cpu.v_registers[2], 0b11101110,
            "Vx was loaded with vx OR vy"
        );
    }

    #[test]
    fn opcode_and_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v_registers[2] = 0b01101100;
        cpu.v_registers[3] = 0b11001110;

        cpu.execute_instruction(Instruction::from_opcode(0x8232));
        assert_eq!(
            cpu.v_registers[2], 0b01001100,
            "Vx was loaded with vx AND vy"
        );
    }

    #[test]
    fn opcode_xor_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v_registers[2] = 0b01101100;
        cpu.v_registers[3] = 0b11001110;

        cpu.execute_instruction(Instruction::from_opcode(0x8233));
        assert_eq!(
            cpu.v_registers[2], 0b10100010,
            "Vx was loaded with vx XOR vy"
        );
    }

    #[test]
    fn opcode_add_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v_registers[1] = 10;
        cpu.v_registers[2] = 100;
        cpu.v_registers[3] = 250;

        cpu.execute_instruction(Instruction::from_opcode(0x8124));
        assert_eq!(cpu.v_registers[1], 110, "Vx was loaded with vx + vy");
        assert_eq!(cpu.v_registers[0xF], 0, "no overflow occured");

        cpu.execute_instruction(Instruction::from_opcode(0x8134));
        assert_eq!(cpu.v_registers[1], 0x68, "Vx was loaded with vx + vy");
        assert_eq!(cpu.v_registers[0xF], 1, "overflow occured");
    }

    #[test]
    fn opcode_sub_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v_registers[1] = 100;
        cpu.v_registers[2] = 10;
        cpu.v_registers[3] = 100;

        cpu.execute_instruction(Instruction::from_opcode(0x8125));
        assert_eq!(cpu.v_registers[1], 90, "Vx was loaded with vx - vy");
        assert_eq!(cpu.v_registers[0xF], 1, "no overflow occured");

        cpu.execute_instruction(Instruction::from_opcode(0x8135));
        assert_eq!(cpu.v_registers[1], 246, "Vx was loaded with vx - vy");
        assert_eq!(cpu.v_registers[0xF], 0, "overflow occured");
    }

    #[test]
    fn opcode_shr_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v_registers[1] = 100;
        cpu.v_registers[2] = 15;

        cpu.execute_instruction(Instruction::from_opcode(0x8126));
        assert_eq!(cpu.v_registers[1], 50, "Vx was loaded with vx >> 1");
        assert_eq!(cpu.v_registers[0xF], 0, "no overflow occured");

        cpu.execute_instruction(Instruction::from_opcode(0x8236));
        assert_eq!(cpu.v_registers[2], 7, "Vx was loaded with vx >> 1");
        assert_eq!(cpu.v_registers[0xF], 1, "overflow occured");
    }

    #[test]
    fn opcode_subn_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v_registers[1] = 10;
        cpu.v_registers[2] = 100;
        cpu.v_registers[3] = 50;

        cpu.execute_instruction(Instruction::from_opcode(0x8127));
        assert_eq!(cpu.v_registers[1], 90, "Vx was loaded with vy - vx");
        assert_eq!(cpu.v_registers[0xF], 1, "no overflow occured");

        cpu.execute_instruction(Instruction::from_opcode(0x8137));
        assert_eq!(cpu.v_registers[1], 216, "Vx was loaded with vy - vx");
        assert_eq!(cpu.v_registers[0xF], 0, "overflow occured");
    }

    #[test]
    fn opcode_shl_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v_registers[1] = 15;
        cpu.v_registers[2] = 200;

        cpu.execute_instruction(Instruction::from_opcode(0x812E));
        assert_eq!(cpu.v_registers[1], 30, "Vx was loaded with vx >> 1");
        assert_eq!(cpu.v_registers[0xF], 0, "no overflow occured");

        cpu.execute_instruction(Instruction::from_opcode(0x823E));
        assert_eq!(cpu.v_registers[2], 144, "Vx was loaded with vx >> 1");
        assert_eq!(cpu.v_registers[0xF], 1, "overflow occured");
    }

    #[test]
    fn opcode_jp_v0_nnn() {
        let mut cpu = Cpu::new();
        cpu.v_registers[0] = 0x12;

        cpu.execute_instruction(Instruction::from_opcode(0xB123));
        assert_eq!(
            cpu.program_counter, 0x0135,
            "program counter jumped to location"
        );
    }

    #[test]
    fn opcode_rnd_vx_kk() {
        let mut cpu = Cpu::new();
        cpu.v_registers[1] = 0x12;

        cpu.execute_instruction(Instruction::from_opcode(0xC1F0));
        assert_eq!(cpu.v_registers[1] % 16, 0, "random number masked correctly");

        cpu.execute_instruction(Instruction::from_opcode(0xC10F));
        assert_eq!(cpu.v_registers[1] / 16, 0, "random number masked correctly");
    }

    #[test]
    fn opcode_skp_vx() {
        let mut cpu = Cpu::new();

        let initial_addr = 0x22;
        let key = 0x02;

        cpu.program_counter = initial_addr;
        cpu.v_registers[1] = key;
        cpu.keypad.key_down(key);

        cpu.execute_instruction(Instruction::from_opcode(0xE19E));
        assert_eq!(
            cpu.program_counter,
            initial_addr + 4,
            "next instruction skipped"
        );

        cpu.keypad.key_up(key);
        cpu.execute_instruction(Instruction::from_opcode(0xE19E));
        assert_eq!(
            cpu.program_counter,
            initial_addr + 6,
            "next instruction not skipped"
        );
    }

    #[test]
    fn opcode_sknp_vx() {
        let mut cpu = Cpu::new();

        let initial_addr = 0x22;
        let key = 0x02;

        cpu.program_counter = initial_addr;
        cpu.v_registers[2] = key;
        cpu.keypad.key_down(key);

        cpu.execute_instruction(Instruction::from_opcode(0xE2A1));
        assert_eq!(
            cpu.program_counter,
            initial_addr + 2,
            "next instruction not skipped"
        );

        cpu.keypad.key_up(key);
        cpu.execute_instruction(Instruction::from_opcode(0xE2A1));
        assert_eq!(
            cpu.program_counter,
            initial_addr + 6,
            "next instruction skipped"
        );
    }

    #[test]
    fn opcode_ld_vx_dt() {
        let mut cpu = Cpu::new();

        cpu.delay_timer = 0x11;

        cpu.execute_instruction(Instruction::from_opcode(0xF107));
        assert_eq!(
            cpu.v_registers[1], 0x11,
            "delay timer value loaded into register"
        );
    }

    #[test]
    fn opcode_ld_vx_k() {
        let mut cpu = Cpu::new();

        cpu.program_counter = 0x10;
        cpu.v_registers[1] = 0x99;

        cpu.execute_instruction(Instruction::from_opcode(0xF10A));
        assert_eq!(cpu.program_counter, 0x10, "pc unchanged");
        assert_eq!(cpu.v_registers[1], 0x99, "vx unchanged");

        cpu.execute_instruction(Instruction::from_opcode(0xF10A));
        assert_eq!(cpu.program_counter, 0x10, "pc unchanged");
        assert_eq!(cpu.v_registers[1], 0x99, "vx unchanged");

        cpu.execute_instruction(Instruction::from_opcode(0xF10A));
        assert_eq!(cpu.program_counter, 0x10, "pc unchanged");
        assert_eq!(cpu.v_registers[1], 0x99, "vx unchanged");

        cpu.keypad.key_down(0x08);
        cpu.execute_instruction(Instruction::from_opcode(0xF10A));
        assert_eq!(cpu.program_counter, 0x12, "pc updated");
        assert_eq!(cpu.v_registers[1], 0x08, "vx updated");
    }

    #[test]
    fn opcode_ld_dt_vx() {
        let mut cpu = Cpu::new();

        cpu.v_registers[5] = 0x11;

        cpu.execute_instruction(Instruction::from_opcode(0xF515));
        assert_eq!(
            cpu.delay_timer, 0x11,
            "register value loaded into delay timer"
        );
    }

    #[test]
    fn opcode_ld_st_vx() {
        let mut cpu = Cpu::new();

        cpu.v_registers[5] = 0x11;

        cpu.execute_instruction(Instruction::from_opcode(0xF518));
        assert_eq!(
            cpu.sound_timer, 0x11,
            "register value loaded into sound timer"
        );
    }

    #[test]
    fn opcode_add_i_vx() {
        let mut cpu = Cpu::new();

        cpu.v_registers[3] = 0x34;
        cpu.i_register = 0x1200;

        cpu.execute_instruction(Instruction::from_opcode(0xF31E));
        assert_eq!(cpu.i_register, 0x1234, "i register updated with sum");

        cpu.v_registers[5] = 0x02;
        cpu.i_register = 0xFFFF;

        cpu.execute_instruction(Instruction::from_opcode(0xF51E));
        assert_eq!(
            cpu.i_register, 0x0001,
            "i register updated with wrapping sum"
        );
    }

    #[test]
    fn opcode_ld_f_vx() {
        let mut cpu = Cpu::new();
        cpu.reset();

        cpu.v_registers[5] = 0x03;

        cpu.execute_instruction(Instruction::from_opcode(0xF529));

        assert_eq!(
            cpu.memory[cpu.i_register as usize], 0xF0,
            "first byte of digit"
        );
        assert_eq!(
            cpu.memory[cpu.i_register as usize + 1],
            0x10,
            "second byte of digit"
        );
        assert_eq!(
            cpu.memory[cpu.i_register as usize + 2],
            0xF0,
            "third byte of digit"
        );
        assert_eq!(
            cpu.memory[cpu.i_register as usize + 3],
            0x10,
            "fourth byte of digit"
        );
        assert_eq!(
            cpu.memory[cpu.i_register as usize + 4],
            0xF0,
            "fifth byte of digit"
        );
    }

    #[test]
    fn opcode_ld_i_vx() {
        let mut cpu = Cpu::new();
        cpu.v_registers[0] = 5;
        cpu.v_registers[1] = 4;
        cpu.v_registers[2] = 3;
        cpu.v_registers[3] = 2;
        cpu.i_register = 0x300;

        // load v0 - v2 into memory at i
        cpu.execute_instruction(Instruction::from_opcode(0xF255));
        assert_eq!(
            cpu.memory[cpu.i_register as usize], 5,
            "V0 was loaded into memory at i"
        );
        assert_eq!(
            cpu.memory[cpu.i_register as usize + 1],
            4,
            "V1 was loaded into memory at i + 1"
        );
        assert_eq!(
            cpu.memory[cpu.i_register as usize + 2],
            3,
            "V2 was loaded into memory at i + 2"
        );
        assert_eq!(
            cpu.memory[cpu.i_register as usize + 3],
            0,
            "i + 3 was not loaded"
        );
    }

    #[test]
    fn opcode_ld_b_vx() {
        let mut cpu = Cpu::new();
        cpu.i_register = 0x300;
        cpu.v_registers[2] = 234;

        // load v0 - v2 from memory at i
        cpu.execute_instruction(Instruction::from_opcode(0xF233));
        assert_eq!(cpu.memory[cpu.i_register as usize], 2, "hundreds");
        assert_eq!(cpu.memory[cpu.i_register as usize + 1], 3, "tens");
        assert_eq!(cpu.memory[cpu.i_register as usize + 2], 4, "digits");
    }

    #[test]
    fn opcode_ld_vx_i() {
        let mut cpu = Cpu::new();
        cpu.i_register = 0x300;
        cpu.memory[cpu.i_register as usize] = 5;
        cpu.memory[cpu.i_register as usize + 1] = 4;
        cpu.memory[cpu.i_register as usize + 2] = 3;
        cpu.memory[cpu.i_register as usize + 3] = 2;

        // load v0 - v2 from memory at i
        cpu.execute_instruction(Instruction::from_opcode(0xF265));
        assert_eq!(cpu.v_registers[0], 5, "V0 was loaded from memory at i");
        assert_eq!(cpu.v_registers[1], 4, "V1 was loaded from memory at i + 1");
        assert_eq!(cpu.v_registers[2], 3, "V2 was loaded from memory at i + 2");
        assert_eq!(cpu.v_registers[3], 0, "i + 3 was not loaded");
    }

    #[test]
    fn opcode_ret() {
        let mut cpu = Cpu::new();
        let addr = 0x23;
        cpu.program_counter = addr;

        // jump to 0x0ABC
        cpu.execute_instruction(Instruction::from_opcode(0x2ABC));
        // return
        cpu.execute_instruction(Instruction::from_opcode(0x00EE));

        assert_eq!(
            cpu.program_counter, 0x25,
            "the program counter is updated to the new address"
        );
        assert_eq!(cpu.stack_pointer, 0, "the stack pointer is decremented");
    }

    #[test]
    fn opcode_ld_i_addr() {
        let mut cpu = Cpu::new();

        cpu.execute_instruction(Instruction::from_opcode(0x61AA));
        assert_eq!(cpu.v_registers[1], 0xAA, "V1 is set");
        assert_eq!(
            cpu.program_counter, 2,
            "the program counter is advanced two bytes"
        );

        cpu.execute_instruction(Instruction::from_opcode(0x621A));
        assert_eq!(cpu.v_registers[2], 0x1A, "V2 is set");
        assert_eq!(
            cpu.program_counter, 4,
            "the program counter is advanced two bytes"
        );

        cpu.execute_instruction(Instruction::from_opcode(0x6A15));
        assert_eq!(cpu.v_registers[10], 0x15, "V10 is set");
        assert_eq!(
            cpu.program_counter, 6,
            "the program counter is advanced two bytes"
        );
    }

    #[test]
    fn opcode_axxx() {
        let mut cpu = Cpu::new();
        cpu.execute_instruction(Instruction::from_opcode(0xAFAF));

        assert_eq!(cpu.i_register, 0x0FAF, "the 'i' register is updated");
        assert_eq!(
            cpu.program_counter, 2,
            "the program counter is advanced two bytes"
        );
    }

    #[test]
    fn opcode_dxyn() {
        let mut cpu = Cpu::new();
        cpu.reset();
        cpu.v_registers[0] = 0;
        cpu.i_register = 0;
        cpu.execute_instruction(Instruction::from_opcode(0xD005));

        let display_buffer = cpu.display.get_buffer();

        assert!(display_buffer[0]);
        assert!(display_buffer[1]);
        assert!(display_buffer[2]);
        assert!(display_buffer[3]);

        assert!(display_buffer[0 + Display::WIDTH]);
        assert!(display_buffer[3 + Display::WIDTH]);

        assert!(display_buffer[0 + Display::WIDTH * 2]);
        assert!(display_buffer[3 + Display::WIDTH * 2]);

        assert!(display_buffer[0 + Display::WIDTH * 3]);
        assert!(display_buffer[3 + Display::WIDTH * 3]);

        assert!(display_buffer[0 + Display::WIDTH * 4]);
        assert!(display_buffer[1 + Display::WIDTH * 4]);
        assert!(display_buffer[2 + Display::WIDTH * 4]);
        assert!(display_buffer[3 + Display::WIDTH * 4]);
    }
}

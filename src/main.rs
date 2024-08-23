struct Chip8 {
    memory: [u8; 4096],
    pc: u16,
    screen: [u8; 64 * 32],
    stack: [u16; 16],
    sp: u8,
}

impl Chip8 {
    fn init() -> Self {
        Chip8 {
            memory: [0; 4096],
            pc: 0x200,
            screen: [0; 64 * 32],
            stack: [0; 16],
            sp: 0,
        }
    }

    fn load_rom(&mut self, rom: &[u8]) {
        for (i, &byte) in rom.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }
    }

    fn cycle(&mut self) {
        let opcode = self.fetch();
        self.decode_and_execute(opcode);
    }

    fn fetch(&self) -> u16 {
        let high_byte = self.memory[self.pc as usize] as u16;
        let low_byte = self.memory[self.pc as usize + 1] as u16;
        (high_byte << 8) | low_byte
    }

    fn decode_and_execute(&mut self, opcode: u16) {
        self.pc += 2;

        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00FF {
                0x00E0 => self.clear_screen(),
                0x00EE => self.return_from_subroutine(),
                _ => println!("Unknown opcode [0x0000]: 0x{:04X}", opcode),
            },
            0x1000 => self.jump_to_address(opcode & 0x0FFF),
            // Other opcodes go here
            _ => println!("Unknown opcode: 0x{:04X}", opcode),
        }
    }

    fn clear_screen(&mut self) {
        self.screen = [0; 64 * 32];
    }

    fn return_from_subroutine(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn jump_to_address(&mut self, address: u16) {
        self.pc = address;
    }
}

fn main() {
    let mut chip8 = Chip8::init();

    let rom = std::fs::read("example.ch8").expect("Failed to read ROM");
    chip8.load_rom(&rom);

    loop {
        chip8.cycle();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_init_chip8() {
        let chip8 = Chip8::init();

        assert_eq!(chip8.memory, [0; 4096]);
        assert_eq!(chip8.pc, 0x200);
        assert_eq!(chip8.screen, [0; 64 * 32]);
        assert_eq!(chip8.stack, [0; 16]);
        assert_eq!(chip8.sp, 0);
    }

    #[test]
    fn can_fetch_opcode() {
        let mut chip8 = Chip8::init();
        chip8.memory[0x200] = 0xAB;
        chip8.memory[0x201] = 0xCD;

        assert_eq!(chip8.fetch(), 0xABCD);
    }

    #[test]
    fn can_jump_to_address() {
        let mut chip8 = Chip8::init();
        chip8.jump_to_address(0x1234);

        assert_eq!(chip8.pc, 0x1234);
    }

    #[test]
    fn can_return_from_subroutine() {
        let mut chip8 = Chip8::init();
        chip8.stack[0] = 0x1234;
        chip8.sp = 1;

        chip8.return_from_subroutine();

        assert_eq!(chip8.pc, 0x1234);
        assert_eq!(chip8.sp, 0);
    }

    #[test]
    fn can_clear_screen() {
        let mut chip8 = Chip8::init();
        chip8.screen[0] = 1;

        chip8.clear_screen();

        assert_eq!(chip8.screen, [0; 64 * 32]);
    }

    #[test]
    fn can_load_rom() {
        let mut chip8 = Chip8::init();
        let rom = [0xAB, 0xCD, 0xEF];

        chip8.load_rom(&rom);

        assert_eq!(chip8.memory[0x200], 0xAB);
        assert_eq!(chip8.memory[0x201], 0xCD);
        assert_eq!(chip8.memory[0x202], 0xEF);
    }

    #[test]
    fn can_cycle() {
        let mut chip8 = Chip8::init();
        chip8.memory[0x200] = 0x12;
        chip8.memory[0x201] = 0x34;

        chip8.cycle();

        assert_eq!(chip8.pc, 0x234);
    }
}

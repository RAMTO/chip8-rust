use std::process;

struct Chip8 {
    memory: [u8; 4096],
    pc: u16,
    screen: [u8; 64 * 32],
    stack: [u16; 16],
    i: u16,
    sp: u8,
    registers: [u8; 16],
}

impl Chip8 {
    fn init() -> Self {
        Chip8 {
            memory: [0; 4096],
            pc: 0x200,
            screen: [0; 64 * 32],
            stack: [0; 16],
            sp: 0,
            i: 0,
            registers: [0; 16],
        }
    }

    fn load_rom(&mut self, rom: &[u8]) {
        for (i, &byte) in rom.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }
        println!("🧠 Rom loaded into memory");
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
        println!("⚙️ Executing opcode: 0x{:04X}", opcode);

        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00FF {
                0x00E0 => self.clear_screen(),
                0x00EE => self.return_from_subroutine(),
                0x00DF => self.exit_program(),
                _ => println!("Unknown opcode [0x0000]: 0x{:04X}", opcode),
            },
            0x1000 => self.jump_to_address(opcode & 0x0FFF),
            0x6000 => self.set_register((opcode & 0x0F00) >> 8, (opcode & 0x00FF) as u8),
            0x7000 => self.add_to_register((opcode & 0x0F00) >> 8, (opcode & 0x00FF) as u8),
            0x8000 => match opcode & 0x000F {
                0x0000 => {
                    self.set_register_to_register((opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4)
                }
                0x0004 => self.add_registers((opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4),
                _ => println!("Unknown opcode [0x8000]: 0x{:04X}", opcode),
            },
            0xA000 => self.set_index_register(opcode & 0x0FFF),
            _ => println!("Unknown opcode: 0x{:04X}", opcode),
        }
    }

    fn clear_screen(&mut self) {
        println!("➡️ Clearing the screen");
        self.screen = [0; 64 * 32];
    }

    fn return_from_subroutine(&mut self) {
        println!("➡️ Returning from subroutine");
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn jump_to_address(&mut self, address: u16) {
        println!("➡️ Jumping to address: 0x{:04X}", address);
        self.pc = address;
    }

    fn set_index_register(&mut self, value: u16) {
        println!("➡️ Setting I to 0x{:04X}", value);
        self.i = value;
    }

    fn exit_program(&mut self) {
        println!("➡️ Exiting the program");
        process::exit(0);
    }

    // New methods for register operations
    fn set_register(&mut self, register: u16, value: u8) {
        println!("➡️ Setting register V{:X} to 0x{:02X}", register, value);
        self.registers[register as usize] = value;
    }

    fn add_to_register(&mut self, register: u16, value: u8) {
        println!("➡️ Adding 0x{:02X} to register V{:X}", value, register);
        self.registers[register as usize] = self.registers[register as usize].wrapping_add(value);
    }

    fn set_register_to_register(&mut self, target: u16, source: u16) {
        println!(
            "➡️ Setting register V{:X} to the value of V{:X}",
            target, source
        );
        self.registers[target as usize] = self.registers[source as usize];
    }

    fn add_registers(&mut self, target: u16, source: u16) {
        println!("➡️ Adding register V{:X} to V{:X}", source, target);
        let (result, overflow) =
            self.registers[target as usize].overflowing_add(self.registers[source as usize]);
        self.registers[target as usize] = result;
        self.registers[0xF] = if overflow { 1 } else { 0 };
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

    #[test]
    fn can_set_index_register() {
        let mut chip8 = Chip8::init();
        chip8.set_index_register(0x1234);

        assert_eq!(chip8.i, 0x1234);
    }

    #[test]
    fn can_set_register() {
        let mut chip8 = Chip8::init();
        chip8.set_register(0x5, 0xAB);
        assert_eq!(chip8.registers[0x5], 0xAB);
    }

    #[test]
    fn can_add_to_register() {
        let mut chip8 = Chip8::init();
        chip8.registers[0x3] = 0x10;
        chip8.add_to_register(0x3, 0x05);
        assert_eq!(chip8.registers[0x3], 0x15);
    }

    #[test]
    fn can_set_register_to_register() {
        let mut chip8 = Chip8::init();
        chip8.registers[0x2] = 0xCD;
        chip8.set_register_to_register(0x4, 0x2);
        assert_eq!(chip8.registers[0x4], 0xCD);
    }

    #[test]
    fn can_add_registers() {
        let mut chip8 = Chip8::init();
        chip8.registers[0x1] = 0x50;
        chip8.registers[0x2] = 0x30;
        chip8.add_registers(0x1, 0x2);
        assert_eq!(chip8.registers[0x1], 0x80);
        assert_eq!(chip8.registers[0xF], 0);

        chip8.registers[0x1] = 0xFF;
        chip8.registers[0x2] = 0x01;
        chip8.add_registers(0x1, 0x2);
        assert_eq!(chip8.registers[0x1], 0x00);
        assert_eq!(chip8.registers[0xF], 1);
    }
}

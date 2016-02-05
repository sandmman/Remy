use super::memory;

#[derive(Default)]
pub struct Cpu {
    reg_pc: u16, // 16 bit program counter
    reg_sp: u8,  // 8 bit stack pointer
    //reg_p:  u8,  // 6 bits used by alu
    reg_fr: u8,  // 8 bit flag register indicates the status of the last instruction executed
                 // NEGATIVE_FLAG bit position 7
                 // OVERFLOW bit position 6
                 // BREAK bit position 5
                 // DECIMAL bit position 4
                 // INTERRUPT bit position 3
                 // ZERO_FLAG bit position 2
                 // CARRY_FLAG bit position 1
                 // NEGATIVE_FLAG bit position 0
    // Work Registers
    reg_a: u8,   // Accumulator
    reg_x: u8,   // Indexed addressing
    reg_y: u8,    // Limited indexed addressing
    memory: memory::Memory
}
impl Cpu {
    pub fn power_on_reset(&mut self) {
        self.reg_pc = 16;
        self.reg_fr = 0x34;
        self.reg_sp = 0xFD;
    }

    fn read_word(&mut self) -> u8 {
        self.memory.read_word(self.reg_pc)
    }
    fn read_opcode(&mut self) -> u16 {
        let mut up: u16 = (self.memory.read_word(self.reg_pc) as u16);
        up << 8;
        up += (self.memory.read_word(self.reg_pc + 1) as u16);
        up
    }
    fn write_word(&mut self, addr: u16, obj: u8) {
        self.memory.write_word(addr, obj);
    }

    pub fn run(&mut self,rom: &Vec<u8>) {
        loop {
            let opcode: u16 = ((rom[self.reg_pc as usize] as u16) << 8) + (rom[(self.reg_pc as usize) + 1] as u16);
            self.reg_pc += 1;
            match opcode {
                0x0A => {
                    println!("ASL!");
                    self.reg_a = self.reg_a << 1; // ASL Shift Left One Bit (Memory or Accumulator)
                },
                0xA9 => {},// LDA (Load Accumulator With Memory) Immediate: Bytes 2
                0xA5 => {},// LDA (Load Accumulator With Memory) Zero Page: Bytes 2
                0xB5 => {},// LDA (Load Accumulator With Memory) Zero Page X: Bytes 2
                0xAD => {},// LDA (Load Accumulator With Memory) Absolute:   Bytes 3
                0xBD => {},// LDA (Load Accumulator With Memory) Absolute X: Bytes 3
                0xB9 => {},// LDA (Load Accumulator With Memory) Absolute Y: Bytes 3
                0xA1 => {},// LDA (Load Accumulator With Memory) Indirect X: Bytes 2
                0xB1 => {},// LDA (Load Accumulator With Memory) Indirect Y: Bytes 2
                _ => {
                    panic!("instruction not found!");
                }
            }
            panic!("instruction Complete!");


            }
        }
    }
enum FlagRegister {
    CARRY_FLAG,
    ZERO_FLAG,
    INTERRUPT_DISABLE,
    DECIMAL_Mode,
    BREAK_COMMAND,
    OVERFLOW_FLAG,
    NEGATIVE_FLAG
}

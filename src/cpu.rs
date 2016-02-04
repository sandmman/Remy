#[derive(Default,Debug)]
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
    reg_y: u8    // Limited indexed addressing
}
impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu::default();
        cpu.reg_pc = 16;
        cpu.reg_fr = 0x34;
        cpu.reg_sp = 0xFD;
        cpu
    }
    pub fn reset() {

    }
    pub fn read_instruction(&self) {
        //let opcode = rom[self.reg_pc].to_string() + &rom[self.reg_pc+1].to_string();
        println!("READ!");
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

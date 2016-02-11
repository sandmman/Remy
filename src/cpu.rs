use super::memory;
use super::opcodes::Opcode::*;
use super::instructions::Instruction;

const SIGN: usize       = 7;
const OVERFLOW: usize   = 6;
const BREAK: usize      = 4;
const DECIMAL: usize    = 3;
const INTERRUPT: usize  = 2;
const ZERO: usize       = 1;
const CARRY: usize      = 0;

pub struct Cpu {
    reg_pc: u16,        // 16 bit program counter The low and high 8-bit halves of the register are called PCL and PCH
    reg_sp: u8,         // 8 bit stack pointer // located at $0100-$01FF
    reg_flag: Vec<u8>,    // 8 bit flag register
    // Work Registers
    reg_a: u8,          // Accumulator
    reg_x: u8,          // Indexed addressing
    reg_y: u8,          // Limited indexed addressing
    memory: memory::Memory
}
impl Cpu {
    pub fn new(mem: memory::Memory) -> Cpu {
        Cpu {
            reg_pc: 16,
            reg_sp: 0xFD,
            reg_flag: vec![0,0,1,1,0,1,0,0],
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            memory: mem,
        }
    }
    pub fn power_on_reset(&mut self) {
        self.reg_pc = 16;
        self.reg_sp = 0xFD;
        self.reg_flag = vec![0,0,1,1,0,1,0,0];
    }
    fn read_u8(&self, pc: u16) -> u8 {
        self.memory.read_u8(pc)
    }
    fn read_u16(&self, pc: u16) -> u16 {
        self.memory.read_u16(pc)
    }
    /*fn read_rom_u8(&self, pc: u16) -> u8 {
        self.memory.read_rom_u8(pc)
    }
    fn read_rom_u16(&self, pc: u16) -> u16 {
        self.memory.read_rom_u16(pc)
    }*/
    fn write_u8(&mut self, addr: u16, obj: u8) {
        self.memory.write_u8(addr, obj);
    }
    fn write_u16(&mut self, addr: u16, obj: u16){
        self.memory.write_u16(addr,obj);
    }
    fn run_instruction(&mut self){
        let instru = self.read_instruction();
        self.incr_pc(1);
        // Adjust information
        println!("Instruction: {:#x}",instru.opcode);
        println!("Program Counter: {:#x}",self.reg_pc);
        self.execute_instruction(instru);
    }
    fn read_instruction(&mut self) -> Instruction {
        Instruction{opcode: self.read_u8(self.reg_pc)}
    }
    fn execute_instruction(&mut self, instr: Instruction){
        match instr.opcode() {
              // The only thing changing is where the memory is coming from, so
              // simple get the correct chunk and run the helper program
              AdcImmediate      => {
                                    let m = self.immediate();
                                    self.adc(m);
                                },
              AdcZeroPage       => {
                                    let m = self.zero_page();
                                    self.adc(m);
                                },
              AdcZeroPageX      => {
                                    let m = self.zero_page_x();
                                    self.adc(m);
                                },
              AdcAbsolute       => {
                                    let m = self.absolute();
                                    self.adc(m);
                                },
              AdcAbsoluteX      => {
                                    let m = self.absolute_x();
                                    self.adc(m);
                                },
              AdcAbsoluteY      => {
                                    let m = self.absolute_y();
                                    self.adc(m);
                                },
              AdcIndirectX      => {
                                    let m = self.indirect_x();
                                    self.adc(m);
                                },
              AdcIndirectY      => {
                                    let m = self.indirect_y();
                                    self.adc(m);
                                },
              AndImmediate      => {
                                    let m = self.immediate();
                                    self.and(m);
                                },
              AndZeroPage       => {
                                    let m = self.zero_page();
                                    self.and(m);
                                },
              AndZeroPageX      => {
                                    let m = self.zero_page_x();
                                    self.and(m);
                                },
              AndAbsolute       => {
                                    let m = self.absolute();
                                    self.and(m);
                                },
              AndAbsoluteX      => {
                                    let m = self.absolute_x();
                                    self.and(m);
                                },
              AndAbsoluteY      => {
                                    let m = self.absolute_y();
                                    self.and(m);
                                },
              AndIndirectX      => {
                                    let m = self.indirect_x();
                                    self.and(m);
                                },
              AndIndirectY      => {
                                    let m = self.indirect_y();
                                    self.and(m);
                                },
              AslAccumulator    => {
                                    let b = self.reg_a;
                                    self.asl(b);
                                },
              AslZeroPage       => {
                                    let b = self.zero_page();
                                    self.asl(b);
                                },
              AslZeroPageX      => {
                                    let b = self.zero_page_x();
                                    self.asl(b);
                                },
              AslAbsolute       => {
                                    let b = self.absolute();
                                    self.asl(b);
                                },
              AslAbsoluteX      => {
                                    let b = self.absolute_x();
                                    self.asl(b);
                                },
              Bcc               => {
                                    let m = self.read_u8(self.reg_pc);
                                    if self.reg_flag[CARRY] == 0 {
                                        self.incr_pc(m);
                                    } else {
                                        self.reg_pc += 1;
                                    }
                                },
              Bcs               => {
                                    let m = self.read_u8(self.reg_pc);
                                    if self.reg_flag[CARRY] == 1 {
                                      self.incr_pc(m);
                                    } else {
                                      self.incr_pc(1);
                                  }
                                },
              Beq               => {
                                    let m = self.read_u8(self.reg_pc);
                                    if self.reg_flag[ZERO] == 1 {
                                      self.incr_pc(m);
                                    } else {
                                      self.incr_pc(1);
                                  }
                                },
              BitZeroPage       => {
                                    let m = self.zero_page();
                                    let t = self.reg_a & m;
                                    self.reg_flag[SIGN]     = if (t & 0x80) == 0x80 { 1 } else { 0 };
                                    self.reg_flag[OVERFLOW] = if (t & 0x40) == 0x40 { 1 } else { 0 };
                                    self.reg_flag[ZERO]     = if  t == 0x00         { 1 } else { 0 };
                                },
              BitAbsolute       => {
                                    let m = self.absolute();
                                    let t = self.reg_a & m;
                                    self.reg_flag[SIGN]     = if (t & 0x80) == 0x80 { 1 } else { 0 };
                                    self.reg_flag[OVERFLOW] = if (t & 0x40) == 0x40 { 1 } else { 0 };
                                    self.reg_flag[ZERO]     = if  t == 0x00         { 1 } else { 0 };
                                },
              Bmi               => {
                                    let m = self.read_u8(self.reg_pc);
                                    if self.reg_flag[SIGN] == 1 {
                                        self.incr_pc(m);
                                    } else {
                                        self.incr_pc(1);
                                    }
                                },
              Bne               => {
                                    let m = self.read_u8(self.reg_pc);
                                    if self.reg_flag[ZERO] == 0 {
                                        self.incr_pc(m);
                                    } else {
                                        self.incr_pc(1);
                                    }
                                },
              Bpl               => {
                                    let m = self.read_u8(self.reg_pc);
                                    if self.reg_flag[SIGN] == 0 {
                                        self.incr_pc(m);
                                    } else {
                                        self.incr_pc(1);
                                    }
                                },
              Brk               => {
                                    self.incr_pc(1);                  //Byte after the break command will not be evaluated
                                    let pc = self.reg_pc;
                                    let hi = (self.reg_pc >> 8) as u8;
                                    let lo = self.reg_pc as u8;
                                    let p = ((self.reg_flag[SIGN] as u8) << 7) + ((self.reg_flag[OVERFLOW] as u8) << 6) + ((self.reg_flag[BREAK] as u8) << 4) + ((self.reg_flag[DECIMAL] as u8) << 3) + ((self.reg_flag[INTERRUPT] as u8) << 2) + ((self.reg_flag[ZERO] as u8) << 1) +(self.reg_flag[CARRY] as u8);

                                    self.write_u8(pc,hi);           //Push(write) the PC high to the top of stack
                                    self.reg_sp -= 1;
                                    self.write_u8(pc,lo);           //Push(write) the PC low to the top of stack
                                    self.reg_sp -= 1;
                                    self.write_u8(pc,p|0x10);       //Push(write) something else on stack
                                    self.reg_sp -= 1;

                                    let l = self.read_u8(0xFFFE) as u16;
                                    let h = (self.read_u8(0xFFFF) as u16) << 8;
                                    self.reg_pc = h | l
                                },
              Bvc               => {
                                    let m = self.read_u8(self.reg_pc);
                                    if self.reg_flag[OVERFLOW] == 0 {
                                        self.incr_pc(m);
                                    } else {
                                        self.incr_pc(1);
                                    }
                                },
              Bvs               => {
                                    let m = self.read_u8(self.reg_pc);
                                    if self.reg_flag[OVERFLOW] == 1 {
                                        self.incr_pc(m);
                                    } else {
                                        self.incr_pc(1);
                                    }
                                },
              Clc               => {self.clear_carry();},       // Clear Carry Flags
              Cld               => {self.clear_decimal();},     // Clear Decimal Mode
              Cli               => {self.clear_interrupt();},   // Clear Interrupt Disable Bit
              Clv               => {self.clear_overflow();},    // Clear Overflow Flag
              Sec               => {self.set_carry();},         // Set Carry Flag
              Sed               => {self.set_decimal();},       // Set Decimal Mode
              Sei               => {self.set_interrupt();},     // Set Interrupt Disable Bit
              CmpImmediate      => {
                                    let m = self.immediate();
                                    let a = self.reg_a;
                                    self.cmp(m,a);
                                },
              CmpZeroPage       => {
                                    let m = self.zero_page();
                                    let a = self.reg_a;
                                    self.cmp(m,a);
                                },
              CmpZeroPageX      => {
                                    let m = self.zero_page_x();
                                    let a = self.reg_a;
                                    self.cmp(m,a);
                                },
              CmpAbsolute       => {
                                    let m = self.absolute();
                                    let a = self.reg_a;
                                    self.cmp(m,a);
                                },
              CmpAbsoluteX      => {
                                    let m = self.absolute_x();
                                    let a = self.reg_a;
                                    self.cmp(m,a);
                                },
              CmpAbsoluteY      => {
                                    let m = self.absolute_y();
                                    let a = self.reg_a;
                                    self.cmp(m,a);
                                },
              CmpIndirectX      => {
                                    let m = self.indirect_x();
                                    let a = self.reg_a;
                                    self.cmp(m,a);
                                },
              CmpIndirectY      => {
                                    let m = self.indirect_y();
                                    let a = self.reg_a;
                                    self.cmp(m,a);
                                },
              CpxImmediate      => {
                                    let m = self.immediate();
                                    let x = self.reg_x;
                                    self.cmp(m,x);
                                },
              CpxZeroPage       => {
                                    let m = self.zero_page();
                                    let x = self.reg_x;
                                    self.cmp(m,x);
                                },
              CpxAbsolute       => {
                                    let m = self.absolute();
                                    let x = self.reg_x;
                                    self.cmp(m,x);
                                },
              CpyImmediate      => {
                                    let m = self.immediate();
                                    let y = self.reg_y;
                                    self.cmp(m,y);
                                },
              CpyZeroPage       => {
                                    let m = self.zero_page();
                                    let y = self.reg_y;
                                    self.cmp(m,y);
                                },
              CpyAbsolute       => {
                                    let m = self.absolute();
                                    let y = self.reg_y;
                                    self.cmp(m,y);
                                },

              DecZeroPage       => {
                                    let m = (self.zero_page() - 1) & 0xFF;
                                    self.sign_zero_flags(m)
                                },
              DecZeroPageX      => {
                                    let m = (self.zero_page_x() - 1) & 0xFF;
                                    self.sign_zero_flags(m)
                                },
              DecAbsolute       => {
                                    let m = (self.absolute() - 1) & 0xFF;
                                    self.sign_zero_flags(m)
                                },
              DecAbsoluteX      => {
                                    let m = (self.absolute_x() - 1) & 0xFF;
                                    self.sign_zero_flags(m)
                                },
              Dex               => {
                                    self.reg_x = self.reg_x - 1;
                                    let x = self.reg_x;
                                    self.sign_zero_flags(x);
                                },
              Dey               => {
                                    self.reg_y = self.reg_y - 1;
                                    let y = self.reg_y;
                                    self.sign_zero_flags(y)
                                },
              Inx               => {
                                    self.reg_x = self.reg_x + 1;
                                    let x = self.reg_x;
                                    self.sign_zero_flags(x)
                                },
              Iny               => {
                                    self.reg_y = self.reg_y + 1;
                                    let y = self.reg_y;
                                    self.sign_zero_flags(y)
                                },
              EorImmediate      => {
                                    let m = self.immediate();
                                    self.eor(m);
                                },
              EorZeroPage       => {
                                    let m = self.zero_page();
                                    self.eor(m);
                                },
              EorZeroPageX      => {
                                    let m = self.zero_page_x();
                                    self.eor(m);
                                },
              EorAbsolute       => {
                                    let m = self.absolute();
                                    self.eor(m);
                                },
              EorAbsoluteX      => {
                                    let m = self.absolute_x();
                                    self.eor(m);
                                },
              EorAbsoluteY      => {
                                    let m = self.absolute_y();
                                    self.eor(m);
                                },
              EorIndirectX      => {
                                    let m = self.indirect_x();
                                    self.eor(m);
                                },
              EorIndirectY      => {
                                    let m = self.indirect_y();
                                    self.eor(m);
                                },

              IncZeroPage       => {
                                    let m = (self.zero_page() + 1) & 0xFF;
                                    self.sign_zero_flags(m);
                                },
              IncZeroPageX      => {
                                    let m = (self.zero_page_x() + 1) & 0xFF;
                                    self.sign_zero_flags(m);
                                },
              IncAbsolute       => {
                                    let m = (self.absolute() + 1) & 0xFF;
                                    self.sign_zero_flags(m);
                                },
              IncAbsoluteX      => {
                                    let m = (self.absolute_x() + 1) & 0xFF;
                                    self.sign_zero_flags(m);
                                },
              JmpAbsolute       => {
                                    self.reg_pc = self.read_u16(self.reg_pc);
                                },
              JmpIndirect       => {
                                    self.reg_pc = self.read_u16(self.reg_pc);
                                },
              JsrAbsolute       => {// push (PC+2),(PC+1) -> PCL,(PC+2) -> PCH
                                    // TODO: Reconsider stack setup. Specialized functions or actual stack might be cool
                                    let pc = self.reg_pc + 2;
                                    let sp = self.reg_sp as u16;
                                    self.write_u16(sp, pc); // Push PC + 2 onto stack
                                    let jsr_addr = self.read_u16(self.reg_pc+1);
                                    self.reg_pc = jsr_addr;
                                },
              LdaImmediate      => {
                                    self.reg_a = self.immediate();
                                    let l = self.reg_a;
                                    self.sign_zero_flags(l);
                                },
              LdaZeroPage       => {
                                    self.reg_a = self.zero_page();
                                    let l = self.reg_a;
                                    self.sign_zero_flags(l);
                                },
              LdaZeroPageX      => {
                                    self.reg_a = self.zero_page_x();
                                    let l = self.reg_a;
                                    self.sign_zero_flags(l);
                                },
              LdaAbsolute       => {
                                    self.reg_a = self.absolute();
                                    let l = self.reg_a;
                                    self.sign_zero_flags(l);
                                },
              LdaAbsoluteX      => {
                                    self.reg_a = self.absolute_x();
                                    let l = self.reg_a;
                                    self.sign_zero_flags(l);
                                },
              LdaAbsoluteY      => {
                                    self.reg_a = self.absolute_y();
                                    let l = self.reg_a;
                                    self.sign_zero_flags(l);
                                },
              LdaIndirectX      => {
                                    self.reg_a = self.indirect_x();
                                    let l = self.reg_a;
                                    self.sign_zero_flags(l);
                                },
              LdaIndirectY      => {
                                    self.reg_a = self.indirect_y();
                                    let l = self.reg_a;
                                    self.sign_zero_flags(l);
                                },

              LdxImmediate      => {
                                    self.reg_x = self.immediate();
                                    let l = self.reg_x;
                                    self.sign_zero_flags(l);
                                },
              LdxZeroPage       => {
                                    self.reg_x = self.zero_page();
                                    let l = self.reg_x;
                                    self.sign_zero_flags(l);
                                },
              LdxZeroPageX      => {
                                    self.reg_x = self.zero_page_x();
                                    let l = self.reg_x;
                                    self.sign_zero_flags(l);
                                },
              LdxAbsolute       => {
                                    self.reg_x = self.absolute();
                                    let l = self.reg_x;
                                    self.sign_zero_flags(l);
                                },
              LdxAbsoluteX      => {
                                    self.reg_x = self.absolute_x();
                                    let l = self.reg_x;
                                    self.sign_zero_flags(l);
                                },
              LdyImmediate      => {
                                    self.reg_y = self.immediate();
                                    let l = self.reg_y;
                                    self.sign_zero_flags(l);
                                },
              LdyZeroPage       => {
                                    self.reg_y = self.zero_page();
                                    let l = self.reg_y;
                                    self.sign_zero_flags(l);
                                },
              LdyZeroPageX      => {
                                    self.reg_y = self.zero_page_x();
                                    let l = self.reg_y;
                                    self.sign_zero_flags(l);
                                },
              LdyAbsolute       => {
                                    self.reg_y = self.absolute();
                                    let l = self.reg_y;
                                    self.sign_zero_flags(l);
                                },
              LdyAbsoluteX      => {
                                    self.reg_y = self.absolute_x();
                                    let l = self.reg_y;
                                    self.sign_zero_flags(l);
                                },
              LsrAccumulator    => {
                                    let a = self.reg_a;
                                    self.lsr(a);
                                },
              LsrZeroPage       => {
                                    let b = self.zero_page();
                                    self.lsr(b);
                                },
              LsrZeroPageX      => {
                                    let b = self.zero_page_x();
                                    self.lsr(b);
                                },
              LSRAbsolute       => {
                                    let b = self.absolute();
                                    self.lsr(b);
                                },
              LSRAbsoluteX      => {
                                    let b = self.absolute_x();
                                    self.lsr(b);
                                },
              NopImplied        => { /* No operation 2 cycles */},
              OraImmediate      => {
                                    self.reg_a = self.reg_a | self.immediate();
                                    let a = self.reg_a;
                                    self.sign_zero_flags(a);
                                },
              OraZeroPage       => {
                                    self.reg_a = self.reg_a | self.zero_page();
                                    let a = self.reg_a;
                                    self.sign_zero_flags(a);
                                },
              OraZeroPageX      => {
                                    self.reg_a = self.reg_a | self.zero_page_x();
                                    let a = self.reg_a;
                                    self.sign_zero_flags(a);
                                },
              OraAbsolute       => {
                                    self.reg_a = self.reg_a | self.absolute();
                                    let a = self.reg_a;
                                    self.sign_zero_flags(a);
                                },
              OraAbsoluteX      => {
                                    self.reg_a = self.reg_a | self.absolute_x();
                                    let a = self.reg_a;
                                    self.sign_zero_flags(a);
                                },
              OraAbsoluteY      => {
                                    self.reg_a = self.reg_a | self.absolute_y();
                                    let a = self.reg_a;
                                    self.sign_zero_flags(a);
                                },
              OraIndirectX      => {
                                    self.reg_a = self.reg_a | self.indirect_x();
                                    let a = self.reg_a;
                                    self.sign_zero_flags(a);
                                },
              OraIndirectY      => {
                                    self.reg_a = self.reg_a | self.indirect_y();
                                    let a = self.reg_a;
                                    self.sign_zero_flags(a);
                                },
              Pha               => { //Push A onto Stack
                                    let a = self.reg_a;
                                    let addr = 0x0100 + (self.reg_sp as u16);
                                    self.write_u8(addr,a);
                                    self.reg_sp -= 1;
                                },
              Php               => { //Push P onto Stack
                                    let p = ((self.reg_flag[SIGN] as u8) << 7) + ((self.reg_flag[OVERFLOW] as u8) << 6) + ((self.reg_flag[BREAK] as u8) << 4) + ((self.reg_flag[DECIMAL] as u8) << 3) + ((self.reg_flag[INTERRUPT] as u8) << 2) + ((self.reg_flag[ZERO] as u8) << 1) +(self.reg_flag[CARRY] as u8);
                                    let addr = 0x0100 + (self.reg_sp as u16);
                                    self.write_u8(addr,p);
                                    self.reg_sp -= 1;
                                },
              Pla               => { //Pull from Stack to A
                                    self.reg_sp += 1;
                                    let addr = 0x0100 + (self.reg_sp as u16);
                                    let a = self.read_u8(addr);
                                    self.reg_a = a;
                                    self.sign_zero_flags(a);
                                },
              Plp               => { //Pull from Stack to P
                                    self.reg_sp += 1;
                                    let addr = 0x0100 + (self.reg_sp as u16);
                                    let p = self.read_u8(addr);
                                    self.convert_to_sign_zero_flags(p);
                                },
              RolAccumulator    => {
                                    let a = self.reg_a;
                                    self.rol(a);
                                },
              RolZeroPage       => {
                                    let b = self.zero_page();
                                    self.rol(b);
                                },
              RolZeroPageX      => {
                                    let b = self.zero_page_x();
                                    self.rol(b);
                                },
              RolAbsolute       => {
                                    let b = self.absolute();
                                    self.rol(b);
                                },
              RolAbsoluteX      => {
                                    let b = self.absolute_x();
                                    self.rol(b);
                                },
              Roraccumulator    => {
                                    let a = self.reg_a;
                                    self.ror(a);
                                },
              RorZeroPage       => {
                                    let b = self.zero_page();
                                    self.ror(b);
                                },
              RorZeroPageX      => {
                                    let b = self.zero_page_x();
                                    self.ror(b);
                                },
              RorAbsolute       => {
                                    let b = self.absolute();
                                    self.ror(b);
                                },
              RorAbsoluteX      => {
                                    let b = self.absolute_x();
                                    self.ror(b);
                                },
              Rti               => { // Return from interrupt
                                    self.reg_sp -= 1;
                                    let addr_p = (self.reg_sp as u16) + 0x0100;
                                    let p = self.read_u8(addr_p);
                                    self.convert_to_sign_zero_flags(p);
                                    self.reg_sp -= 1;
                                    let addr_l = (self.reg_sp as u16) + 0x0100;
                                    let l = self.read_u8(addr_l) as u16;
                                    self.reg_sp -= 1;
                                    let addr_h = (self.reg_sp as u16) + 0x0100;
                                    let h = (self.read_u8(addr_h) as u16) << 8;
                                    self.reg_pc = h | l;
                                },
              Rts               => { // Return from subroutine: Operation:  PC from S, PC + 1 -> PC
                                    self.reg_sp += 1;
                                    let addr_l = (self.reg_sp as u16) + 0x0100;
                                    let l = self.read_u8(addr_l) as u16;
                                    self.reg_sp += 1;
                                    let addr_h = (self.reg_sp as u16) + 0x0100;
                                    let h = (self.read_u8(addr_h) as u16) << 8;
                                    self.reg_pc = ( h | l ) + l;
                                },
              SbcImmediate      => {
                                    let m = self.immediate();
                                    self.sbc(m);
                                },
              SbcZeroPage       => {
                                    let m = self.zero_page();
                                    self.sbc(m);
                                },
              SbcZeroPageX      => {
                                    let m = self.zero_page_x();
                                    self.sbc(m);
                                },
              SbcAbsolute       => {
                                    let m = self.absolute();
                                    self.sbc(m);
                                },
              SbcAbsoluteX      => {
                                    let m = self.absolute_x();
                                    self.sbc(m);
                                },
              SbcAbsoluteY      => {
                                    let m = self.absolute_y();
                                    self.sbc(m);
                                },
              SbcIndirectX      => {
                                    let m = self.indirect_x();
                                    self.sbc(m);
                                },
              SbcIndirectY      => {
                                    let m = self.indirect_y();
                                    self.sbc(m);
                                },
              StaZeroPage       => {
                                    let a = self.reg_a;
                                    let addr = self.zero_page() as u16;
                                    self.st_axy(addr,a);
                                },
              StaZeroPageX      => {
                                    let a = self.reg_a;
                                    let addr = self.zero_page_x() as u16;
                                    self.st_axy(addr,a);
                                },
              StaAbsolute       => {
                                    let a = self.reg_a;
                                    let addr = self.absolute() as u16;
                                    self.st_axy(addr,a);
                                },
              StaAbsoluteX      => {
                                    let a = self.reg_a;
                                    let addr = self.absolute_x() as u16;
                                    self.st_axy(addr,a);
                                },
              StaAbsoluteY      => {
                                    let a = self.reg_a;
                                    let addr = self.absolute_y() as u16;
                                    self.st_axy(addr,a);
                                },
              StaIndirectX      => {
                                    let a = self.reg_a;
                                    let addr = self.indirect_x() as u16;
                                    self.st_axy(addr,a);
                                },
              StaIndirectY      => {
                                    let a = self.reg_a;
                                    let addr = self.indirect_y() as u16;
                                    self.st_axy(addr,a);
                                },
              StxZeroPage       => {
                                    let x = self.reg_x;
                                    let addr = self.zero_page() as u16;
                                    self.st_axy(addr,x);
                                },
              StxZeroPageX      => {
                                    let x = self.reg_x;
                                    let addr = self.zero_page_x() as u16;
                                    self.st_axy(addr,x);
                                },
              StxAbsolute       => {
                                    let x = self.reg_x;
                                    let addr = self.absolute() as u16;
                                    self.st_axy(addr,x);
                                },
              StyZeroPage       => {
                                    let y = self.reg_y;
                                    let addr = self.zero_page() as u16;
                                    self.st_axy(addr,y);
                                },
              StyZeroPageX      => {
                                    let y = self.reg_y;
                                    let addr = self.zero_page_x() as u16;
                                    self.st_axy(addr,y);
                                },
              StyAbsolute       => {
                                    let y = self.reg_y;
                                    let addr = self.absolute() as u16;
                                    self.st_axy(addr,y);
                                },
              Tax               => {self.reg_x = self.reg_a;},  // Transfer Accumulator to index x
              Tay               => {self.reg_y = self.reg_a;},  // Transfer Accumulator to index y
              Tsx               => {self.reg_x = self.reg_sp;}, // Transfer Stack Pointer to index x
              Txa               => {self.reg_a = self.reg_x;},  // Transfer index X to accumulator
              Txs               => {self.reg_sp = self.reg_x;}, // Transfer index X to stack pointer
              Tya               => {self.reg_a = self.reg_y;},  // Transfer index Y to accumulator

        }
    }
    fn immediate(&mut self) -> u8 {
        let m = self.read_u8(self.reg_pc);
        self.incr_pc(1);
        m
    }
    fn zero_page(&mut self) -> u8 {
        let _addr = self.read_u8(self.reg_pc); //Fetch the operand from the next byte
        self.incr_pc(1);                       //Increment the PC
        self.read_u8(_addr as u16)             //Fetch the value of the address/operand
    }
    fn zero_page_x(&mut self) -> u8 {
        let mut _addr = self.read_u8(self.reg_pc);  //Fetch the operand from the next byte
        self.incr_pc(1);                            //Increment the PC
        _addr = (_addr + self.reg_x) & 0xFF;        //Add the X register to the address and wrap around if address +X > 0xF
        self.read_u8(_addr as u16)
    }
    fn absolute(&mut self) -> u8 {
        let _addr = self.read_u16(self.reg_pc); // Fetch the next addr pointing anywhere in memory
        self.incr_pc(2);
        self.read_u8(_addr)
    }
    fn absolute_x(&mut self) -> u8 {
        let mut addr = self.read_u16(self.reg_pc); // Fetch the next addr pointing anywhere in memory
        let _addr = addr & 0x100;
        self.incr_pc(2);
        addr += self.reg_x as u16;
        // Check if there is a page crossing
        /*if (addr & 0x100) ^ _addr {
            self.read_u8(addr);
        }*/
        self.read_u8(addr)
    }
    fn absolute_y(&mut self) -> u8 {
        let mut addr = self.read_u16(self.reg_pc); // Fetch the next addr pointing anywhere in memory
        let _addr = addr & 0x100;
        self.incr_pc(2);
        addr += self.reg_y as u16;
        // Check if there is a page crossing
        /*if (addr & 0x100) ^ _addr {
            self.read_u8(addr);
        }*/
        self.read_u8(addr)
    }
    fn indirect_x(&mut self) -> u8 {
        let addr = self.read_u8(self.reg_pc);  // Fetch the pointer address from the next byte in the PC
        let tmp = (addr + self.reg_x) & 0xFF;
        let _addr = self.read_u16(tmp as u16);
        self.incr_pc(1);
        self.read_u8(_addr)
    }
    fn indirect_y(&mut self) -> u8 {
        let mut _addr = self.read_u8(self.reg_pc); //u8
        let mut _addr = _addr as u16;
        let mut addr = self.read_u16(_addr); //u16
        self.incr_pc(1);
        _addr = addr & 0x100;
        addr += self.reg_y as u16;
        if (addr & 0x100) != _addr {
            self.read_u8(addr);
        }
        self.read_u8(addr)
    }
    // Basic functions
    fn adc(&mut self, m: u8){ // Add Memory to Accumulator with Carry: A + M + C -> A, C
        let ret = self.reg_a + m + self.reg_flag[CARRY];

        self.reg_flag[OVERFLOW] = (ret ^ self.reg_a) & (ret ^ m) & 0x80;
        self.reg_flag[CARRY] = ret & 0x100; //>> 8;
        self.reg_flag[ZERO]  = ret & 0xFF;
        self.reg_flag[SIGN]  = ret & 0xFF;
        self.reg_a           = ret & 0xFF;
        self.reg_flag[ZERO]  = !self.reg_flag[ZERO];
    }
    fn and(&mut self, m: u8){
        self.reg_a = self.reg_a & m;
        self.reg_flag[SIGN] = if (self.reg_a & 0x80) == 0x80 { 1 } else { 0 };
        self.reg_flag[ZERO] = if self.reg_a == 0x00          { 1 } else { 0 };
    }
    fn asl(&mut self, b: u8) {
        self.reg_flag[CARRY]= if (b & 0x80) == 0x80 { 1 } else { 0 };
        let b = (b << 1) & 0xFE;
        self.reg_flag[SIGN] = if (b & 0x80) == 0x80 { 1 } else { 0 };
        self.reg_flag[ZERO] = if b == 0x00          { 1 } else { 0 };
    }
    fn cmp(&mut self, m: u8, z: u8) {
        let t = z - m;
        self.reg_flag[SIGN]  = if (t & 0x80) == 0x80 { 1 } else { 0 };
        self.reg_flag[CARRY] = if z >= m             { 1 } else { 0 };
        self.reg_flag[ZERO]  = if t == 0x00          { 1 } else { 0 };
    }
    fn eor(&mut self, m: u8){
        self.reg_a = self.reg_a ^ m;
        self.reg_flag[SIGN] = if (self.reg_a & 0x80) == 0x80 { 1 } else { 0 };
        self.reg_flag[ZERO] = if self.reg_a == 0             { 1 } else { 0 };
    }
    fn sign_zero_flags(&mut self, l: u8){
        self.reg_flag[SIGN] = if (l & 0x80) == 0x80 { 1 } else { 0 };
        self.reg_flag[ZERO] = if l == 0x00          { 1 } else { 0 };
    }
    fn lsr(&mut self, b: u8){
        self.reg_flag[SIGN] = 0;
        self.reg_flag[CARRY] = if (b & 0x01) == 0x01 { 1 } else { 0 };
        let b = (b >> 1) & 0x7F;
        self.reg_flag[ZERO]  = if b == 0x00          { 1 } else { 0 };
    }

    fn rol(&mut self,b : u8){
        let t = if (b & 0x80) == 0x80 { 1 } else { 0 };
        let b = (b << 1) & 0xFE;
        self.reg_flag[CARRY] = t;
        self.sign_zero_flags(b);
    }
    fn ror(&mut self, b: u8){
        let t = if (b & 0x01) == 0x01 { 1 } else { 0 };
        let b = (b >> 1) & 0x7F;
        self.reg_flag[CARRY] = t;
        self.sign_zero_flags(b);
    }
    fn sbc(&mut self, m: u8){
        let not_pc = !self.reg_flag[CARRY];
        let mut t;
        if self.reg_flag[DECIMAL] == 1 {
            t = self.reg_a - m - not_pc; // TODO: Binary Coded Decimal  on a and m
        } else{
            t = self.reg_a - m - not_pc;
            self.reg_flag[OVERFLOW] = if t > 127 { 1 } else { 0 }; // TODO: VERIFY ACCURACY OF THIS METHOD  ( || t < (0 - 128) )
        }
        self.reg_flag[CARRY] = if t >= 0 { 1 } else { 0 };
        self.reg_flag[SIGN]  = if (t & 0x80) == 0x80 { 0 } else { 1 };
        self.reg_flag[ZERO]  = if t == 0 { 1 } else { 0 };
        self.reg_a           = t & 0xFF;
    }
    fn st_axy(&mut self, addr: u16, obj: u8){
        self.write_u8(addr,obj);
    }
    fn incr_pc(&mut self,num: u8){
        self.reg_pc += num as u16;
    }
    fn convert_to_sign_zero_flags(&mut self, p: u8){
        self.reg_flag[SIGN]     = if (p & 0x80) == 0x80 { 1 } else { 0 };
        self.reg_flag[OVERFLOW] = if (p & 0x40) == 0x40 { 1 } else { 0 };
        self.reg_flag[BREAK]    = if (p & 0x10) == 0x10 { 1 } else { 0 };
        self.reg_flag[DECIMAL]  = if (p & 0x08) == 0x08 { 1 } else { 0 };
        self.reg_flag[INTERRUPT]= if (p & 0x04) == 0x04 { 1 } else { 0 };
        self.reg_flag[ZERO]     = if (p & 0x02) == 0x02 { 1 } else { 0 };
        self.reg_flag[CARRY]    = if (p & 0x01) == 0x01 { 1 } else { 0 };
    }
    /*fn set_zero(&mut self){self.reg_flag[ZERO] = 1;}
    fn clear_zero(&mut self){self.reg_flag[ZERO] = 0;}
    fn set_sign(&mut self){self.reg_flag[SIGN] = 1;}
    fn clear_sign(&mut self){self.reg_flag[SIGN] = 0;}
    fn set_break(&mut self){self.reg_flag[BREAK] = 1;}
    fn clear_break(&mut self){self.reg_flag[BREAK] = 0;}
    fn set_overflow(&mut self){self.reg_flag[OVERFLOW] = 1;}*/
    fn clear_overflow(&mut self){self.reg_flag[OVERFLOW] = 0;}
    fn set_decimal(&mut self){self.reg_flag[DECIMAL] = 1;}
    fn clear_decimal(&mut self){self.reg_flag[DECIMAL] = 0;}
    fn set_interrupt(&mut self){self.reg_flag[INTERRUPT] = 1;}
    fn clear_interrupt(&mut self){self.reg_flag[INTERRUPT] = 0;}
    fn set_carry(&mut self){self.reg_flag[CARRY] = 1;}
    fn clear_carry(&mut self){self.reg_flag[CARRY] = 0;}


    pub fn run(&mut self) {
        let mut x = 0;
        loop {
            if x == 10{ panic!("instructions Complete!");}
            self.run_instruction();
            x += 1;
            }
        }
    }

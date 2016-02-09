use super::memory;
use super::opcodes::Opcode::*;
use super::instructions::Instruction;

const SIGN: usize       = 7;
const OVERFLOW: usize   = 6;
const BREAK: usize      = 4;
const DECIMAL: usize    = 3;
const INTERRUPT: usize  = 2;
const ZERO_FLAG: usize  = 1;
const CARRY_FLAG: usize = 0;

fn check_overflow(M: u8, N: u8) -> bool{
    let result = M + N;
    if(((M^result)&(N^result)&0x80) == 0x00){
        return false;
    }
    return true
}

pub struct Cpu {
    reg_pc: u16, // 16 bit program counter The low and high 8-bit halves of the register are called PCL and PCH
    reg_sp: u8,  // 8 bit stack pointer // located at $0100-$01FF
    //reg_p:  u8,  // 6 bits used by alu
    reg_fr: Vec<u8>,  // 8 bit flag register
    // Work Registers
    reg_a: u8,   // Accumulator
    reg_x: u8,   // Indexed addressing
    reg_y: u8,    // Limited indexed addressing
    memory: memory::Memory
}
impl Cpu {
    pub fn new(mem: memory::Memory) -> Cpu {
        Cpu {
            reg_pc: 16,
            reg_sp: 0xFD,
            reg_fr: vec![0,0,1,1,0,1,0,0],
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            memory: mem,
        }
    }
    pub fn power_on_reset(&mut self) {
        self.reg_pc = 16;
        self.reg_sp = 0xFD;
        self.reg_fr = vec![0,0,1,1,0,1,0,0];
    }
    fn read_u8(&self, pc: u16) -> u8 {
        self.memory.read_u8(pc)
    }
    fn read_u16(&self, pc: u16) -> u16 {
        self.memory.read_u16(pc)
    }
    fn read_rom_u8(&self, pc: u16) -> u8 {
        self.memory.read_rom_u8(pc)
    }
    fn read_rom_u16(&self, pc: u16) -> u16 {
        self.memory.read_rom_u16(pc)
    }
    fn write_u8(&mut self, addr: u16, obj: u8) {
        self.memory.write_u8(addr, obj);
    }
    fn write_u16(&mut self, addr: u16, obj: u16){
        self.memory.write_u16(addr,obj);
    }
    fn run_instruction(&mut self){
        let instru = self.read_instruction();
        self.reg_pc += 1; // Increment PC
        // Adjust information
        println!("Instruction: {:#x}",instru.opcode);
        println!("Program Counter: {:#x}",self.reg_pc);
        self.execute_instruction(instru);
    }
    fn read_instruction(&mut self) -> Instruction {
        Instruction{opcode: self.read_rom_u8(self.reg_pc)}
    }
    fn execute_instruction(&mut self, instr: Instruction){
        match instr.opcode() {
              // The only thing changing is where the memory is coming from, so
              // simple get the correct chunk and run the helper program
              adc_immediate     => {
                                    let m = self.immediate();
                                    self.ADC(m);
                                },
              adc_zero_page     => {
                                    let m = self.zero_page();
                                    self.ADC(m);
                                },
              adc_zero_page_x   => {
                                    let m = self.zero_pagex();
                                    self.ADC(m);
                                },
              adc_absolute      => {
                                    let m = self.absolute();
                                    self.ADC(m);
                                },
              adc_absolute_x    => {
                                    let m = self.absolute_x();
                                    self.ADC(m);
                                },
              adc_absolute_y    => {
                                    let m = self.absolute_y();
                                    self.ADC(m);
                                },
              adc_indirect_x    => {
                                    let m = self.indirect_x();
                                    self.ADC(m);
                                },
              adc_indirect_y    => {
                                    let m = self.indirect_y();
                                    self.ADC(m);
                                },
              and_immediate     => {
                                    let m = self.immediate();
                                    self.AND(m);
                                },
              and_zero_page     => {
                                    let m = self.zero_page();
                                    self.AND(m);
                                },
              and_zero_page_x   => {
                                    let m = self.zero_pagex();
                                    self.AND(m);
                                },
              and_absolute      => {
                                    let m = self.absolute();
                                    self.AND(m);
                                },
              and_absolute_x    => {
                                    let m = self.absolute_x();
                                    self.AND(m);
                                },
              and_absolute_y    => {
                                    let m = self.absolute_y();
                                    self.AND(m);
                                },
              and_indirect_x    => {
                                    let m = self.indirect_x();
                                    self.AND(m);
                                },
              and_indirect_y    => {
                                    let m = self.indirect_y();
                                    self.AND(m);
                                },
              asl_zero_page     => {},
              asl_zero_page_x   => {},
              asl_absolute      => {},
              asl_absolute_x    => {},
              bcc               => {},
              bcs               => {},
              beq               => {},
              bit_zero_page     => {},
              bit_absolute      => {},
              bmi               => {
                                self.reg_pc += 2
                                },
              bne               => {},
              bpl               => {},
              brk               => {
                                    // incremenet PC Register by 2: 7 clock cycles
                                    // INTERRUPT bit = 1
                                    println!("Break!");
                                    self.reg_pc += 1
                                },
              bvc               => {},
              bvs               => {},
              // Clear / Set Instructions
              clc               => {self.clear_carry();}, // Clear Carry Flags
              cld               => {self.clear_decimal();}, // Clear Decimal Mode
              cli               => {self.clear_interrupt();}, // Clear Interrupt Disable Bit
              clv               => {self.clear_overflow();}, // Clear Overflow Flag
              sec               => {self.set_carry();}, // Set Carry Flag
              sed               => {self.set_decimal();}, // Set Decimal Mode
              sei               => {self.set_interrupt();}, // Set Interrupt Disable Bit
              cmp_immediate     => {},
              cmp_zero_page     => {},
              cmp_zero_page_x   => {},
              cmp_absolute      => {},
              cmp_absolute_x    => {},
              cmp_absolute_y    => {},
              cmp_indirect_x    => {},
              cmp_indirect_y    => {},
              cpx_immediate     => {},
              cpx_zero_page     => {},
              cpx_absolute      => {},
              cpy_immediate     => {},
              cpy_zero_page     => {},
              cpy_absolute      => {},

              dec_zero_page     => {},
              dec_zero_page_x   => {},
              dec_absolute      => {},
              dec_absolute_x    => {},

              dex               => {self.reg_x -= 1;}, // Decrement index X by one
              dey               => {self.reg_y -= 1;}, // Decrement index Y by one
              inx               => {self.reg_x += 1;}, // Increment index X by one
              iny               => {self.reg_y += 1;}, // Increment index Y by one
              eor_immediate     => {},
              eor_zero_page     => {},
              eor_zero_page_x   => {},
              eor_absolute      => {},
              eor_absolute_x    => {},
              eor_absolute_y    => {},
              eor_indirect_x    => {},
              eor_indirect_y    => {},

              inc_zero_page     => {},
              inc_zero_page_x   => {},
              inc_absolute      => {},
              inc_absolute_x    => {},

              jmp_absolute      => {},
              jmp_indirect      => {},
              jsr_absolute      => {// push (PC+2),(PC+1) -> PCL,(PC+2) -> PCH
                                    // TODO: Reconsider stack setup. Specialized functions or actual stack might be cool
                                    let pc = self.reg_pc + 2;
                                    let sp = self.reg_sp as u16;

                                    self.write_u16(sp, pc); // Push PC + 2 onto stack
                                    let jsr_addr = self.read_u16(self.reg_pc+1);
                                    println!("{}",jsr_addr);
                                    self.reg_pc = jsr_addr;
                                    //1    PC     R  fetch opcode, increment PC
                                    //2    PC     R  fetch low address byte, increment PC
                                    //3  $0100,S  R  internal operation (predecrement S?)
                                    //4  $0100,S  W  push PCH on stack, decrement S
                                    //5  $0100,S  W  push PCL on stack, decrement S
                                    //6    PC     R  copy low address byte to PCL, fetch high address byte to PCH
;
                                    },
              lda_immediate     => {},
              lda_zero_page     => {},
              lda_zero_page_x   => {},
              lda_absolute      => {},
              lda_absolute_x    => {},
              lda_absolute_y    => {},
              lda_indirect_x    => {},
              lda_indirect_y    => {},

              ldx_immediate     => {},
              ldx_zero_page     => {},
              ldx_zero_page_x   => {},
              ldx_absolute      => {},
              ldx_absolute_x    => {},

              ldy_immediate     => {},
              ldy_zero_page     => {},
              ldy_zero_page_x   => {},
              ldy_absolute      => {},
              ldy_absolute_x    => {},

              lsr_accumulator   => {},
              lsr_zero_page     => {},
              lsr_zero_page_x   => {},
              lsr_absolute      => {},
              lsr_absolute_x    => {},

              nop_implied       => { /* No operation 2 cycles */},
              // A OR M --> A
              ora_immediate     => {
                                    println!("ora!");
                                    self.reg_a = self.reg_a | self.read_u8(self.reg_pc);
                                    self.reg_pc += 1;
                                },
              ora_zero_page     => {
                                    println!("ora!");
                                    self.reg_a = self.reg_a | self.read_u8(self.reg_pc);
                                    self.reg_pc += 1;
                                },
              ora_zero_page_x   => {
                                    println!("ora!");
                                    self.reg_a = self.reg_a | self.read_u8(self.reg_pc);
                                    self.reg_pc += 1;
                                },
              ora_absolute      => {
                                    println!("ora!");
                                    self.reg_a = self.reg_a | self.read_u8(self.reg_pc);
                                    self.reg_pc += 1;
                                },
              ora_absolute_x    => {
                                    println!("ora!");
                                    self.reg_a = self.reg_a | self.read_u8(self.reg_pc);
                                    self.reg_pc += 1;
                                },
              ora_absolute_y    => {
                                    println!("ora!");
                                    self.reg_a = self.reg_a | self.read_u8(self.reg_pc);
                                    self.reg_pc += 1;
                                },
              ora_indirect_x    => {
                                    println!("ora!");
                                    self.reg_a = self.reg_a | self.read_u8(self.reg_pc);
                                    self.reg_pc += 1;
                                },
              ora_indirect_y    => {
                                    println!("ora!");
                                    self.reg_a = self.reg_a | self.read_u8(self.reg_pc);
                                    self.reg_pc += 1;
                                },
              pha               => {},
              php               => {},
              pla               => {},
              plp               => {},
              rol_accumulator   => {},
              rol_zero_page     => {},
              rol_zero_page_x   => {},
              rol_absolute      => {},
              rol_absolute_x    => {},
              ror_accumulator   => {},
              ror_zero_page     => {},
              ror_zero_page_x   => {},
              ror_absolute      => {},
              ror_absolute_x    => {},
              rti               => { // Return from interrupt

                                },
              rts               => { // Return from subroutine: Operation:  PC from S, PC + 1 -> PC

                                },
              sbc_immediate     => {
                                    let m = self.read_u8(self.reg_pc);
                                    self.SBC(m);
                                },
              sbc_zero_page     => {},
              sbc_zero_page_x   => {},
              sbc_absolute      => {},
              sbc_absolute_x    => {},
              sbc_absolute_y    => {},
              sbc_indirect_x    => {},
              sbc_indirect_y    => {},
              sta_zero_page     => {},
              sta_zero_page_x   => {},
              sta_absolute      => {},
              sta_absolute_x    => {},
              sta_absolute_y    => {},
              sta_indirect_x    => {},
              sta_indirect_y    => {},
              stx_zero_page     => {},
              stx_zero_page_x   => {},
              stx_absolute      => {},
              sty_zero_page     => { // Store index y in memory: 2 bytes

                                },
              sty_zero_page_x   => { // Store index y in memory: 2 bytes

                                },
              sty_absolute      => { // Store index y in memory: 3 bytes

                                },
              tax               => {self.reg_x = self.reg_a;},  // Transfer Accumulator to index x
              tay               => {self.reg_y = self.reg_a;},  // Transfer Accumulator to index y
              tsx               => {self.reg_x = self.reg_sp;}, // Transfer Stack Pointer to index x
              txa               => {self.reg_a = self.reg_x;},  // Transfer index X to accumulator
              txs               => {self.reg_sp = self.reg_x;}, // Transfer index X to stack pointer
              tya               => {self.reg_a = self.reg_y;},  // Transfer index Y to accumulator
              _ => {
                  panic!("instruction not found!");
              }
        }
    }
    fn immediate(&mut self) -> u8 {
        let m = self.read_u8(self.reg_pc);
        self.reg_pc += 1;
        m
    }
    fn zero_page(&mut self) -> u8 {
        let _addr = self.read_u8(self.reg_pc);  //Fetch the operand from the next byte
        self.reg_pc += 1;                       //Increment the PC
        self.read_u8(_addr as u16)             //Fetch the value of the address/operand
    }
    fn zero_pagex(&mut self) -> u8 {
        let mut _addr = self.read_u8(self.reg_pc);  //Fetch the operand from the next byte
        self.reg_pc += 1;                           //Increment the PC
        _addr = (_addr + self.reg_x) & 0xFF;        //Add the X register to the address and wrap around if address +X > 0xF
        self.read_u8(_addr as u16)
    }
    fn absolute(&mut self) -> u8 {
        let _addr = self.read_u16(self.reg_pc); // Fetch the next addr pointing anywhere in memory
        self.reg_pc += 2;
        self.read_u8(_addr)
    }
    fn absolute_x(&mut self) -> u8 {
        let mut _addr = self.read_u16(self.reg_pc); // Fetch the next addr pointing anywhere in memory
        self.reg_pc += 2;
        let a = (_addr & 0x100);
        _addr += (self.reg_x as u16);
        //if((_addr & 0x100) ^ a){
        //    self.read_u8(_addr);
        //}
        self.read_u8(_addr)
    }
    fn absolute_y(&mut self) -> u8 {
        let mut _addr = self.read_u16(self.reg_pc); // Fetch the next addr pointing anywhere in memory
        self.reg_pc += 2;
        let a = (_addr & 0x100);
        _addr += (self.reg_y as u16);
        //if((_addr & 0x100) ^ a){
        //    self.read_u8(_addr);
        //}
        self.read_u8(_addr)
    }
    fn indirect_x(&mut self) -> u8 {
        let mut addr = self.read_u8(self.reg_pc);  // Fetch the pointer address from the next byte in the PC
        self.reg_pc += 1;
        let tmp = (addr + self.reg_x) & 0xFF;
        let _addr = self.read_u16(tmp as u16);
        self.read_u8(_addr)
    }
    fn indirect_y(&mut self) -> u8 {
        let mut _addr = self.read_u8(self.reg_pc); //u8
        let mut _addr = _addr as u16;
        self.reg_pc += 1;
        let mut addr = self.read_u16(_addr); //u16
        _addr = addr & 0x100;
        addr += (self.reg_y as u16);
        if((addr & 0x100) != _addr){
            self.read_u8(addr);
        }
        self.read_u8(addr)
    }
    // Basic functions
    fn ADC(&mut self, m: u8){ // Add Memory to Accumulator with Carry: A + M + C -> A, C
        let ret = self.reg_a + m + self.reg_fr[CARRY_FLAG];

        self.reg_fr[OVERFLOW] = (ret ^ self.reg_a) & (ret ^ m) & 0x80;
        self.reg_fr[CARRY_FLAG  ] = (ret & 0x100); //>> 8;
        self.reg_fr[ZERO_FLAG]  = ret & 0xFF;
        self.reg_fr[SIGN]       = ret & 0xFF;
        self.reg_a              = ret & 0xFF;
        self.reg_fr[ZERO_FLAG]  = !self.reg_fr[ZERO_FLAG];
    }
    fn AND(&mut self, m: u8){
        self.reg_a = self.reg_a & m;
        if((self.reg_a & 0x80) == 0x80){
            self.set_sign();
        }
        else {
            self.clear_sign();
        }
        if(self.reg_a = 0x00){
            self.set_zero();
        }
        else {
            self.clear_zero();
        }
    }
    fn SBC(&mut self, m: u8){
        let ret = self.reg_a - m - !self.reg_fr[CARRY_FLAG];

        self.reg_fr[OVERFLOW]   = (ret ^ self.reg_a) & (ret ^ m) & 0x80;
        self.reg_fr[CARRY_FLAG] = !(ret & 0x100); //>> 8;
        self.reg_fr[ZERO_FLAG]  = ret & 0xFF;
        self.reg_fr[SIGN]       = ret & 0xFF;
        self.reg_a              = ret & 0xFF;
        self.reg_fr[ZERO_FLAG]  = !self.reg_fr[ZERO_FLAG];
    }
    fn set_sign(&mut self){self.reg_fr[SIGN] = 1;}
    fn clear_sign(&mut self){self.reg_fr[SIGN] = 0;}
    fn set_overflow(&mut self){self.reg_fr[OVERFLOW] = 1;}
    fn clear_overflow(&mut self){self.reg_fr[OVERFLOW] = 0;}
    fn set_break(&mut self){self.reg_fr[BREAK] = 1;}
    fn clear_break(&mut self){self.reg_fr[BREAK] = 0;}
    fn set_decimal(&mut self){self.reg_fr[DECIMAL] = 1;}
    fn clear_decimal(&mut self){self.reg_fr[DECIMAL] = 0;}
    fn set_interrupt(&mut self){self.reg_fr[INTERRUPT] = 1;}
    fn clear_interrupt(&mut self){self.reg_fr[INTERRUPT] = 0;}
    fn set_zero(&mut self){self.reg_fr[ZERO_FLAG] = 1;}
    fn clear_zero(&mut self){self.reg_fr[ZERO_FLAG] = 0;}
    fn set_carry(&mut self){self.reg_fr[CARRY_FLAG] = 1;}
    fn clear_carry(&mut self){self.reg_fr[CARRY_FLAG] = 0;}


    pub fn run(&mut self) {
        let mut x = 0;
        loop {
            if x == 10{ panic!("instructions Complete!");}
            self.run_instruction();
            x += 1;
            }
        }
    }

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
#[derive(Default)]
pub struct Cpu {
    reg_pc: u16, // 16 bit program counter
    reg_sp: u8,  // 8 bit stack pointer
    //reg_p:  u8,  // 6 bits used by alu
    reg_fr: Vec<u8>,  // 8 bit flag register
    // Work Registers
    reg_a: u8,   // Accumulator
    reg_x: u8,   // Indexed addressing
    reg_y: u8,    // Limited indexed addressing
    memory: memory::Memory
}
impl Cpu {
    pub fn power_on_reset(&mut self) {
        self.reg_pc = 16;
        self.reg_sp = 0xFD;
        self.reg_fr = vec![0,0,1,1,0,1,0,0];
    }
    fn read_word(&mut self) -> u8 {
        self.memory.read_word(self.reg_pc)
    }
    fn write_word(&mut self, addr: u16, obj: u8) {
        self.memory.write_word(addr, obj);
    }
    fn read_instruction(&mut self) -> Instruction {
        Instruction{opcode: self.read_word()}
    }
    fn run_instruction(&mut self){
        let instru = self.read_instruction();
        // Adjust information
        println!("Instruction: {:#x}",instru.opcode);
        self.execute_instruction(instru);
    }
    fn execute_instruction(&mut self, instr: Instruction){
        match instr.opcode() {
              adc_immediate     => { // Add Memory to Accumulator with Carry: A + M + C -> A, C
                                    self.reg_pc += 1;
                                    let m = self.read_word();
                                    if(check_overflow(m,self.reg_a)){
                                        self.set_overflow();
                                    }
              },
              adc_zero_page     => {},
              adc_zero_page_x   => {},
              adc_absolute      => {},
              adc_absolute_x    => {},
              adc_absolute_y    => {},
              adc_indirect_x    => {},
              adc_indirect_y    => {},
              and_immediate     => {},
              and_zero_page     => {},
              and_zero_page_x   => {},
              and_absolute      => {},
              and_absolute_x    => {},
              and_absolute_y    => {},
              and_indirect_x    => {},
              and_indirect_y    => {},
              asl_zero_page     => {},
              asl_zero_page_x   => {},
              asl_absolute      => {},
              asl_absolute_x    => {},
              bcc               => {},
              bcs               => {},
              beq               => {},
              bit_zero_page     => {},
              bit_absolute      => {},
              bmi               => {},
              bne               => {},
              bpl               => {},
              brk               => {
                                    // incremenet PC Register by 2: 7 clock cycles
                                    // INTERRUPT bit = 1
                                    println!("Break!");
                                    self.reg_pc += 2
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
              jsr_absolute      => {},

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
              ora_immediate     => {},
              ora_zero_page     => {},
              ora_zero_page_x   => {},
              ora_absolute      => {},
              ora_absolute_x    => {},
              ora_absolute_y    => {},
              ora_indirect_x    => {println!("ora!");},
              ora_indirect_y    => {},
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
              sbc_immediate     => {},
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


    pub fn run(&mut self,rom: &Vec<u8>) {
        loop {
            self.run_instruction();
            panic!("instruction Complete!");


            }
        }
    }

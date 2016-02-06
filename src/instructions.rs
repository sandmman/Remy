use num::FromPrimitive;
use super::opcodes::Opcode;

pub struct Instruction{
  pub opcode: u8,
}
impl Instruction {
    pub fn opcode(&self) -> Opcode {
        println!("{}",self.opcode);
        Opcode::from_u8((self.opcode) & 0xff).unwrap_or_else(
            || panic!("Unrecognized instruction: {:#x}", self.opcode))
    }
    /*fn ADC_immediate(&mut self){}// ADd with Carry: Length 2
    fn ADC_zero_page(&mut self){}// ADd with Carry: Length 2
    fn ADC_zero_page_x(&mut self){}// ADd with Carry: Length 2
    fn ADC_absolute(&mut self){} // ADd with Carry: Length 3
    fn ADC_absolute_x(&mut self){}// ADd with Carry: Length 3
    fn ADC_absolute_y(&mut self){}// ADd with Carry: Length 3
    fn ADC_indirect_x(&mut self){}// ADd with Carry: Length 2
    fn ADC_indirect_y(&mut self){}// ADd with Carry: Length 2

    fn AND_immediate(&mut self){}// Bitwise AND with accumultor: Length 2
    fn AND_zero_page(&mut self){}// Bitwise AND with accumultor: Length 2
    fn AND_zero_page_x(&mut self){}// Bitwise AND with accumultor: Length 2
    fn AND_absolute(&mut self){}// Bitwise AND with accumultor: Length 3
    fn AND_absolute_x(&mut self){}// Bitwise AND with accumultor: Length 3
    fn AND_absolute_y(&mut self){}// Bitwise AND with accumultor: Length 3
    fn AND_indirect_x(&mut self){}// Bitwise AND with accumultor: Length 2
    fn AND_indirect_y(&mut self){}// Bitwise AND with accumultor: Length 2

    fn ASL_accumulator(&mut self){}// Arithmetic Shift Left: Length 1
    fn ASL_zero_page(&mut self){}// Arithmetic Shift Left: Length 2
    fn ASL_zero_page_x(&mut self){}// Arithmetic Shift Left: Length 2
    fn ASL_absolute(&mut self){}// Arithmetic Shift Left: Length 3
    fn ASL_absolute_x(&mut self){}// Arithmetic Shift Left: Length 3

    fn BIT_zero_page(&mut self){}// test BITs
    fn BIT_absolute(&mut self){}// test BITs
    // Branch Instructions
    fn BCC(&mut self){}// Beanch on Carry Clear
    fn BCS(&mut self){}// Branch on Carry Set
    fn BEQ(&mut self){}// Branch on Equal
    fn BMI(&mut self){}// Branch on Minus
    fn BNE(&mut self){}// Branch on Not Equal
    fn BPL(&mut self){}// Branch of Plus
    fn BVC(&mut self){}// Branch on overflow clear
    fn BVS(&mut self){}// Branch on overflow set

    fn BRK(&mut self){}// Break

    // Clear/Set Operations
    fn CLC(&mut self){}// Clear Carry
    fn CLD(&mut self){}// Clear Decimal
    fn CLI(&mut self){}// Clear Interrupt
    fn CLV(&mut self){}// Clear Overflow
    fn SEC(&mut self){} // Set Carry
    fn SED(&mut self){} // Set Decimal
    fn SEI(&mut self){}// Set Interrupt

    fn CMP_immediate(&mut self){}// Compare Accumulator: Length 2
    fn CMP_zero_page(&mut self){}// Compare Accumulator: Length 2
    fn CMP_zero_page_x(&mut self){}// Compare Accumulator: Length 2
    fn CMP_absolute(&mut self){}// Compare Accumulator: Length 3
    fn CMP_absolute_x(&mut self){}// Compare Accumulator: Length 3
    fn CMP_absolute_y(&mut self){}// Compare Accumulator: Length 3
    fn CMP_indirect_x(&mut self){}// Compare Accumulator: Length 2
    fn CMP_indirect_y(&mut self){}// Compare Accumulator: Length 2

    fn CPX_immediate(&mut self){}// Compare X Register: Length 2
    fn CPX_zero_page(&mut self){}// Compare X Register: Length 2
    fn CPX_absolute(&mut self){}// Compare X Register: Length 3
    fn CPY_immediate(&mut self){}// Compare Y Register: Length 2
    fn CPY_zero_page(&mut self){}// Compare Y Register: Length 2
    fn CPY_absolute(&mut self){}// Compare Y Register: Length 3

    fn DEC_zero_page(&mut self){}// Decrement Memory: Length 2
    fn DEC__zero_page_x(&mut self){}// Decrement Memory: Length 2
    fn DEC_absolute(&mut self){}// Decrement Memory: Length 3
    fn DEC__absolute_x(&mut self){}// Decrement Memory: Length 3

    fn EOR_immediate(&mut self){}// Bitwise exlusive OR: Length 2
    fn EOR_zero_page(&mut self){}// Bitwise exlusive OR: Length 2
    fn EOR_zero_page_x(&mut self){} // Bitwise exlusive OR: Length 2
    fn EOR_absolute(&mut self){}// Bitwise exlusive OR: Length 3
    fn EOR_absolute_x(&mut self){}// Bitwise exlusive OR: Length 3
    fn EOR_absolute_y(&mut self){}// Bitwise exlusive OR: Length 3
    fn EOR_indirect_x(&mut self){}// Bitwise exlusive OR: Length 2
    fn EOR_indirect_y(&mut self){}// Bitwise exlusive OR: Length 2

    fn INC_zero_page(&mut self){}// Increment Memory: Length 2
    fn INC_zero_page_x(&mut self){}// Increment Memory: Length 2
    fn INC_absolute(&mut self){} // Increment Memory: Length 3
    fn INC_absolute_x(&mut self){} // Increment Memory: Length 3

    fn INX(&mut self){} // Increment X
    fn INY(&mut self){} // Increment Y
    fn DEX(&mut self){} // Decrement X
    fn DEY(&mut self){} // Decrement Y
    fn JMP_absolute(&mut self){} // Jump absolute(){}: Length 3
    fn JMP_indirect(&mut self){} // Jump indirect(){}: Length 3
    fn JSR_absolute(&mut self){}  // Jump to SubRoutine

    fn LDA_immediate(&mut self){} // Load Accumulator: Length 2
    fn LDA_zero_page(&mut self){}  // Load Accumulator: Length 2
    fn LDA_zero_page_x(&mut self){} // Load Accumulator: Length 2
    fn LDA_absolute(&mut self){}   // Load Accumulator: Length 3
    fn LDA_absolute_x(&mut self){}  // Load Accumulator: Length 3
    fn LDA_absolute_y(&mut self){}  // Load Accumulator: Length 3
    fn LDA_indirect_x(&mut self){}  // Load Accumulator: Length 2
    fn LDA_indirect_y(&mut self){}  // Load Accumulator: Length 2

    fn LDX_immediate(&mut self){}  // Load X Register: Length 2
    fn LDX_zero_page(&mut self){}  // Load X Register: Length 2
    fn LDX_zero_page_y(&mut self){}  // Load X Register: Length 2
    fn LDX_absolute(&mut self){}   // Load X Register: Length 3
    fn LDX_absolute_y(&mut self){}// Load X Register: Length 3

    fn LDY_immediate(&mut self){}  // Load Y Register: Length 2
    fn LDY_zero_page(&mut self){}  // Load Y Register: Length 2
    fn LDY_zero_page_x(&mut self){} // Load Y Register: Length 2
    fn LDY_absolute(&mut self){}   // Load Y Register: Length 3
    fn LDY_absolute_x(&mut self){} // Load Y Register: Length 3

    fn LSR_accumulator(&mut self){}// Logical Shift Right: Length 1
    fn LSR_zero_page(&mut self){} // Logical Shift Right: Length 2
    fn LSR_zero_page_x(&mut self){}// Logical Shift Right: Length 2
    fn LSR_absolute(&mut self){}  // Logical Shift Right: Length 3
    fn LSR_absolute_x(&mut self){} // Logical Shift Right: Length 3

    fn NOP_implied(&mut self){}// No operation

    fn ORA_immediate(&mut self){} // Bitwise OR with Accumulator: Length 2
    fn ORA_zero_page(&mut self){} // Bitwise OR with Accumulator: Length 2
    fn ORA_zero_page_x(&mut self){} // Bitwise OR with Accumulator: Length 2
    fn ORA_absolute(&mut self){} // Bitwise OR with Accumulator: Length 3
    fn ORA_absolute_x(&mut self){} // Bitwise OR with Accumulator: Length 3
    fn ORA_absolute_y(&mut self){}  // Bitwise OR with Accumulator: Length 3
    fn ORA_indirect_x(&mut self){}  // Bitwise OR with Accumulator: Length 2
    fn ORA_indirect_y(&mut self){} // Bitwise OR with Accumulator: Length 2
    //stack Instructions
    fn TXS(&mut self){} // Transfer X to stack ptr
    fn TSX(&mut self){} // Transfer stack ptr to X
    fn PHA(&mut self){} // Push Accumulator
    fn PHP(&mut self){} // Push Processor status
    fn PLA(&mut self){} // Pull Acumulator
    fn PLP(&mut self){} // Pull Process status

    fn rol_accumulator(&mut self){}  // Rotate Left: Length 1
    fn rol_zero_page(&mut self){} // Rotate Left: Length 2
    fn rol_zero_page_x(&mut self){} // Rotate Left: Length 2
    fn rol_absolute(&mut self){}  // Rotate Left: Length 3
    fn rol_absolute_x(&mut self){}   // Rotate Left: Length 3

    fn ror_accumulator(&mut self){} // Rotate Right: Length 1
    fn ror_zero_page(&mut self){} // Rotate Right: Length 2
    fn ror_zero_page_x(&mut self){} // Rotate Right: Length 2
    fn ror_absolute(&mut self){}  // Rotate Right: Length 3
    fn ror_absolute_x(&mut self){} // Rotate Right: Length 3

    fn RTI(&mut self){}// Return from Interrupt
    fn RTS(&mut self){} // Return from Subroutine

    fn sbc_immediate(&mut self){} // Subtract with Carry: Length 2
    fn sbc_zero_page(&mut self){} // Subtract with Carry: Length 2
    fn sbc_zero_page_x(&mut self){}// Subtract with Carry: Length 2
    fn sbc_absolute(&mut self){}// Subtract with Carry: Length 3
    fn sbc_absolute_x(&mut self){} // Subtract with Carry: Length 3
    fn sbc_absolute_y(&mut self){} // Subtract with Carry: Length 3
    fn sbc_indirect_x(&mut self){} // Subtract with Carry: Length 2
    fn sbc_indirect_y(&mut self){} // Subtract with Carry: Length 2

    fn sta_zero_page(&mut self){}// Store Accumulator: Length 2
    fn sta_zero_page_x(&mut self){}  // Store Accumulator: Length 2
    fn sta_absolute(&mut self){} // Store Accumulator: Length 3
    fn sta_absolute_x(&mut self){} // Store Accumulator: Length 3
    fn sta_absolute_y(&mut self){} // Store Accumulator: Length 3
    fn sta_indirect_x(&mut self){} // Store Accumulator: Length 2
    fn sta_indirect_y(&mut self){}  // Store Accumulator: Length 2

    fn stx_zero_page(&mut self){} // Store X register: Length 2
    fn stx_zero_page_y(&mut self){}// Store X register: Length 2
    fn stx_absolute(&mut self){} // Store X register: Length 3
    fn sty_zero_page(&mut self){}// Store Y register: Length 2
    fn sty_zero_page_x(&mut self){} // Store Y register: Length 2
    fn sty_absolute(&mut self){}// Store Y register: Length 3

    fn tax(&mut self){} // Transfer A to X
    fn TAY(&mut self){}// Tranfer A to Y
    fn txa(&mut self){}  // Transfer X to A
    fn TYA(&mut self){} // Transfer Y to A*/
}

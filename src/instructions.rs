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
    /*fn Adcimmediate(&mut self){}// ADd with Carry: Length 2
    fn Adczero_page(&mut self){}// ADd with Carry: Length 2
    fn Adczero_page_x(&mut self){}// ADd with Carry: Length 2
    fn Adcabsolute(&mut self){} // ADd with Carry: Length 3
    fn Adcabsolute_x(&mut self){}// ADd with Carry: Length 3
    fn Adcabsolute_y(&mut self){}// ADd with Carry: Length 3
    fn Adcindirect_x(&mut self){}// ADd with Carry: Length 2
    fn Adcindirect_y(&mut self){}// ADd with Carry: Length 2

    fn Andimmediate(&mut self){}// Bitwise AND with accumultor: Length 2
    fn Andzero_page(&mut self){}// Bitwise AND with accumultor: Length 2
    fn Andzero_page_x(&mut self){}// Bitwise AND with accumultor: Length 2
    fn AndAbsolute(&mut self){}// Bitwise AND with accumultor: Length 3
    fn AndAbsoluteX(&mut self){}// Bitwise AND with accumultor: Length 3
    fn AndAbsoluteY(&mut self){}// Bitwise AND with accumultor: Length 3
    fn Andindirect_x(&mut self){}// Bitwise AND with accumultor: Length 2
    fn Andindirect_y(&mut self){}// Bitwise AND with accumultor: Length 2

    fn Aslaccumulator(&mut self){}// Arithmetic Shift Left: Length 1
    fn Aslzero_page(&mut self){}// Arithmetic Shift Left: Length 2
    fn Aslzero_page_x(&mut self){}// Arithmetic Shift Left: Length 2
    fn AslAbsolute(&mut self){}// Arithmetic Shift Left: Length 3
    fn AslAbsoluteX(&mut self){}// Arithmetic Shift Left: Length 3

    fn Bitzero_page(&mut self){}// test BITs
    fn BitAbsolute(&mut self){}// test BITs
    // Branch Instructions
    fn Bcc(&mut self){}// Beanch on Carry Clear
    fn Bcs(&mut self){}// Branch on Carry Set
    fn Beq(&mut self){}// Branch on Equal
    fn Bmi(&mut self){}// Branch on Minus
    fn Bne(&mut self){}// Branch on Not Equal
    fn Bpl(&mut self){}// Branch of Plus
    fn BVC(&mut self){}// Branch on overflow clear
    fn BVS(&mut self){}// Branch on overflow set

    fn Brk(&mut self){}// Break

    // Clear/Set Operations
    fn CLC(&mut self){}// Clear Carry
    fn CLD(&mut self){}// Clear Decimal
    fn CLI(&mut self){}// Clear Interrupt
    fn CLV(&mut self){}// Clear Overflow
    fn SEC(&mut self){} // Set Carry
    fn SED(&mut self){} // Set Decimal
    fn SEI(&mut self){}// Set Interrupt

    fn Cmpimmediate(&mut self){}// Compare Accumulator: Length 2
    fn Cmpzero_page(&mut self){}// Compare Accumulator: Length 2
    fn Cmpzero_page_x(&mut self){}// Compare Accumulator: Length 2
    fn CmpAbsolute(&mut self){}// Compare Accumulator: Length 3
    fn CmpAbsoluteX(&mut self){}// Compare Accumulator: Length 3
    fn CmpAbsoluteY(&mut self){}// Compare Accumulator: Length 3
    fn Cmpindirect_x(&mut self){}// Compare Accumulator: Length 2
    fn Cmpindirect_y(&mut self){}// Compare Accumulator: Length 2

    fn Cpximmediate(&mut self){}// Compare X Register: Length 2
    fn Cpxzero_page(&mut self){}// Compare X Register: Length 2
    fn CpxAbsolute(&mut self){}// Compare X Register: Length 3
    fn Cpyimmediate(&mut self){}// Compare Y Register: Length 2
    fn Cpyzero_page(&mut self){}// Compare Y Register: Length 2
    fn CpyAbsolute(&mut self){}// Compare Y Register: Length 3

    fn Deczero_page(&mut self){}// Decrement Memory: Length 2
    fn Dec_zero_page_x(&mut self){}// Decrement Memory: Length 2
    fn DecAbsolute(&mut self){}// Decrement Memory: Length 3
    fn Dec_absolute_x(&mut self){}// Decrement Memory: Length 3

    fn Eorimmediate(&mut self){}// Bitwise exlusive OR: Length 2
    fn Eorzero_page(&mut self){}// Bitwise exlusive OR: Length 2
    fn Eorzero_page_x(&mut self){} // Bitwise exlusive OR: Length 2
    fn EorAbsolute(&mut self){}// Bitwise exlusive OR: Length 3
    fn EorAbsoluteX(&mut self){}// Bitwise exlusive OR: Length 3
    fn EorAbsoluteY(&mut self){}// Bitwise exlusive OR: Length 3
    fn Eorindirect_x(&mut self){}// Bitwise exlusive OR: Length 2
    fn Eorindirect_y(&mut self){}// Bitwise exlusive OR: Length 2

    fn Inczero_page(&mut self){}// Increment Memory: Length 2
    fn Inczero_page_x(&mut self){}// Increment Memory: Length 2
    fn IncAbsolute(&mut self){} // Increment Memory: Length 3
    fn IncAbsoluteX(&mut self){} // Increment Memory: Length 3

    fn INX(&mut self){} // Increment X
    fn INY(&mut self){} // Increment Y
    fn DEX(&mut self){} // Decrement X
    fn DEY(&mut self){} // Decrement Y
    fn Jmpabsolute(&mut self){} // Jump absolute(){}: Length 3
    fn Jmpindirect(&mut self){} // Jump indirect(){}: Length 3
    fn JSR_absolute(&mut self){}  // Jump to SubRoutine

    fn Ldaimmediate(&mut self){} // Load Accumulator: Length 2
    fn Ldazero_page(&mut self){}  // Load Accumulator: Length 2
    fn Ldazero_page_x(&mut self){} // Load Accumulator: Length 2
    fn LdaAbsolute(&mut self){}   // Load Accumulator: Length 3
    fn LdaAbsoluteX(&mut self){}  // Load Accumulator: Length 3
    fn LdaAbsoluteY(&mut self){}  // Load Accumulator: Length 3
    fn Ldaindirect_x(&mut self){}  // Load Accumulator: Length 2
    fn Ldaindirect_y(&mut self){}  // Load Accumulator: Length 2

    fn Ldximmediate(&mut self){}  // Load X Register: Length 2
    fn Ldxzero_page(&mut self){}  // Load X Register: Length 2
    fn Ldxzero_page_y(&mut self){}  // Load X Register: Length 2
    fn Ldxabsolute(&mut self){}   // Load X Register: Length 3
    fn Ldxabsolute_y(&mut self){}// Load X Register: Length 3

    fn Ldyimmediate(&mut self){}  // Load Y Register: Length 2
    fn Ldyzero_page(&mut self){}  // Load Y Register: Length 2
    fn Ldyzero_page_x(&mut self){} // Load Y Register: Length 2
    fn LdyAbsolute(&mut self){}   // Load Y Register: Length 3
    fn LdyAbsoluteX(&mut self){} // Load Y Register: Length 3

    fn Lsraccumulator(&mut self){}// Logical Shift Right: Length 1
    fn Lsrzero_page(&mut self){} // Logical Shift Right: Length 2
    fn Lsrzero_page_x(&mut self){}// Logical Shift Right: Length 2
    fn LSRAbsolute(&mut self){}  // Logical Shift Right: Length 3
    fn LSRAbsoluteX(&mut self){} // Logical Shift Right: Length 3

    fn NOP_implied(&mut self){}// No operation

    fn Oraimmediate(&mut self){} // Bitwise OR with Accumulator: Length 2
    fn Orazero_page(&mut self){} // Bitwise OR with Accumulator: Length 2
    fn Orazero_page_x(&mut self){} // Bitwise OR with Accumulator: Length 2
    fn OraAbsolute(&mut self){} // Bitwise OR with Accumulator: Length 3
    fn OraAbsoluteX(&mut self){} // Bitwise OR with Accumulator: Length 3
    fn OraAbsoluteY(&mut self){}  // Bitwise OR with Accumulator: Length 3
    fn Oraindirect_x(&mut self){}  // Bitwise OR with Accumulator: Length 2
    fn Oraindirect_y(&mut self){} // Bitwise OR with Accumulator: Length 2
    //stack Instructions
    fn Txs(&mut self){} // Transfer X to stack ptr
    fn TSX(&mut self){} // Transfer stack ptr to X
    fn PHA(&mut self){} // Push Accumulator
    fn PHP(&mut self){} // Push Processor status
    fn PLA(&mut self){} // Pull Acumulator
    fn PLP(&mut self){} // Pull Process status

    fn Rolaccumulator(&mut self){}  // Rotate Left: Length 1
    fn Rolzero_page(&mut self){} // Rotate Left: Length 2
    fn Rolzero_page_x(&mut self){} // Rotate Left: Length 2
    fn Rolabsolute(&mut self){}  // Rotate Left: Length 3
    fn RolAbsoluteX(&mut self){}   // Rotate Left: Length 3

    fn Roraccumulator(&mut self){} // Rotate Right: Length 1
    fn Rorzero_page(&mut self){} // Rotate Right: Length 2
    fn Rorzero_page_x(&mut self){} // Rotate Right: Length 2
    fn RorAbsolute(&mut self){}  // Rotate Right: Length 3
    fn RorAbsoluteX(&mut self){} // Rotate Right: Length 3

    fn Rti(&mut self){}// Return from Interrupt
    fn Rts(&mut self){} // Return from Subroutine

    fn SbcImmediate(&mut self){} // Subtract with Carry: Length 2
    fn SbcZeroPage(&mut self){} // Subtract with Carry: Length 2
    fn SbcZeroPageX(&mut self){}// Subtract with Carry: Length 2
    fn SbcAbsolute(&mut self){}// Subtract with Carry: Length 3
    fn SbcAbsoluteX(&mut self){} // Subtract with Carry: Length 3
    fn SbcAbsoluteY(&mut self){} // Subtract with Carry: Length 3
    fn SbcIndirectX(&mut self){} // Subtract with Carry: Length 2
    fn SbcIndirectY(&mut self){} // Subtract with Carry: Length 2

    fn StaZeroPage(&mut self){}// Store Accumulator: Length 2
    fn StaZeroPageX(&mut self){}  // Store Accumulator: Length 2
    fn StaAbsolute(&mut self){} // Store Accumulator: Length 3
    fn StaAbsoluteX(&mut self){} // Store Accumulator: Length 3
    fn StaAbsoluteY(&mut self){} // Store Accumulator: Length 3
    fn StaIndirectX(&mut self){} // Store Accumulator: Length 2
    fn StaIndirectY(&mut self){}  // Store Accumulator: Length 2

    fn StxZeroPage(&mut self){} // Store X register: Length 2
    fn StxZeroPage_y(&mut self){}// Store X register: Length 2
    fn StxAbsolute(&mut self){} // Store X register: Length 3
    fn StyZeroPage(&mut self){}// Store Y register: Length 2
    fn StyZeroPageX(&mut self){} // Store Y register: Length 2
    fn StyAbsolute(&mut self){}// Store Y register: Length 3

    fn Tax(&mut self){} // Transfer A to X
    fn TAY(&mut self){}// Tranfer A to Y
    fn Txa(&mut self){}  // Transfer X to A
    fn page_x(&mut self){} // Transfer Y to A*/
}

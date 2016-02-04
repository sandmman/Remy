fn LDA_ZeroPage(operand){
    """Load Accumulator With Memory"""
    SET_NEGATIVE(operand)
    SET_ZERO(operand)
    emulator.cpu.accumulator = operand
}
fn LDY_Absolute(){
    """Load Y Index With Memory"""
    SET_NEGATIVE(operand)
    SET_ZERO(operand)
    emulator.cpu.y_index = operand
}
fn LDX_Immediate(){
    """Load X Index With Memory"""
    SET_NEGATIVE(operand)
    SET_ZERO(operand)
    emulator.cpu.x_index = operand
}
fn LDA_Immediate(){
    SET_NEGATIVE(operand)
    SET_ZERO(operand)
    emulator.cpu.accumulator = operand
}
fn ADC(A: i8, x i8){
    //Adds A plus another number, plus the Carry flag, and stores the result in A
    A = A + x;
}
fn BNE(){}
fn JMP(){}
fn INX(){}
fn ASL(){}
fn BIT(){}
fn SEI(){}
fn CLD(){}
fn ORA_ZeroPage(){}
fn ORA_Immediate(){}
fn BPL(){}
fn CLC(){}
fn STA_Absolute(){}
fn JSR(){}
fn ROL_Accumulator(){}
fn SEC(){}
fn AND_Absolute(){}
fn RTI(){}
fn EOR_ZeroPage(){}
fn PHA(){}
fn LSR_Accumulator(){}``
fn AND_Immediate(){}
fn TAY(){}
fn TAX(){}
fn LDX_Absolute(){}
fn TXA(){}
fn TXS(){}
fn STA_AbsoluteY(){}
fn LDA_Absolute(){}
fn LDY_Immediate(){}
fn LDA_IndirectY(){}
fn LDA_AbsoluteX(){}
fn LDX_AbsoluteY(){}
fn LDA_AbsoluteY(){}
fn CMP(){}
fn DEC_ZeroPage(){}
fn BCS(){}
fn DEX(){}
fn DEC_Absolute(){}
fn JMP_Indirect(){}

fn ADC_Immediate(){}
fn ROR_AbsoluteX(){}
fn STA_ZeroPage(){}
fn STX_ZeroPage(){}
fn STY_Absolute(){}
fn CPX(){}
fn BCC(){}
fn STA_IndirectY(){}
fn TYA(){}
fn STA_AbsoluteX(){}
fn DEY(){}
fn CPY(){}
fn INY(){}
fn INC(){}
fn INC_ZeroPage(){}
fn RTS(){}
fn ADC_ZeroPage(){}
fn PLA(){}
fn BEQ(){}
fn SED(){}
fn SBC_AbsoluteY(){}
    pass

fn SET_NEGATIVE(operand){
    if 0x00 < operand < 0x7F{
        emulator.cpu.status_registers[NEGATIVE_FLAG] = 0
    }
    else{
        emulator.cpu.status_registers[NEGATIVE_FLAG] = 1
    }
}
fn SET_ZERO(operand){
    if 0x00 == operand{
        emulator.cpu.status_registers[ZERO_FLAG] = 1
    }
    else{
        emulator.cpu.status_registers[ZERO_FLAG] = 0
    }
}

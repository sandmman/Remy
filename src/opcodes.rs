enum_from_primitive! {
    #[repr(u8)]
    pub enum Opcode {
        AdcImmediate  = 0x69, // ADd with Carry: Length 2
        AdcZeroPage  = 0x65, // ADd with Carry: Length 2
        AdcZeroPageX= 0x75, // ADd with Carry: Length 2
        AdcAbsolute   = 0x6D, // ADd with Carry: Length 3
        AdcAbsoluteX = 0x7D, // ADd with Carry: Length 3
        AdcAbsoluteY= 0x79, // ADd with Carry: Length 3
        AdcIndirectX= 0x61, // ADd with Carry: Length 2
        AdcIndirectY = 0x71, // ADd with Carry: Length 2

        AndImmediate  = 0x29, // bitwise and with accumultor: Length 2
        AndZeroPage  = 0x25, // bitwise and with accumultor: Length 2
        AndZeroPageX= 0x35, // bitwise and with accumultor: Length 2
        AndAbsolute   = 0x2D, // bitwise and with accumultor: Length 3
        AndAbsoluteX  = 0x3D, // bitwise and with accumultor: Length 3
        AndAbsoluteY  = 0x39, // bitwise and with accumultor: Length 3
        AndIndirectX= 0x21, // bitwise and with accumultor: Length 2
        AndIndirectY = 0x31, // bitwise and with accumultor: Length 2

        AslAccumulator  = 0x0A, // Arithmetic Shift Left: Length 1
        AslZeroPage     = 0x06, // Arithmetic Shift Left: Length 2
        AslZeroPageX    = 0x16, // Arithmetic Shift Left: Length 2
        AslAbsolute     = 0x0E, // Arithmetic Shift Left: Length 3
        AslAbsoluteX    = 0x1E, // Arithmetic Shift Left: Length 3

        BitZeroPage  = 0x24, // test bits
        BitAbsolute   = 0x2C, // test bits
        // Branch Instructions
        Bcc             = 0x90, // Beanch on Carry Clear
        Bcs             = 0xB0, // Branch on Carry Set
        Beq             = 0xF0, // Branch on Equal
        Bmi             = 0x30, // Branch on Minus
        Bne             = 0xD0, // Branch on Not Equal
        Bpl             = 0x10, // Branch of Plus
        Bvc             = 0x50, // Branch on overflow clear
        Bvs             = 0x70, // Branch on overflow set
        Brk             = 0x00, // Break
        // Set / Clear Instructions
        Clc             = 0x18, // Clear Carry
        Cld             = 0xD8, // Clear decimal
        Cli             = 0x58, // Clear Interrupt
        Clv             = 0xB8, // Clear Overflow
        Sec             = 0x38, // Set Carry
        Sed             = 0xF8, // Set decimal
        Sei             = 0x78, // Set Interrupt

        CmpImmediate  = 0xC9, // Compare Accumulator: Length 2
        CmpZeroPage  = 0xC5, // Compare Accumulator: Length 2
        CmpZeroPageX= 0xD5, // Compare Accumulator: Length 2
        CmpAbsolute   = 0xCD, // Compare Accumulator: Length 3
        CmpAbsoluteX  = 0xDD, // Compare Accumulator: Length 3
        CmpAbsoluteY  = 0xD9, // Compare Accumulator: Length 3
        CmpIndirectX= 0xC1, // Compare Accumulator: Length 2
        CmpIndirectY = 0xD1, // Compare Accumulator: Length 2

        CpxImmediate  = 0xE0, // Compare X Register: Length 2
        CpxZeroPage  = 0xE4, // Compare X Register: Length 2
        CpxAbsolute   = 0xEC, // Compare X Register: Length 3

        CpyImmediate  = 0xC0, // Compare Y Register: Length 2
        CpyZeroPage  = 0xC4, // Compare Y Register: Length 2
        CpyAbsolute   = 0xCC, // Compare Y Register: Length 3

        DecZeroPage  = 0xC6, // decrement Memory: Length 2
        DecZeroPageX= 0xD6, // decrement Memory: Length 2
        DecAbsolute   = 0xCE, // decrement Memory: Length 3
        DecAbsoluteX  = 0xDE, // decrement Memory: Length 3
        //Register Instruction
        Tax             = 0xAA, // Transfer A to X
        Txa             = 0x8A, // Transfer X to A
        Dex             = 0xCA, // decrement X
        Inx             = 0xE8, // increment X
        Tay             = 0xA8, // Transfer A to Y
        Tya             = 0x98, // Transfer Y to A
        Dey             = 0x88, // decrement Y
        Iny             = 0xC8, // increment Y
        //stack Instructions
        Tsx             = 0xBA, // Transfer stack ptr to X
        Txs             = 0x9A, // Transfer X to stack ptr
        Pha             = 0x48, // Push Accumulator
        Php             = 0x08, // Push Processor status
        Pla             = 0x68, // Pull Acumulator
        Plp             = 0x28, // Pull Process status

        EorImmediate  = 0x49, // bitwise exlusive OR: Length 2
        EorZeroPage  = 0x45, // bitwise exlusive OR: Length 2
        EorZeroPageX= 0x55, // bitwise exlusive OR: Length 2
        EorAbsolute   = 0x4D, // bitwise exlusive OR: Length 3
        EorAbsoluteX  = 0x5D, // bitwise exlusive OR: Length 3
        EorAbsoluteY  = 0x59, // bitwise exlusive OR: Length 3
        EorIndirectX= 0x41, // bitwise exlusive OR: Length 2
        EorIndirectY = 0x51, // bitwise exlusive OR: Length 2

        IncZeroPage  = 0xE6, // increment Memory: Length 2
        IncZeroPageX= 0xF6, // increment Memory: Length 2
        IncAbsolute   = 0xEE, // increment Memory: Length 3
        IncAbsoluteX  = 0xFE, // increment Memory: Length 3



        JmpAbsolute   = 0x4c, // Jump Absolute: Length 3
        JmpIndirect    = 0x6c, // Jump Indirect: Length 3
        JsrAbsolute   = 0x20, // Jump to SubRoutine

        LdaImmediate  = 0xA9, // Load Accumulator: Length 2
        LdaZeroPage  = 0xA5, // Load Accumulator: Length 2
        LdaZeroPageX= 0xB5, // Load Accumulator: Length 2
        LdaAbsolute   = 0xAD, // Load Accumulator: Length 3
        LdaAbsoluteX  = 0xBD, // Load Accumulator: Length 3
        LdaAbsoluteY  = 0xB9, // Load Accumulator: Length 3
        LdaIndirectX= 0xA1, // Load Accumulator: Length 2
        LdaIndirectY = 0xB1, // Load Accumulator: Length 2

        LdxImmediate  = 0xA2, // Load X Register: Length 2
        LdxZeroPage  = 0xA6, // Load X Register: Length 2
        LdxZeroPageX= 0xB6, // Load X Register: Length 2
        LdxAbsolute   = 0xAE, // Load X Register: Length 3
        LdxAbsoluteX  = 0xBE, // Load X Register: Length 3

        LdyImmediate  = 0xA0, // Load Y Register: Length 2
        LdyZeroPage   = 0xA4, // Load Y Register: Length 2
        LdyZeroPageX  = 0xB4, // Load Y Register: Length 2
        LdyAbsolute   = 0xAC, // Load Y Register: Length 3
        LdyAbsoluteX  = 0xBC, // Load Y Register: Length 3

        LsrAccumulator = 0x4A, // Logical Shift Right: Length 1
        LsrZeroPage  = 0x46, // Logical Shift Right: Length 2
        LsrZeroPageX= 0x56, // Logical Shift Right: Length 2
        LSRAbsolute   = 0x4E, // Logical Shift Right: Length 3
        LSRAbsoluteX  = 0x5E, // Logical Shift Right: Length 3

        NopImplied     = 0xea, // No operation

        OraImmediate  = 0x09, // bitwise OR with Accumulator: Length 2
        OraZeroPage  = 0x05, // bitwise OR with Accumulator: Length 2
        OraZeroPageX= 0x15, // bitwise OR with Accumulator: Length 2
        OraAbsolute   = 0x0D, // bitwise OR with Accumulator: Length 3
        OraAbsoluteX  = 0x1D, // bitwise OR with Accumulator: Length 3
        OraAbsoluteY  = 0x19, // bitwise OR with Accumulator: Length 3
        OraIndirectX= 0x01, // bitwise OR with Accumulator: Length 2
        OraIndirectY = 0x11, // bitwise OR with Accumulator: Length 2

        //Rotate Instructions
        RolAccumulator = 0x2A, // Rotate Left: Length 1
        RolZeroPage  = 0x26, // Rotate Left: Length 2
        RolZeroPageX= 0x36, // Rotate Left: Length 2
        RolAbsolute   = 0x2E, // Rotate Left: Length 3
        RolAbsoluteX  = 0x3E, // Rotate Left: Length 3
        Roraccumulator = 0x6A, // Rotate Right: Length 1
        RorZeroPage  = 0x66, // Rotate Right: Length 2
        RorZeroPageX= 0x76, // Rotate Right: Length 2
        RorAbsolute   = 0x6E, // Rotate Right: Length 3
        RorAbsoluteX  = 0x7E, // Rotate Right: Length 3

        Rti = 0x40, // Return from Interrupt
        Rts = 0x60, // Return from Subroutine

        SbcImmediate  = 0xE9, // Subtract with Carry: Length 2
        SbcZeroPage   = 0xE5, // Subtract with Carry: Length 2
        SbcZeroPageX = 0xF5, // Subtract with Carry: Length 2
        SbcAbsolute   = 0xED, // Subtract with Carry: Length 3
        SbcAbsoluteX  = 0xFD, // Subtract with Carry: Length 3
        SbcAbsoluteY  = 0xF9, // Subtract with Carry: Length 3
        SbcIndirectX  = 0xE1, // Subtract with Carry: Length 2
        SbcIndirectY  = 0xF1, // Subtract with Carry: Length 2

        StaZeroPage   = 0x85, // Store Accumulator: Length 2
        StaZeroPageX = 0x95, // Store Accumulator: Length 2
        StaAbsolute   = 0x8D, // Store Accumulator: Length 3
        StaAbsoluteX  = 0x9D, // Store Accumulator: Length 3
        StaAbsoluteY  = 0x99, // Store Accumulator: Length 3
        StaIndirectX  = 0x81, // Store Accumulator: Length 2
        StaIndirectY  = 0x91, // Store Accumulator: Length 2

        StxZeroPage   = 0x86, // Store X register: Length 2
        StxZeroPageX= 0x96, // Store X register: Length 2
        StxAbsolute   = 0x8E, // Store X register: Length 3
        StyZeroPage   = 0x84, // Store Y register: Length 2
        StyZeroPageX = 0x94, // Store Y register: Length 2
        StyAbsolute   = 0x8C, // Store Y register: Length 3
    }
}

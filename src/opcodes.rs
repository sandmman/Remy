enum_from_primitive! {
    #[repr(u8)]
    pub enum Opcode {
        adc_immediate   = 0x69, // ADd with Carry: Length 2
        adc_zero_page   = 0x65, // ADd with Carry: Length 2
        adc_zero_page_x = 0x75, // ADd with Carry: Length 2
        adc_absolute    = 0x6D, // ADd with Carry: Length 3
        adc_absolute_x  = 0x7D, // ADd with Carry: Length 3
        adc_absolute_y  = 0x79, // ADd with Carry: Length 3
        adc_indirect_x  = 0x61, // ADd with Carry: Length 2
        adc_indirect_y  = 0x71, // ADd with Carry: Length 2

        and_immediate   = 0x29, // bitwise and with accumultor: Length 2
        and_zero_page   = 0x25, // bitwise and with accumultor: Length 2
        and_zero_page_x = 0x35, // bitwise and with accumultor: Length 2
        and_absolute    = 0x2D, // bitwise and with accumultor: Length 3
        and_absolute_x  = 0x3D, // bitwise and with accumultor: Length 3
        and_absolute_y  = 0x39, // bitwise and with accumultor: Length 3
        and_indirect_x  = 0x21, // bitwise and with accumultor: Length 2
        and_indirect_y  = 0x31, // bitwise and with accumultor: Length 2

        asl_accumulator = 0x0A, // Arithmetic Shift Left: Length 1
        asl_zero_page   = 0x06, // Arithmetic Shift Left: Length 2
        asl_zero_page_x = 0x16, // Arithmetic Shift Left: Length 2
        asl_absolute    = 0x0E, // Arithmetic Shift Left: Length 3
        asl_absolute_x  = 0x1E, // Arithmetic Shift Left: Length 3

        bit_zero_page   = 0x24, // test bits
        bit_absolute    = 0x2C, // test bits
        // Branch Instructions
        bcc             = 0x90, // Beanch on Carry Clear
        bcs             = 0xB0, // Branch on Carry Set
        beq             = 0xF0, // Branch on Equal
        bmi             = 0x30, // Branch on Minus
        bne             = 0xD0, // Branch on Not Equal
        bpl             = 0x10, // Branch of Plus
        bvc             = 0x50, // Branch on overflow clear
        bvs             = 0x70, // Branch on overflow set
        brk             = 0x00, // Break
        // Set / Clear Instructions
        clc             = 0x18, // Clear Carry
        cld             = 0xD8, // Clear decimal
        cli             = 0x58, // Clear Interrupt
        clv             = 0xB8, // Clear Overflow
        sec             = 0x38, // Set Carry
        sed             = 0xF8, // Set decimal
        sei             = 0x78, // Set Interrupt

        cmp_immediate   = 0xC9, // Compare Accumulator: Length 2
        cmp_zero_page   = 0xC5, // Compare Accumulator: Length 2
        cmp_zero_page_x = 0xD5, // Compare Accumulator: Length 2
        cmp_absolute    = 0xCD, // Compare Accumulator: Length 3
        cmp_absolute_x  = 0xDD, // Compare Accumulator: Length 3
        cmp_absolute_y  = 0xD9, // Compare Accumulator: Length 3
        cmp_indirect_x  = 0xC1, // Compare Accumulator: Length 2
        cmp_indirect_y  = 0xD1, // Compare Accumulator: Length 2

        cpx_immediate   = 0xE0, // Compare X Register: Length 2
        cpx_zero_page   = 0xE4, // Compare X Register: Length 2
        cpx_absolute    = 0xEC, // Compare X Register: Length 3

        cpy_immediate   = 0xC0, // Compare Y Register: Length 2
        cpy_zero_page   = 0xC4, // Compare Y Register: Length 2
        cpy_absolute    = 0xCC, // Compare Y Register: Length 3

        dec_zero_page   = 0xC6, // decrement Memory: Length 2
        dec_zero_page_x = 0xD6, // decrement Memory: Length 2
        dec_absolute    = 0xCE, // decrement Memory: Length 3
        dec_absolute_x  = 0xDE, // decrement Memory: Length 3
        //Register Instruction
        tax             = 0xAA, // Transfer A to X
        txa             = 0x8A, // Transfer X to A
        dex             = 0xCA, // decrement X
        inx             = 0xE8, // increment X
        tay             = 0xA8, // Transfer A to Y
        tya             = 0x98, // Transfer Y to A
        dey             = 0x88, // decrement Y
        iny             = 0xC8, // increment Y
        //stack Instructions
        tsx             = 0xBA, // Transfer stack ptr to X
        txs             = 0x9A, // Transfer X to stack ptr
        pha             = 0x48, // Push Accumulator
        php             = 0x08, // Push Processor status
        pla             = 0x68, // Pull Acumulator
        plp             = 0x28, // Pull Process status

        eor_immediate   = 0x49, // bitwise exlusive OR: Length 2
        eor_zero_page   = 0x45, // bitwise exlusive OR: Length 2
        eor_zero_page_x = 0x55, // bitwise exlusive OR: Length 2
        eor_absolute    = 0x4D, // bitwise exlusive OR: Length 3
        eor_absolute_x  = 0x5D, // bitwise exlusive OR: Length 3
        eor_absolute_y  = 0x59, // bitwise exlusive OR: Length 3
        eor_indirect_x  = 0x41, // bitwise exlusive OR: Length 2
        eor_indirect_y  = 0x51, // bitwise exlusive OR: Length 2

        inc_zero_page   = 0xE6, // increment Memory: Length 2
        inc_zero_page_x = 0xF6, // increment Memory: Length 2
        inc_absolute    = 0xEE, // increment Memory: Length 3
        inc_absolute_x  = 0xFE, // increment Memory: Length 3



        jmp_absolute    = 0x4c, // Jump Absolute: Length 3
        jmp_indirect    = 0x6c, // Jump Indirect: Length 3
        jsr_absolute    = 0x20, // Jump to SubRoutine

        lda_immediate   = 0xA9, // Load Accumulator: Length 2
        lda_zero_page   = 0xA5, // Load Accumulator: Length 2
        lda_zero_page_x = 0xB5, // Load Accumulator: Length 2
        lda_absolute    = 0xAD, // Load Accumulator: Length 3
        lda_absolute_x  = 0xBD, // Load Accumulator: Length 3
        lda_absolute_y  = 0xB9, // Load Accumulator: Length 3
        lda_indirect_x  = 0xA1, // Load Accumulator: Length 2
        lda_indirect_y  = 0xB1, // Load Accumulator: Length 2

        ldx_immediate   = 0xA2, // Load X Register: Length 2
        ldx_zero_page   = 0xA6, // Load X Register: Length 2
        ldx_zero_page_x = 0xB6, // Load X Register: Length 2
        ldx_absolute    = 0xAE, // Load X Register: Length 3
        ldx_absolute_x  = 0xBE, // Load X Register: Length 3

        ldy_immediate   = 0xA0, // Load Y Register: Length 2
        ldy_zero_page   = 0xA4, // Load Y Register: Length 2
        ldy_zero_page_x = 0xB4, // Load Y Register: Length 2
        ldy_absolute    = 0xAC, // Load Y Register: Length 3
        ldy_absolute_x  = 0xBC, // Load Y Register: Length 3

        lsr_accumulator = 0x4A, // Logical Shift Right: Length 1
        lsr_zero_page   = 0x46, // Logical Shift Right: Length 2
        lsr_zero_page_x = 0x56, // Logical Shift Right: Length 2
        lsr_absolute    = 0x4E, // Logical Shift Right: Length 3
        lsr_absolute_x  = 0x5E, // Logical Shift Right: Length 3

        nop_implied     = 0xea, // No operation

        ora_immediate   = 0x09, // bitwise OR with Accumulator: Length 2
        ora_zero_page   = 0x05, // bitwise OR with Accumulator: Length 2
        ora_zero_page_x = 0x15, // bitwise OR with Accumulator: Length 2
        ora_absolute    = 0x0D, // bitwise OR with Accumulator: Length 3
        ora_absolute_x  = 0x1D, // bitwise OR with Accumulator: Length 3
        ora_absolute_y  = 0x19, // bitwise OR with Accumulator: Length 3
        ora_indirect_x  = 0x01, // bitwise OR with Accumulator: Length 2
        ora_indirect_y  = 0x11, // bitwise OR with Accumulator: Length 2

        //Rotate Instructions
        rol_accumulator = 0x2A, // Rotate Left: Length 1
        rol_zero_page   = 0x26, // Rotate Left: Length 2
        rol_zero_page_x = 0x36, // Rotate Left: Length 2
        rol_absolute    = 0x2E, // Rotate Left: Length 3
        rol_absolute_x  = 0x3E, // Rotate Left: Length 3
        ror_accumulator = 0x6A, // Rotate Right: Length 1
        ror_zero_page   = 0x66, // Rotate Right: Length 2
        ror_zero_page_x = 0x76, // Rotate Right: Length 2
        ror_absolute    = 0x6E, // Rotate Right: Length 3
        ror_absolute_x  = 0x7E, // Rotate Right: Length 3

        rti = 0x40, // Return from Interrupt
        rts = 0x60, // Return from Subroutine

        sbc_immediate   = 0xE9, // Subtract with Carry: Length 2
        sbc_zero_page   = 0xE5, // Subtract with Carry: Length 2
        sbc_zero_page_x = 0xF5, // Subtract with Carry: Length 2
        sbc_absolute    = 0xED, // Subtract with Carry: Length 3
        sbc_absolute_x  = 0xFD, // Subtract with Carry: Length 3
        sbc_absolute_y  = 0xF9, // Subtract with Carry: Length 3
        sbc_indirect_x  = 0xE1, // Subtract with Carry: Length 2
        sbc_indirect_y  = 0xF1, // Subtract with Carry: Length 2

        sta_zero_page   = 0x85, // Store Accumulator: Length 2
        sta_zero_page_x = 0x95, // Store Accumulator: Length 2
        sta_absolute    = 0x8D, // Store Accumulator: Length 3
        sta_absolute_x  = 0x9D, // Store Accumulator: Length 3
        sta_absolute_y  = 0x99, // Store Accumulator: Length 3
        sta_indirect_x  = 0x81, // Store Accumulator: Length 2
        sta_indirect_y  = 0x91, // Store Accumulator: Length 2

        stx_zero_page   = 0x86, // Store X register: Length 2
        stx_zero_page_x = 0x96, // Store X register: Length 2
        stx_absolute    = 0x8E, // Store X register: Length 3
        sty_zero_page   = 0x84, // Store Y register: Length 2
        sty_zero_page_x = 0x94, // Store Y register: Length 2
        sty_absolute    = 0x8C, // Store Y register: Length 3
    }
}

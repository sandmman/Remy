# Remy
Rust NES Emulator

Following [Nes Docs](http://nesdev.com/NESDoc.pdf)
## To Do General
- input/output
  - Learn how to deal with graphics  
  - Learn how to read in a .nes file

## CPU Emulation
- 8 bit 6502 Processor
- Nes Little-Endian

## PPU Emulation
- PPU has its own memory  
  - VRAM: 64 KB memory  - 16 KB of physical RAM
  - SPR-RAM (Sprite RAM) - 256 byte area
- PPU Registers
  - Located at $2000 - $2007 and DMA register $4014

## Memory
- 64 KB of Memory 0x0000 - 0xFFFF
  - $0000-$00FF -- Zero Page is used by certain addressing modes to allow quicker execution
- Memory locations $0000-$07FF are mirrored three times at $0800-$1FFF
- The memory mapped I/O registers are located at $2000-$401F
-  $2000-$2007 are mirrored every 8 bytes
in the region $2008-$3FFF
- $8000-0xFFFF -- the addresses allocated to cartridge PRG-ROM

## Registers (6)
- Program Counter (PC): 16-bit register which holds the address of the next instruction to be executed
- Stack Pointer (SP): 8-bit register which serves as an offset from $0100.
- Accumulator (A): 8-bit register which stores the results of arithmetic and logic operations
- Index Register X (X): 8-bit register typically used as a counter or an offset for certain addressing modes
- Index Register Y (Y): 8-bit register typically used as a counter or an offset for certain addressing modes. Y register cannot affect the stack pointer
- Processor Status (P) -

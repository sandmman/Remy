use instructions;
use opcodes::Opcode;
use byteorder::LittleEndian;

const RAM_SIZE: usize = 2048;
const MEM: usize = 64 * 1024;
const staCK_SIZE: usize = 256;
const MAX_MEM_ADDR: u16 = 0xffff;
/*
Address	Size	Description
$0000	$800	2KB of work RAM
$0800	$800	Mirror of $000-$7FF
$1000	$800	Mirror of $000-$7FF
$1800	$800	Mirror of $000-$7FF
$2000	8	PPU Ctrl Registers
$2008	$1FF8	*Mirror of $2000-$2007
$4000	$20	Registers (Mostly APU)
$4020	$1FDF	Cartridge Expansion ROM
$6000	$2000	SRAM
$8000	$4000	PRG-ROM
$C000	$4000	PRG-ROM
*/
pub struct Memory {
    pub ram: Box<[u8; RAM_SIZE]>, // 0x100 to 0x1FF is the stack
                                  // 0x200-0x800 is General Ram
                                  // 0x801 to 0x2000 mirrors 0x00 -> 0x7ff
    pub mem: Box<[u8; MEM]>,
    pub rom: Vec<u8>,
}
impl Memory {
    pub fn new(rom_buf: Vec<u8>) -> Memory{
        Memory {
            ram: Box::new([0; RAM_SIZE]),
            mem: Box::new([0; MEM]),
            rom: rom_buf
        }
    }
    pub fn read_u8(&self, addr: u16) -> u8{
        if addr > MAX_MEM_ADDR{
            panic!("Address out of bounds: {:#x}", addr)
        }
        else{
            self.mem[(addr as usize)]
        }
    }
    pub fn read_u16(&self, addr: u16) -> u16 {
        if addr+2 > MAX_MEM_ADDR{
            panic!("Address out of bounds: {:#x}", addr)
        }
        else{
            let most_sig = self.mem[((addr+1) as usize)];
            let least_sig = self.mem[(addr as usize)];
            ((most_sig as u16) <<8) + (least_sig as u16)
        }

    }
    pub fn read_rom_u8(&self, addr: u16) -> u8{
        if addr > MAX_MEM_ADDR{
            panic!("Address out of bounds: {:#x}", addr)
        }
        else{
            self.rom[(addr as usize)]
        }
    }
    pub fn read_rom_u16(&self, addr: u16) -> u16{
        if addr+2 > MAX_MEM_ADDR{
            panic!("Address out of bounds: {:#x}", addr)
        }
        else{
            let most_sig = self.mem[((addr+1) as usize)];
            let least_sig = self.mem[(addr as usize)];
            ((most_sig as u16) <<8) + (least_sig as u16)
        }
    }
    pub fn write_u8(&mut self, addr: u16, obj: u8){
        self.mem[(addr as usize)] = obj;
    }
    pub fn write_u16(&mut self, addr: u16, obj: u16) {
        let most_sig:  u8 = (addr >> 8) as u8;
        let least_sig: u8 = obj as u8;
        self.mem[(addr as usize)] = least_sig;
        self.mem[((addr+1) as usize)] = most_sig;

    }
}

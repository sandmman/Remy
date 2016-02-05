const RAM_SIZE: usize = 2048;
const MEM: usize = 64 * 1024;
const STACK_SIZE: usize = 256;
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
}
impl Memory {
    pub fn read_word(&mut self, addr: u16) -> u8{
        if addr > MAX_MEM_ADDR{
            println!("{}, {}",MAX_MEM_ADDR, addr);
            panic!("Address out of bounds: {:#x}", addr)
        }
        else{
            self.mem[(addr as usize)]
        }
    }
    pub fn write_word(&mut self, addr: u16, obj: u8){
        self.mem[(addr as usize)] = obj;
    }
}
impl Default for Memory {
    fn default() -> Memory {
        Memory {
            ram: Box::new([0; RAM_SIZE]),
            mem: Box::new([0; MEM])
        }
    }
}

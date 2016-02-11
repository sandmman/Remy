extern crate byteorder;
extern crate num;
#[macro_use]
extern crate enum_primitive;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

mod nes;
mod cpu;
mod ppu;
mod memory;
mod opcodes;
mod instructions;

fn main() {
    let rom_file_name = env::args().nth(1).unwrap();
    let rom = read_bin(rom_file_name);                  // make it into an unmutable binding
    // NES file format header information (first 16 bytes)
    // Should contain "NES"
    let file_type = rom[0].to_string() + &rom[1].to_string() + &rom[2].to_string();
    // Should contain $1A
    let file_type2 = rom[3].to_string();
    // staRting Byte 4: Length 1 - # of 16 KB PRG-ROM banks
    let prg = rom[4].to_string();
    // staRting Byte 5: Length 1 - # of 8 KB CHR-ROM / VROM banks
    let chr = rom[5].to_string();
    // staRting Byte 6: Length 1 - Rom control Byte 1
    let rom_cntrl_byte_1 = rom[6];
    // staRting Byte 7: Length 1 - Rom control Byte 1
    let rom_cntrl_byte_2 = rom[7];
    // staRting Byte 8: Length 1 - # of 8 KB RAM banks
    let num_ram_banks = rom[8].to_string();
    // staRting Byte 9: Length 7 - Reserved
    let reserved = rom[9].to_string() + &rom[10].to_string() + &rom[11].to_string() + &rom[12].to_string() + &rom[13].to_string() + &rom[14].to_string() + &rom[15].to_string();
    // Registers


    println!("File Type:                      {:?} {:?}",file_type,file_type2);
    println!("# of 16 KB PRG-ROM banks:       {:?}",prg);
    println!("# of 8 KB CHR-ROM / VROM banks: {:?}",chr);
    println!("Rom_cntrl_byte_1:               {:b}",rom_cntrl_byte_1);
    println!("Rom_cntrl_byte_2:               {:b}",rom_cntrl_byte_2);
    println!("# of 8 KB RAM banks:            {:?}",num_ram_banks);
    println!("Reserved:                       {:?}",reserved);

    // Create a new instance of the NES
    let mut nes = nes::NES::new(rom);
    nes.power_on_reset();

    loop{
        nes.run();
    }
}
fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();        // open file
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();          // read the file into a buffer
    file_buf
}

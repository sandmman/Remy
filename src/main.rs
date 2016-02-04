use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::default::Default;

mod cpu;
mod ppu;

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();        // open file
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();          // read the file into a buffer
    file_buf
}

fn main() {
    let rom_file_name = env::args().nth(1).unwrap();
    let rom = read_bin(rom_file_name);                  // make it into an unmutable binding
    // NES file format header information (first 16 bytes)
    // Should contain "NES"
    let file_type = rom[0].to_string() + &rom[1].to_string() + &rom[2].to_string();
    // Should contain $1A
    let file_type2 = rom[3].to_string();
    // Starting Byte 4: Length 1 - # of 16 KB PRG-ROM banks
    let prg = rom[4].to_string();
    // Starting Byte 5: Length 1 - # of 8 KB CHR-ROM / VROM banks
    let chr = rom[5].to_string();
    // Starting Byte 6: Length 1 - Rom control Byte 1
    let rom_cntrl_byte_1 = rom[6].to_string();
    // Starting Byte 7: Length 1 - Rom control Byte 1
    let rom_cntrl_byte_2 = rom[7].to_string();
    // Starting Byte 8: Length 1 - # of 8 KB RAM banks
    let num_ram_banks = rom[8].to_string();
    // Starting Byte 9: Length 7 - Reserved
    let reserved = rom[9].to_string() + &rom[10].to_string() + &rom[11].to_string() + &rom[12].to_string() + &rom[13].to_string() + &rom[14].to_string() + &rom[15].to_string();
    // Registers


    println!("File Type {:?}",file_type);
    println!("File Type2 {:?}",file_type2);
    println!("prg {:?}",prg);
    println!("chr {:?}",chr);
    println!("rom_cntrl_byte_1 {:?}",rom_cntrl_byte_1);
    println!("rom_cntrl_byte_2  {:?}",rom_cntrl_byte_2);
    println!("num_ram_banks {:?}",num_ram_banks);
    println!("reserved {:?}",reserved);

    // Game Instructions begin at bytes 17 after 16-byte header
    let instruction = rom[16].to_string() + &rom[17].to_string();
    println!("instruction example {}",instruction);

    // Create a new instance of the CPu
    let mut cpu = cpu::Cpu::new();
    println!("CPU {:#?}",&cpu);
    //cpu.read_instruction()


}

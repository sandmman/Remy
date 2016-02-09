use super::cpu;
use super::memory;


pub struct NES {
    cpu: cpu::Cpu,
}
impl NES {
    pub fn new(rom: Vec<u8>) -> NES{
        let memory = memory::Memory::new(rom);
        NES {
            cpu: cpu::Cpu::new(memory)
        }
    }
    pub fn power_on_reset(&mut self){
        self.cpu.power_on_reset();
    }
    pub fn run(&mut self){
        self.cpu.run();
    }
}

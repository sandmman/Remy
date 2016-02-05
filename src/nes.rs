use super::cpu;

#[derive(Default)]
pub struct NES {
    cpu: cpu::Cpu,
}
impl NES {
    pub fn power_on_reset(&mut self){
        self.cpu.power_on_reset();
    }
    pub fn run(&mut self,rom: &Vec<u8>){
        self.cpu.run(rom);
    }
}

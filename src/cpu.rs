use std::fs::File;
use std::io::Read;

const MAX_REG: usize = 16;
const MAX_STACK: usize = 16;
const MAX_MEMORY: usize = 4096;

pub trait Cpu {
    fn new() -> Self;
    fn load_rom(&mut self, rom: &String);
}

pub struct Chip8 {
    pub v:      [u8; MAX_REG],
    pub i:      u16,
    pub pc:     u16,
    pub sp:     u16,
    pub mem:    [u8; MAX_MEMORY],
    pub stack:  [u16; MAX_STACK],
}

impl Cpu for Chip8 {
    fn new() -> Self {
        Chip8 { v: [0;MAX_REG], i: 0, pc: 0x200, sp: 0, mem: [0;MAX_MEMORY], stack: [0;MAX_STACK] }
    }

    fn load_rom(&mut self, rom: &String) {
        let mut romfile: File = File::open(rom).unwrap();
        romfile.read(&mut self.mem[512..MAX_MEMORY]).unwrap();
    }
}

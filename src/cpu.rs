use std::fs::File;
use std::io::Read;

const MAX_REG: usize = 16;
const MAX_STACK: usize = 16;
const MAX_MEMORY: usize = 4096;

pub enum Opcode {
    Clear,
    Return,
    JumpAddr,
    Call,
    SkipEqualVxkk,
    SkipNotEqualVxkk,
    SkipEqualVxVy,
    LoadVxkk,
    AddVxkk,
    LoadVxVy,
    Or,
    And,
    Xor,
    AddVxVy,
    Subtract,
    RightShift,
    SubtractNotBorrow,
    LeftShift,
    SkipNotEqualVxVy,
    LoadI,
    JumpAddrV0,
    Random,
    Draw,
    SkipIfVxPressed,
    SkipIfVxNotPressed,
    LoadVxDelayTimer,
    LoadPressedKeyVx,
    LoadDelayTimerVx,
    LoadSoundTimerVx,
    AddIVx,
    LoadISpritePositionVx,
    LoadIBCDVx,
    LoadIVRegisters,
    LoadVRegistersI,
}

pub trait Cpu {
    fn new() -> Self;
    fn load_rom(&mut self, rom: &String);
    fn advance_pc(&mut self);
    fn process_opcode(opcode: &u16) -> Result<Opcode, &str>;
}

pub struct Chip8 {
    pub v:      [u8; MAX_REG],
    pub i:      usize,
    pub pc:     usize,
    pub sp:     usize,
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

    fn advance_pc(&mut self) {
        let opcode: u16 = (self.mem[self.pc] as u16 * 256) + self.mem[self.pc + 1] as u16;
        self.pc = if self.pc + 2 >= MAX_MEMORY { 0 } else { self.pc + 2 };
    }

    fn process_opcode(opcode: &u16) -> Result<Opcode, &str> {

    }
}

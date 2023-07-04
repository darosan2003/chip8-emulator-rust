extern crate rand;

use rand::Rng;

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
    fn process_opcode(&mut self, opcode: &u16) -> Result<Opcode, &str>;
    fn do_instruction(&mut self, instruction: &Opcode, opcode: &u16);
}

pub struct Chip8 {
    pub v:      [u8; MAX_REG],
    pub dt:     u8,
    pub st:     u8,
    pub i:      usize,
    pub pc:     usize,
    pub sp:     usize,
    pub mem:    [u8; MAX_MEMORY],
    pub stack:  [u16; MAX_STACK],
}

impl Cpu for Chip8 {
    fn new() -> Self {
        Chip8 { v: [0;MAX_REG], dt: 0, st: 0, i: 0, pc: 0x200, sp: 0, mem: [0;MAX_MEMORY], stack: [0;MAX_STACK] }
    }

    fn load_rom(&mut self, rom: &String) {
        let mut romfile: File = File::open(rom).unwrap();
        romfile.read(&mut self.mem[512..MAX_MEMORY]).unwrap();
    }

    fn advance_pc(&mut self) {
        let opcode: u16 = (self.mem[self.pc] as u16 * 256) + self.mem[self.pc + 1] as u16;
        self.pc = if self.pc + 2 >= MAX_MEMORY { 0x200 } else { self.pc + 2 };
        match self.process_opcode(&opcode) {
            Ok(instruction) => self.do_instruction(&instruction, &opcode),
            Err(e) => println!("{}", e)
        }
    }

    fn process_opcode(&mut self, opcode: &u16) -> Result<Opcode, &str> {
        match opcode >> 12 {
            0x0 => match opcode & 0x00FF {
                0xe0 => Ok(Opcode::Clear),
                0xee => Ok(Opcode::Return),
                _ => Err("Opcode not found")
            },
            0x1 => Ok(Opcode::JumpAddr),
            0x2 => Ok(Opcode::Call),
            0x3 => Ok(Opcode::SkipEqualVxkk),
            0x4 => Ok(Opcode::SkipNotEqualVxkk),
            0x5 => Ok(Opcode::SkipEqualVxVy),
            0x6 => Ok(Opcode::LoadVxkk),
            0x7 => Ok(Opcode::AddVxkk),
            0x8 => match opcode & 0x000F {
                0x0 => Ok(Opcode::LoadVxVy),
                0x1 => Ok(Opcode::Or),
                0x2 => Ok(Opcode::And),
                0x3 => Ok(Opcode::Xor),
                0x4 => Ok(Opcode::AddVxVy),
                0x5 => Ok(Opcode::Subtract),
                0x6 => Ok(Opcode::RightShift),
                0x7 => Ok(Opcode::SubtractNotBorrow),
                0xe => Ok(Opcode::LeftShift),
                _ => Err("Opcode not found")
            },
            0x9 => Ok(Opcode::SkipNotEqualVxVy),
            0xa => Ok(Opcode::LoadI),
            0xb => Ok(Opcode::JumpAddrV0),
            0xc => Ok(Opcode::Random),
            0xd => Ok(Opcode::Draw),
            0xe => match opcode & 0x00FF {
                0x9e => Ok(Opcode::SkipIfVxPressed),
                0xa1 => Ok(Opcode::SkipIfVxNotPressed),
                _ => Err("Opcode not found")
            },
            0xf => match opcode & 0x00FF {
                0x07 => Ok(Opcode::LoadVxDelayTimer),
                0x0a => Ok(Opcode::LoadPressedKeyVx),
                0x15 => Ok(Opcode::LoadDelayTimerVx),
                0x18 => Ok(Opcode::LoadSoundTimerVx),
                0x1e => Ok(Opcode::AddIVx),
                0x29 => Ok(Opcode::LoadISpritePositionVx),
                0x33 => Ok(Opcode::LoadIBCDVx),
                0x55 => Ok(Opcode::LoadIVRegisters),
                0x65 => Ok(Opcode::LoadVRegistersI),
                _ => Err("Opcode not found")
            },
            _ => Err("Opcode not found")
        }
    }

    fn do_instruction(&mut self, instruction: &Opcode, opcode: &u16) {
        match instruction {
            Opcode::Clear => println!("CLS"),

            Opcode::Return => {
                if self.sp > 0 {
                    self.pc = self.stack[self.sp] as usize;
                    self.sp -= 1;
                }
            },

            Opcode::JumpAddr => self.pc = (opcode & 0x0FFF) as usize,

            Opcode::Call => {
                if self.sp < MAX_STACK {
                    self.sp += 1;
                    self.stack[self.sp] = self.pc as u16;
                    self.pc = (opcode & 0x0FFF) as usize
                }
            },

            Opcode::SkipEqualVxkk => {
                if self.v[(opcode >> 8 & 0x0F) as usize] == (opcode & 0x00FF) as u8 {
                    self.pc = if self.pc + 2 >= MAX_MEMORY { 0x200 } else { self.pc + 2 };
                }
            } 
            
            Opcode::SkipNotEqualVxkk => {
                if self.v[(opcode >> 8 & 0x0F) as usize] != (opcode & 0x00FF) as u8 {
                    self.pc = if self.pc + 2 >= MAX_MEMORY { 0x200 } else { self.pc + 2 };
                }
            },

            Opcode::SkipEqualVxVy => {
                if self.v[(opcode >> 8 & 0x0F) as usize] == self.v[(opcode >> 4 & 0x00F) as usize] {
                    self.pc = if self.pc + 2 >= MAX_MEMORY { 0x200 } else { self.pc + 2 };     
                }
            },

            Opcode::LoadVxkk => self.v[(opcode >> 8 & 0x0F) as usize] = (opcode & 0x00FF) as u8,

            Opcode::AddVxkk => self.v[(opcode >> 8 & 0x0F) as usize] += ((opcode & 0x00FF) as u8) & 0x00FF,
            
            Opcode::LoadVxVy => self.v[(opcode >> 8 & 0x0F) as usize] = self.v[(opcode >> 4 & 0x00F) as usize],

            Opcode::Or => self.v[(opcode >> 8 & 0x0F) as usize] |= self.v[(opcode >> 4 & 0x00F) as usize],
            
            Opcode::And => self.v[(opcode >> 8 & 0x0F) as usize] &= self.v[(opcode >> 4 & 0x00F) as usize],
            
            Opcode::Xor => self.v[(opcode >> 8 & 0x0F) as usize] ^= self.v[(opcode >> 4 & 0x00F) as usize],

            Opcode::AddVxVy => {
                self.v[(opcode >> 8 & 0x0F) as usize] += self.v[(opcode >> 4 & 0x00F) as usize];
                if self.v[(opcode >> 8 & 0x0F) as usize] > self.v[(opcode >> 8 & 0x0F) as usize] + self.v[(opcode >> 4 & 0x00F) as usize] {
                    self.v[0x0F] = 1;
                }else {
                    self.v[0x0F] = 0;
                }
            },

            Opcode::Subtract => {
                if self.v[(opcode >> 8 & 0x0F) as usize] > self.v[(opcode >> 4 & 0x00F) as usize] {
                    self.v[0x0F] = 1;
                }else {
                    self.v[0x0F] = 0;
                }
                self.v[(opcode >> 8 & 0x0F) as usize] -= self.v[(opcode >> 4 & 0x00F) as usize];
            },
            
            Opcode::RightShift => {
                self.v[0x0F] = self.v[(opcode >> 8 & 0x0F) as usize] & 1;
                self.v[(opcode >> 8 & 0x0F) as usize] >>= 1;
            },

            Opcode::SubtractNotBorrow => {
                if self.v[(opcode >> 4 & 0x00F) as usize] > self.v[(opcode >> 8 & 0x0F) as usize] {
                    self.v[0x0F] = 1;
                }else {
                    self.v[0x0F] = 0;
                }
                self.v[(opcode >> 8 & 0x0F) as usize] = self.v[(opcode >> 4 & 0x00F) as usize] - self.v[(opcode >> 8 & 0x0F) as usize];
            },
            
            Opcode::LeftShift => {
                self.v[0x0F] = if self.v[(opcode >> 8 & 0x0F) as usize] & 0x80 != 0 { 1 } else { 0 };
                self.v[(opcode >> 8 & 0x0F) as usize] <<= 1;
            },
            
            Opcode::SkipNotEqualVxVy => println!("SNE Vx, Vy"),

            Opcode::LoadI => self.i = (opcode & 0x0FFF) as usize,

            Opcode::JumpAddrV0 => self.pc = (opcode & 0x0FFF) as usize + self.v[0x00] as usize,
            
            Opcode::Random => self.v[(opcode >> 8 & 0x0F) as usize] = rand::thread_rng().gen_range(0..=255) & ((opcode & 0x00FF) as u8),
            
            Opcode::Draw => println!("DRW Vx, Vy, nibble"),
            Opcode::SkipIfVxPressed => println!("SKP Vx"),
            Opcode::SkipIfVxNotPressed => println!("SKNP Vx"),

            Opcode::LoadVxDelayTimer => self.v[(opcode >> 8 & 0x0F) as usize] = self.dt,

            Opcode::LoadPressedKeyVx => println!("LD Vx, K"),

            Opcode::LoadDelayTimerVx => self.dt = self.v[(opcode >> 8 & 0x0F) as usize],

            Opcode::LoadSoundTimerVx => self.st = self.v[(opcode >> 8 & 0x0F) as usize],
            
            Opcode::AddIVx => self.i += self.v[(opcode >> 8 & 0x0F) as usize] as usize,

            Opcode::LoadISpritePositionVx => println!("LD F, Vx"),
            Opcode::LoadIBCDVx => println!("LD B, Vx"),

            Opcode::LoadIVRegisters => {
                for pos in 0..(opcode >> 8 & 0x0F) as usize {
                    self.mem[self.i + pos] = self.v[pos];
                }
            },

            Opcode::LoadVRegistersI => {
                for pos in 0..(opcode >> 8 & 0x0F) as usize {
                    self.v[pos] = self.mem[self.i + pos];
                }
            }
        }
    }
}

mod cpu;

use cpu::Cpu;
use cpu::Chip8;

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("[-] ERROR (1): Expected one argument");
        println!("[?] Usage: {} <rom>", args[0]);
        return;
    }

    let mut chip: Chip8 = Chip8::new();
    chip.load_rom(&args[1]);

    println!("{:?}", chip.mem);

}

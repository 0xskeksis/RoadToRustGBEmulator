mod emu;
mod rom;
mod registers;
mod instructions;
mod memory;

use crate::registers::Registers;
use crate::memory::Memory;
use crate::emu::Emu;
use std::env;

use crate::rom::load_rom;



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./Emu <Rom>");
        return ;
    }
    let rom = load_rom(args[1].clone()).unwrap();
    let mut emu:Emu = Emu::new();
    for (i, chunk) in rom.chunks(2).enumerate(){
        let opcode = if chunk.len() == 2 {
            (chunk[0] as u16) << 8 | chunk[1] as u16
        } else{
            chunk[0] as u16
        };
        println!("0x{:04X}: 0x{:04X}\t", 0x200 + i * 2, opcode);
        emu.instructions[((opcode & 0xF000) >> 12) as usize](&mut emu, opcode);
    }
}

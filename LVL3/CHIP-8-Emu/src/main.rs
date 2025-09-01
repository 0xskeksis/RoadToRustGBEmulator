mod emu;
mod rom;
mod registers;
mod instructions;
mod sdl;

use sdl2::render;

use crate::registers::Registers;
use crate::emu::Emu;
use std::env;

use crate::rom::load_rom;
use crate::sdl::render_loop;



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./Emu <Rom>");
        return ;
    }
    let rom = load_rom(args[1].clone()).unwrap();
    let mut emu:Emu = Emu::new();
    for (i, &byte) in rom.iter().enumerate() {
        emu.memory[0x200 + i] = byte;

    }

    render_loop(&mut emu);
}

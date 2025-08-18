use crate::instructions::{self, *};
use crate::registers::Registers;
use crate::memory::Memory;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const START_ADDR: u16 = 0x200;

pub struct Emu {
    pub registers: Registers,
    pub memory: [u8; 4096],
    pub instructions: [fn(&mut Emu, u16); 16],
    pub display: [bool; SCREEN_HEIGHT * SCREEN_WIDTH],
    pub stack: [u16; STACK_SIZE],
}

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

impl Emu{
    fn load_fonts(&mut self){
        self.memory[0x000..0x050].copy_from_slice(&FONTSET);
    }
    pub fn new() -> Self {
        let mut emu = Emu {
            registers: Registers {
                v: [0; 16],
                i:0,
                pc: START_ADDR,
                sp: 0,
                dt: 0,
                st:0,
            },
            memory: {[0; 4096]},
            instructions: [
                instructions::op_0,
                instructions::op_1,
                instructions::op_2,
                instructions::op_3,
                instructions::op_4,
                instructions::op_5,
                instructions::op_6,
                instructions::op_7,
                instructions::op_8,
                instructions::op_9,
                instructions::op_a,
                instructions::op_b,
                instructions::op_c,
                instructions::op_d,
                instructions::op_e,
                instructions::op_f,
            ],
            display: [false; SCREEN_HEIGHT * SCREEN_WIDTH],
            stack: [0x0; STACK_SIZE],
        };
        emu.load_fonts();
        emu
    }
}

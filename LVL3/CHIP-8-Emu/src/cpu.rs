use crate::registers::Registers;

pub struct Cpu {
    pub registers: Registers,
    pub memory: Memory,
}

impl Cpu{
    pub fn new() -> Self {
        Cpu {
            v: [0; 16],
            i:0,
            pc: 0x200,
            sp: 0,
            dt: 0,
            st:0,
        }
    }
}

use crate::registers::Registers;

pub struct Cpu {
    pub registers: Registers,
    pub memory: Memory,
    pub instructions: [fn(&mut Cpu, u16); 16],
}

impl Cpu{
    pub fn new() -> Self {
        Cpu {
            registers: Registers {
                v: [0; 16],
                i:0,
                pc: 0x200,
                sp: 0,
                dt: 0,
                st:0,
            },
            memory: Memory {data: [0; 4096]},
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
                instructions::op_A,
                instructions::op_B,
                instructions::op_C,
                instructions::op_D,
                instructions::op_E,
                instructions::op_F,
            ]
        }
    }
}

///This struct represents all of the CHIP-8 Registers.
///v0..VE are the multi-purpose Registers.
///VF should not be used by any program, as it is used as a flag by some instructions.
///The 'I' register are normally used to store address, so the 12 lowest bits are usually unused
///Next, we got the program counter (PC), used to store the currently executing address
///The Stack Pointer (SP) is used to store the topmost level of the stack.

struct Registers {
    v: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
    dt: u8,
    st: u8,
}

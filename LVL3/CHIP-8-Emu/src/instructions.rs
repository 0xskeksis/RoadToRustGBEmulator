/// Instructions lookup tables
///
/// Refs: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
///

use crate::emu::Emu;

pub fn op_0(emu: &mut Emu, opcode: u16){
    match opcode {
        0x00E0 => println!("CLS"),
        0x00EE => println!("RET"),
        _ => println!("Error in op_0: Opcode: {opcode} unknown."),
    }
}

pub fn op_1(emu: &mut Emu, opcode: u16){
    println!("JP");
}

pub fn op_2(emu: &mut Emu,opcode: u16){
    println!("CALL");
}

pub fn op_3(emu: &mut Emu, opcode: u16){
    println!("SE Vx, byte");
}

pub fn op_4(emu: &mut Emu, opcode: u16){
    println!("SNE Vx, byte")
}

pub fn op_5(emu: &mut Emu, opcode: u16){
    println!("SE Vx, Vy");
}

pub fn op_6(emu: &mut Emu, opcode: u16){
    println!("LD Vx, byte");
}

pub fn op_7(emu: &mut Emu, opcode: u16){
    println!("ADD Vx, byte")
}

pub fn op_8(emu: &mut Emu, opcode: u16){
    let n = opcode & 0xF;
    match n {
        0x0 => println!("LD Vx, Vy"),
        0x1 => println!("OR Vx, Vy"),
        0x2 => println!("AND Vx, Vy"),
        0x3 => println!("XOR Vx, Vy"),
        0x4 => println!("ADD Vx, Vy"),
        0x5 => println!("SUB Vx, Vy"),
        0x6 => println!("SHR Vx , Vy"),
        0x7 => println!("SUBN Vx, Vy"),
        0xE => println!("SHL Vx ,Vy"),
        _ => println!("Error in op_8: Opcode: {n} unknown."),
    }
}

pub fn op_9(emu: &mut Emu, opcode: u16){
    println!("SNE Vx, Vy");
}

pub fn op_a(emu: &mut Emu, opcode: u16){
    println!("LD I, addr");
}

pub fn op_b(emu: &mut Emu, opcode: u16){
    println!("JP V0, addr");
}

pub fn op_c(emu: &mut Emu, opcode: u16){
    println!("RND Vx, byte");
}

pub fn op_d(emu: &mut Emu, opcode: u16){
    println!("DRW Vx, Vy, nibble");
}

pub fn op_e(emu: &mut Emu, opcode: u16){
    let n = opcode & 0xFF;

    match n {
        0x9E => println!("SKP Vx"),
        0xA1 => println!("SKNP Vx"),
        _ => println!("Error in op_E: Opcode: {n} unknown."),
    }
}

pub fn op_f(emu: &mut Emu, opcode: u16){
    let n = opcode & 0xFF;

    match n {
        0x7 => println!("LD Vx, DT"),
        0x0A => println!("LD Vx, K"),
        0x15 => println!("LD DT, Vx"),
        0x18 => println!("LD ST, Vx"),
        0x1E => println!("ADD I, Vx"),
        0x29 => println!("LD F, Vx"),
        0x33 => println!("LD B, Vx"),
        0x55 => println!("LD [I], Vx"),
        0x65 => println!("LD Vx, [I]"),
        _ => println!("Error in op_F: Opcode: {n} unknown."),
    }
}

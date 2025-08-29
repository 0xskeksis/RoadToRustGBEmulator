/// Instructions lookup tables
///
/// Refs: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
///

use crate::emu::Emu;

const KK_MASK: u16 = 0x00FF;
const NNN_MASK: u16 = 0x0FFF;
const X_MASK: u16 = 0x0F00;
const Y_MASK: u16 = 0x00F0;
const LAST_MASK: u16 = 0x000F;

/*----------------------------------------------------------------------------*/
/*                          INSTRUCTIONS REDIRECTIONS                         */
/*----------------------------------------------------------------------------*/

///0x00E0 - CLS:
///Clear the display.
///00EE - RET:
///Return from a subroutine.
pub fn op_0(emu: &mut Emu, opcode: u16){
    match opcode {
        0x00E0 => clear(emu),
        0x00EE => ret(emu),
        _ => println!("Error in op_0: Opcode: {opcode} unknown."),
    }
    _ = emu;
}

///1nnn - JP addr
///Jump to location nnn.
pub fn op_1(emu: &mut Emu, opcode: u16){
    let nnn = opcode & NNN_MASK;
    jump(emu, nnn);
    _ = emu;
}

///2nnn - CALL addr
///Call subroutine at nnn.
pub fn op_2(emu: &mut Emu,opcode: u16){
    let nnn = opcode & NNN_MASK;
    call(emu, nnn);
    _ = emu;
}

///3xkk - SE Vx, byte
///Skip next instruction if Vx = kk.
pub fn op_3(emu: &mut Emu, opcode: u16){
    let x = ((opcode & X_MASK) >> 8) as usize;
    let kk = (opcode & KK_MASK) as u8;
    if emu.registers.v[x] == kk {skip(emu);}
}

///4xkk - SNE Vx, byte
///Skip next instruction if Vx != kk.
pub fn op_4(emu: &mut Emu, opcode: u16){
    let x = ((opcode & X_MASK) >> 8) as usize;
    let kk = (opcode & KK_MASK) as u8;
    if emu.registers.v[x] != kk {skip(emu);}
}

///5xy0 - SE Vx, Vy
///Skip next instruction if Vx = Vy.
pub fn op_5(emu: &mut Emu, opcode: u16){
    let x = ((opcode & X_MASK) >> 8) as usize;
    let y = ((opcode & Y_MASK) >> 4) as usize;
    let last = opcode & LAST_MASK;

    if last > 0x0 {
        println!("Error in op_5: opcode: {opcode} unknown.");
        return ;
    }
    if emu.registers.v[x] != emu.registers.v[y] {skip(emu);}
    _ = emu;
}

///6xkk - LD Vx, byte
///Set Vx = kk.
pub fn op_6(emu: &mut Emu, opcode: u16){
    let x = ((opcode & X_MASK) >> 8) as usize;
    let kk = (opcode & KK_MASK) as u8;
    emu.registers.v[x] = kk;
}

///7xkk - ADD Vx, byte
///Set Vx = Vx + kk.
pub fn op_7(emu: &mut Emu, opcode: u16){
    let x = ((opcode & X_MASK) >> 8) as usize;
    let kk = (opcode & KK_MASK) as u8;
    emu.registers.v[x] += kk as u8; //overflow
}

pub fn op_8(emu: &mut Emu, opcode: u16){
    let n = opcode & LAST_MASK;
    let x = ((opcode & X_MASK) >> 8) as usize;
    let y = ((opcode & Y_MASK) >> 4) as usize;
    match n {
        0x0 => emu.registers.v[x] = emu.registers.v[y],     //LD Vx, Vy
        0x1 => emu.registers.v[x] |= emu.registers.v[y],    //OR Vx, Vy 
        0x2 => emu.registers.v[x] &= emu.registers.v[y],    //AND Vx, Vy 
        0x3 => emu.registers.v[x] ^= emu.registers.v[y],    //XOR Vx, Vy 
        0x4 => _ = emu, // flemme pour l'instant (ADD Vx, Vy but with carry)
        0x5 => println!("SUB Vx, Vy"),
        0x6 => println!("SHR Vx , Vy"),
        0x7 => println!("SUBN Vx, Vy"),
        0xE => println!("SHL Vx ,Vy"),
        _ => println!("Error in op_8: Opcode: {n} unknown."),
    }
    _ = emu;
}

///9xy0 - SNE Vx, Vy
///Skip next instruction if Vx != Vy.
pub fn op_9(emu: &mut Emu, opcode: u16){
    let x = ((opcode & X_MASK) >> 8) as usize;
    let y = ((opcode & Y_MASK) >> 4) as usize;
    let last = opcode & LAST_MASK;

    if last > 0x0 {
        println!("Error in op_9: opcode: {opcode} unknown.");
        return ;
    }
    if emu.registers.v[x] != emu.registers.v[y] {skip(emu);}
}

///Annn - LD I, addr
///Set I = nnn.
pub fn op_a(emu: &mut Emu, opcode: u16){
    let nnn = opcode & NNN_MASK;
    println!("LD I, addr | addr = {nnn}");
    _ = emu;
}

///Bnnn - JP V0, addr
//Jump to location nnn + V0.
pub fn op_b(emu: &mut Emu, opcode: u16){
    let nnn = opcode & NNN_MASK;
    println!("JP V0, addr | addr = {nnn}");
    _ = emu;
}

///Cxkk - RND Vx, byte
///Set Vx = random byte AND kk.
pub fn op_c(emu: &mut Emu, opcode: u16){
    let x = (opcode & X_MASK) >> 8;
    let kk = opcode & KK_MASK;
    println!("RND Vx, byte | Vx = V{x}, byte = {kk}");
    _ = emu;
}

///Dxyn - DRW Vx, Vy, nibble
///Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
pub fn op_d(emu: &mut Emu, opcode: u16){
    let x = ((opcode & X_MASK) >> 8) as usize;
    let y = ((opcode & Y_MASK) >> 4) as usize;
    let n = opcode & LAST_MASK;
    println!("DRW Vx, Vy, nibble | DRW V{x}, V{y}, {n}");
    _ = emu;
}

///Ex9E - SKP Vx
///Skip next instruction if key with the value of Vx is pressed.
pub fn op_e(emu: &mut Emu, opcode: u16){
    let n = opcode & KK_MASK;
    let x = (opcode & X_MASK) >> 8;
    _ = x;
    match n {
        0x9E => println!("SKP Vx"),
        0xA1 => println!("SKNP Vx"),
        _ => println!("Error in op_e: Opcode: {n} unknown."),
    }
    _ = emu;
}

pub fn op_f(emu: &mut Emu, opcode: u16){
    let n = opcode & KK_MASK;
    let x = (opcode & X_MASK) >> 8;

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
    _ = emu;
}

/*----------------------------------------------------------------------------*/
/*                            BASIC INSTRUCTIONS                              */
/*----------------------------------------------------------------------------*/

fn clear(emu: &mut Emu){
    emu.display.fill(false);
}

fn ret(emu: &mut Emu){
    emu.registers.pc = emu.stack[0x0];
    emu.registers.sp -= 1;
}

fn jump(emu: &mut Emu, addr: u16){
    emu.registers.pc = addr;
}

fn call(emu: &mut Emu, addr: u16){
    emu.registers.sp += 1;
    emu.stack[0x0] = emu.registers.pc; // Warning !!! Need to save the value before erasing her !
    emu.registers.pc = addr;
}

fn skip(emu: &mut Emu){
    emu.registers.pc += 2;
}

use std::ops::Add;
use sdl2::event::Event;

/// Instructions lookup tables
///
/// Refs: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
///
use rand::Rng;

use crate::emu::Emu;
use crate::emu::FONTSET;

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
    emu.registers.v[x] = emu.registers.v[x].wrapping_add(kk);}

pub fn op_8(emu: &mut Emu, opcode: u16){
    let n = opcode & LAST_MASK;
    let x = ((opcode & X_MASK) >> 8) as usize;
    let y = ((opcode & Y_MASK) >> 4) as usize;
    match n {
        0x0 => emu.registers.v[x] = emu.registers.v[y],     //LD Vx, Vy
        0x1 => emu.registers.v[x] |= emu.registers.v[y],    //OR Vx, Vy 
        0x2 => emu.registers.v[x] &= emu.registers.v[y],    //AND Vx, Vy 
        0x3 => emu.registers.v[x] ^= emu.registers.v[y],    //XOR Vx, Vy 
        0x4 => {
            let (sum, carry) = emu.registers.v[x].overflowing_add(emu.registers.v[y]);
            emu.registers.v[x] = sum;
            emu.registers.v[0xF] = if carry{1} else {0};
        }
        0x5 => {
            let (sum, overflow) = emu.registers.v[x].overflowing_sub(emu.registers.v[y]);
            emu.registers.v[x] = sum;
            emu.registers.v[0xF] = if overflow{1} else {0};
        }
        0x6 => {
           let lsb = emu.registers.v[x] & 0x1;
           emu.registers.v[0xF] = lsb;
           emu.registers.v[x] >>= 1;
        },
        0x7 => {
            let (sum, carry) = emu.registers.v[y].overflowing_add(emu.registers.v[x]);
            emu.registers.v[x] = sum;
            emu.registers.v[0xF] = if carry{1} else {0};
        },
        0xE => {
           let lsb = emu.registers.v[x] & 0x1;
           emu.registers.v[0xF] = lsb;
           emu.registers.v[x] <<= 1;
        },
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
    emu.registers.i = nnn;
}

///Bnnn - JP V0, addr
//Jump to location nnn + V0.
pub fn op_b(emu: &mut Emu, opcode: u16){
    let nnn = opcode & NNN_MASK;
    jump(emu, nnn + emu.registers.v[0] as u16);
}

///Cxkk - RND Vx, byte
///Set Vx = random byte AND kk.
pub fn op_c(emu: &mut Emu, opcode: u16){
    let x = ((opcode & X_MASK) >> 8) as usize;
    let kk = (opcode & KK_MASK) as u8;
    let mut rng = rand::rng();
    let random_nb = rng.random::<u8>();

    emu.registers.v[x] = kk & random_nb;
}

///Dxyn - DRW Vx, Vy, nibble
///Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
pub fn op_d(emu: &mut Emu, opcode: u16){
    let x = ((opcode & X_MASK) >> 8) as usize;
    let y = ((opcode & Y_MASK) >> 4) as usize;
    let n = opcode & LAST_MASK;

    let vx = emu.registers.v[x as usize];
    let vy = emu.registers.v[y as usize];

    emu.registers.v[0xF] = 0;

    for row in 0..n {
        let sprite_byte = emu.memory[(emu.registers.i as usize).wrapping_add(row.into())];
        for bit in 0..8 {
            // on lit de gauche à droite (MSB → LSB)
            let pixel_on = (sprite_byte >> (7 - bit)) & 1;

            if pixel_on == 1 {
                let px = ((vx as usize + bit) % 64) as usize;
                let py = ((vy as u16 + row) % 32) as usize;
                let index = py * 64 + px;
                if emu.display[index] {
                    emu.registers.v[0xF] = 1;
                }
                emu.display[index] = !emu.display[index];
            }
        }
    }
}

///Ex9E - SKP Vx
///Skip next instruction if key with the value of Vx is pressed.
pub fn op_e(emu: &mut Emu, opcode: u16){
    let n = opcode & KK_MASK;
    let x = (opcode & X_MASK) >> 8;
    _ = x;
    if x <= 0xF {
        match n {
            0x9E => {
                if emu.keys[emu.registers.v[x as usize] as usize] == true {
                    skip(emu);
                }
            }
            0xA1 => {
                if emu.keys[emu.registers.v[x as usize] as usize] != true {
                    skip(emu);
                }
            }
            _ => println!("Error in op_e: Opcode: {n} unknown."),
        }

    }
}

pub fn op_f(emu: &mut Emu, opcode: u16){
    let n = opcode & KK_MASK;
    let x = ((opcode & X_MASK) >> 8) as usize;

    match n {
        0x7 => emu.registers.v[x as usize] = emu.registers.dt,
        0x0A => {
            emu.waiting_for_key = Some(x);
        },
        0x15 => emu.registers.dt = emu.registers.v[x],
        0x18 => emu.registers.st = emu.registers.v[x],
        0x1E => emu.registers.i += (emu.registers.v[x]) as u16,
        0x29 => emu.registers.i = (emu.registers.v[x] as u16) * 5,
        0x33 => {
            emu.memory[emu.registers.i as usize] = emu.registers.v[x] / 100;
            emu.memory[emu.registers.i as usize + 1] = (emu.registers.v[x] / 10) % 10;
            emu.memory[emu.registers.i as usize + 2] = emu.registers.v[x] % 10;}
        0x55 => {
            for i in 0..=x {
                emu.memory[emu.registers.i as usize + i] = emu.registers.v[i];
            }
        }
        0x65 => {
               for i in 0..=x {
                emu.registers.v[i] = emu.memory[emu.registers.i as usize + i];
            }
        }
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
    if emu.registers.sp == 0 {
        println!("Error: stack underflow on RET");
        return;
    }
    emu.registers.pc = emu.stack[emu.registers.sp as usize];
    emu.registers.sp -= 1;
}

fn jump(emu: &mut Emu, addr: u16){
    emu.registers.pc = addr;
}

fn call(emu: &mut Emu, addr: u16){
   if emu.registers.sp as usize >= emu.stack.len() - 1 { 
        println!("Error: stack overflow on CALL");
        return;
    }
    emu.registers.sp += 1;
    emu.stack[emu.registers.sp as usize] = emu.registers.pc;
    emu.registers.pc = addr;
}

fn skip(emu: &mut Emu){
    emu.registers.pc += 2;
}

pub fn fetch_opcode(emu: &mut Emu) -> u16 {
    let pc = emu.registers.pc as usize;
    let high = emu.memory[pc] as u16;
    let low  = emu.memory[pc + 1] as u16;
    emu.registers.pc += 2;
    (high << 8) | low
}

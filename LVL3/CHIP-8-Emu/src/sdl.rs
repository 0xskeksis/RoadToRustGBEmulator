use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

use crate::emu::Emu;
use crate::instructions::fetch_opcode;

fn map_to_chip8(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q    => Some(0x4),
        Keycode::W    => Some(0x5),
        Keycode::E    => Some(0x6),
        Keycode::R    => Some(0xD),
        Keycode::A    => Some(0x7),
        Keycode::S    => Some(0x8),
        Keycode::D    => Some(0x9),
        Keycode::F    => Some(0xE),
        Keycode::Z    => Some(0xA),
        Keycode::X    => Some(0x0),
        Keycode::C    => Some(0xB),
        Keycode::V    => Some(0xF),
        _ => None,
    }
}

pub fn render_loop(emu: &mut Emu) -> Result<(), String>{
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;


    let window = video_subsystem
        .window("Chip8-Emulator", 64 * 10, 32 * 10)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

     let mut event_pump = sdl_context.event_pump()?;
     let mut last_timer_update = Instant::now();

'running: loop {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} => break 'running,

            Event::KeyDown { keycode: Some(key), .. } => {
                if let Some(chip8_key) = map_to_chip8(key) {
                    emu.keys[chip8_key] = true;

                    if let Some(vx_index) = emu.waiting_for_key {
                        emu.registers.v[vx_index] = chip8_key as u8;
                        emu.waiting_for_key = None;
                        emu.registers.pc += 2;
                    }
                }
            }

            Event::KeyUp { keycode: Some(key), .. } => {
                if let Some(chip8_key) = map_to_chip8(key) {
                    emu.keys[chip8_key] = false;
                }
            }

            _ => {}
        }
    }

    if emu.waiting_for_key.is_some() {
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
        continue;
    }

    for _ in 0..50 {
        let opcode = fetch_opcode(emu);
        let index = ((opcode & 0xF000) >> 12) as usize;
        emu.instructions[index](emu, opcode);
    }

    if last_timer_update.elapsed() >= Duration::from_millis(16) {
            if emu.registers.dt > 0 {
                emu.registers.dt -= 1;
            }
            if emu.registers.st > 0 {
                emu.registers.st -= 1;
            }
            last_timer_update = Instant::now();
        }


        canvas.set_draw_color(Color::BLACK);
        canvas.clear();


        canvas.set_draw_color(Color::WHITE);
        for y in 0..32 {
            for x in 0..64 {
                if emu.display[y * 64 + x] == true {
                    let rect = sdl2::rect::Rect::new(
                        (x as i32) * 10,
                        (y as i32) * 10,
                        10,
                        10);
                    canvas.fill_rect(rect)?;
                }
            }
        }

        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}

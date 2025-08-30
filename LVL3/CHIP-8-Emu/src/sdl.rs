use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use crate::emu::Emu;

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

     'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                Event::KeyDown { keycode: Some(key), .. } => {
                    if let Some(chip8_key) = map_to_chip8(key) {
                        emu.keys[chip8_key] = true;
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

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.set_draw_color(Color::WHITE);
        canvas.fill_rect(sdl2::rect::Rect::new(10 * 10, 5 * 10, 10, 10))?;

        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}
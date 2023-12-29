extern crate sdl2;

use sdl2::video::*;
use sdl2::render::*;
use sdl2::pixels::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

extern crate crossbeam;

extern crate chirp8_engine as chirp8;

mod engine;
mod machine;
mod lcd;

use machine::*;
use lcd::*;

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut events = sdl.event_pump().unwrap();

    let mut sdltimer = sdl.timer().unwrap();

    let vidsys = sdl.video().unwrap();
    let window = vidsys.window("CHIRP-8", LCD_WIDTH as u32 * 8, LCD_HEIGHT as u32 * 8)
        // .position_centered()
        .build()
        .unwrap();
    let mut canvas: Canvas<Window> = window.into_canvas()
        .build()
        .unwrap();

    let virt = SDLVirt::new();

    crossbeam::scope(|scope| {
        scope.spawn(|| {
            engine::run(virt.clone());
        });

        'main: loop {
            for event in events.poll_iter() {
                match event {
                    Event::Quit {..} => break 'main,
                    Event::KeyDown {keycode: Some(keycode), ..} => match keycode {
                        Keycode::Escape => break 'main,
                        Keycode::Backspace => {
                            let ref framebuf = virt.lock_framebuf().unwrap();
                            for row in framebuf.iter() {
                                for bit in row.iter() {
                                    print!("{}", if *bit { '*' } else { ' ' });
                                }
                                println!();
                            }
                        },
                        _ => {} },
                    _ => {}
                }
            };

            virt.process_keys(events.keyboard_state());

            if virt.take_redraw() {
                let ref framebuf = virt.lock_framebuf().unwrap();
                draw_lcd(framebuf, &mut canvas);
            };

            virt.tick();

            sdltimer.delay(17); // 60 FPS
        }

        virt.stop_running();
    });
}

extern crate sdl2;

use sdl2::video::*;
use sdl2::render::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

extern crate crossbeam;

extern crate chirp8_engine as chirp8;
extern crate clap;

mod engine;
mod machine;
mod lcd;

use machine::*;
use lcd::*;
use chirp8::cpu::CPU;

use std::sync::Mutex;

use clap::Parser;

/// CHIRP-8 SDL frontend
#[derive(Parser)]
struct CliOpts {
    /// The .ch8 program to run
    #[arg(value_name = "FILE", default_value="hidden.ch8")]
    path: std::path::PathBuf,
}

fn main() {
    let args = CliOpts::parse();
    let file_name: std::path::PathBuf = args.path;

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

    let cpu = Mutex::new(CPU::new());
    let virt = SDLVirt::new();

    crossbeam::scope(|scope| {
        scope.spawn(|_| {
            let mut io = virt.clone();
            engine::setup(&file_name, &mut io);
            while io.keep_running() {
                cpu.lock().unwrap().step(&mut io);
            }
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

            virt.tick(&mut cpu.lock().unwrap());

            sdltimer.delay(17); // 60 FPS
        }

        virt.stop_running();
    }).unwrap();
}

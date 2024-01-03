extern crate sdl2;

use sdl2::video::*;
use sdl2::render::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

extern crate chirp8_engine as chirp8;
extern crate clap;
extern crate fixedstep;

mod engine;
mod machine;
mod lcd;

use machine::*;
use lcd::*;
use chirp8::cpu::{CPU, DefaultQuirks};

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

    let vidsys = sdl.video().unwrap();
    let window = vidsys.window("CHIRP-8", LCD_WIDTH as u32 * 8, LCD_HEIGHT as u32 * 8)
        // .position_centered()
        .build()
        .unwrap();
    let mut canvas: Canvas<Window> = window.into_canvas()
        .build()
        .unwrap();

    let mut cpu = CPU::<DefaultQuirks>::new();
    let mut virt = SDLVirt::new();

    engine::setup(&file_name, &mut virt);

    let mut fixedstep = fixedstep::FixedStep::start(60.0);

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                Event::KeyDown {keycode: Some(keycode), ..} => match keycode {
                    Keycode::Escape => break 'main,
                    Keycode::Backspace => {
                        let ref framebuf = virt.get_framebuf();
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

        while !fixedstep.update() {
            cpu.step(&mut virt)
        }
        let _delta = fixedstep.render_delta();

        draw_lcd(&virt.get_framebuf(), &mut canvas);
        virt.tick(&mut cpu);
    }
}

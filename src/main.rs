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
mod video;

use machine::*;
use video::*;
use chirp8::quirks::*;
use chirp8::cpu::CPU;

use clap::Parser;
use clap::ArgAction;

/// CHIRP-8 SDL frontend
#[derive(Parser)]
struct CliOpts {
    /// The .ch8 program to run
    #[arg(value_name = "FILE")]
    path: std::path::PathBuf,

    #[arg(long = "no-shift-vy", action = ArgAction::SetFalse, default_value_t = true)]
    shift_vy: bool,

    #[arg(long = "no-reset-vf", action = ArgAction::SetFalse, default_value_t = true)]
    reset_vf: bool,

    #[arg(long = "no-increment-ptr", action = ArgAction::SetFalse, default_value_t = true)]
    increment_ptr: bool,

    #[arg(long = "no-video-wait", action = ArgAction::SetFalse, default_value_t = true)]
    video_wait: bool,

    #[arg(long = "no-clip-sprites", action = ArgAction::SetFalse, default_value_t = true)]
    clip_sprites: bool,
}

fn main() {
    let args = CliOpts::parse();
    let file_name: std::path::PathBuf = args.path;
    let quirks = Quirks{
        shift_vy: args.shift_vy,
        reset_vf: args.reset_vf,
        increment_ptr: args.increment_ptr,
        video_wait: args.video_wait,
        clip_sprites: args.clip_sprites,
    };

    let sdl = sdl2::init().unwrap();
    let mut events = sdl.event_pump().unwrap();

    let vidsys = sdl.video().unwrap();
    let window = vidsys.window("CHIRP-8", PIX_WIDTH, PIX_HEIGHT)
        // .position_centered()
        .build()
        .unwrap();
    let mut canvas: Canvas<Window> = window.into_canvas()
        .build()
        .unwrap();

    let mut cpu = CPU::new(quirks);
    let mut virt = SDLVirt::new();

    engine::setup(&file_name, &mut virt);
    // virt.write_ram(0x1ff, 1);

    let mut fixedstep = fixedstep::FixedStep::start(60.0);

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                Event::KeyDown {keycode: Some(keycode), ..} => match keycode {
                    Keycode::Escape => break 'main,
                    // Keycode::Backspace => {
                    //     let ref framebuf = virt.get_framebuf();
                    //     for row in framebuf.iter() {
                    //         for bit in row.iter() {
                    //             print!("{}", if *bit { '#' } else { ' ' });
                    //         }
                    //         println!();
                    //     }
                    // },
                    _ => {} },
                _ => {}
            }
        };

        virt.process_keys(events.keyboard_state());

        while !fixedstep.update() {
            cpu.step(&mut virt)
        }
        let _delta = fixedstep.render_delta();

        render_lcd(&virt.get_framebuf(), &mut canvas);
        virt.tick(&mut cpu);
    }
}

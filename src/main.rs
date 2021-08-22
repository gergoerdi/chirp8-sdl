use std::sync::{Arc,Mutex};
use std::sync::atomic::{Ordering,AtomicBool};

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

extern crate crossbeam;

extern crate chip8_engine as chip8;
use chip8::prelude::*;
use chip8::peripherals::*;

mod engine;
mod lcd;
use lcd::*;

type KeyBuf = [[bool; 4]; 4];
type RAM = [Byte; 1 << 12];

#[derive(Clone)]
struct SDLVirt {
    framebuf: Arc<Mutex<FrameBuf>>,
    redraw: Arc<AtomicBool>,
    run_flag: Arc<AtomicBool>,
    key_state: Arc<Mutex<KeyBuf>>,
    timer: Arc<Mutex<u8>>,
    ram: Arc<Mutex<RAM>>,
}

impl SDLVirt {
    fn tick(&self) {
        let mut timer = self.timer.lock().unwrap();
        if *timer > 0 {
            *timer = *timer - 1
        }
    }

    fn process_key(&self, keycode: Keycode, state: bool) {
        let pos = match keycode {

            Keycode::Num1 => Some((0, 0)),
            Keycode::Num2 => Some((0, 1)),
            Keycode::Num3 => Some((0, 2)),
            Keycode::Num4 => Some((0, 3)),

            Keycode::Q => Some((1, 0)),
            Keycode::W => Some((1, 1)),
            Keycode::E => Some((1, 2)),
            Keycode::R => Some((1, 3)),

            Keycode::A => Some((2, 0)),
            Keycode::S => Some((2, 1)),
            Keycode::D => Some((2, 2)),
            Keycode::F => Some((2, 3)),

            Keycode::Z => Some((3, 0)),
            Keycode::X => Some((3, 1)),
            Keycode::C => Some((3, 2)),
            Keycode::V => Some((3, 3)),

            _ => None,
        };

        match pos {
            Some((r, c)) => {
                self.key_state.lock().unwrap()[r][c] = state;
            },
            _ => {}
        }
    }

    // fn blit(&self, draw_surface: &SurfaceRef, dest: &mut SurfaceRef) {
    //     let ref framebuf = self.framebuf.lock().unwrap();
    //     draw_lcd(framebuf, dest);
    //     draw_surface.blit_scaled(None, dest, None).unwrap();
    // }
}

impl Peripherals for SDLVirt {
    fn keep_running(&self) -> bool {
        self.run_flag.load(Ordering::Relaxed)
    }

    fn clear_pixels(&self) {
        let mut framebuf = self.framebuf.lock().unwrap();
        for row in framebuf.iter_mut() {
            for x in row.iter_mut() {
                *x = false;
            }
        }
    }

    fn set_pixel(&self, x: Byte, y: Byte, v: bool) {
        self.framebuf.lock().unwrap()[(y + 8) as usize][(x + 10) as usize] = v;
    }

    fn get_pixel(&self, x: Byte, y: Byte) -> bool {
        self.framebuf.lock().unwrap()[(y + 8) as usize][(x + 10) as usize]
    }

    fn scan_key_row(&self, row: Byte) -> Byte {
        let row = self.key_state.lock().unwrap()[row as usize];

        let mut mask = 0;
        for (i, b) in row.iter().enumerate() {
            if *b { mask = mask | 1 << i }
        }
        mask
    }

    fn set_timer(&self, v: Byte) {
        *self.timer.lock().unwrap() = v
    }

    fn get_timer(&self) -> Byte {
        *self.timer.lock().unwrap()
    }

    fn redraw(&self) {
        self.redraw.store(true, Ordering::Relaxed);
    }

    fn read_ram(&self, addr: Addr) -> Byte {
        self.ram.lock().unwrap()[addr as usize]
    }

    fn write_ram(&self, addr: Addr, b: Byte) {
        self.ram.lock().unwrap()[addr as usize] = b;
    }

    fn get_random(&self) -> Byte {
        return 42; // TODO
    }

    fn set_sound(&self, _value: Byte) {
        // TODO
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut events = sdl.event_pump().unwrap();

    let mut sdltimer = sdl.timer().unwrap();

    let vidsys = sdl.video().unwrap();
    let mut window = vidsys.window("RUST-8", LCD_WIDTH as u32 * 8, LCD_HEIGHT as u32 * 8)
        // .position_centered()
        .build()
        .unwrap();

    let ref mut draw_surface = lcd::new_draw_surface(window.surface(&events).unwrap().pixel_format());

    let virt = SDLVirt{
        run_flag: Arc::new(AtomicBool::new(true)),
        framebuf: Arc::new(Mutex::new([[false; LCD_WIDTH as usize]; LCD_HEIGHT as usize])),
        redraw: Arc::new(AtomicBool::new(true)),
        key_state: Arc::new(Mutex::new([[false; 4]; 4])),
        timer: Arc::new(Mutex::new(0)),
        ram: Arc::new(Mutex::new([0; 1 << 12])),
    };

    crossbeam::scope(|scope| {
        scope.spawn(|| {
            engine::run(virt.clone());
        });

        'main: loop {
            for event in events.poll_iter() {
                match event {
                    Event::Quit {..} => break 'main,

                    Event::KeyDown {keycode: Some(keycode), ..} => {
                        if keycode == Keycode::Escape {
                            break 'main
                        } else {
                            virt.process_key(keycode, true);
                        }
                    }

                    Event::KeyUp { keycode: Some(keycode), ..} => {
                        virt.process_key(keycode, false);
                    }

                    _ => {}
                }
            };

            if virt.redraw.swap(false, Ordering::Relaxed) {
                let mut screen_surface = window.surface_mut(&events).unwrap();

                // virt.blit(&draw_surface, &mut screen_surface);

                let ref framebuf = virt.framebuf.lock().unwrap();
                draw_lcd(framebuf, draw_surface);
                draw_surface.blit_scaled(None, &mut screen_surface, None).unwrap();
            };
            window.update_surface().unwrap();

            virt.tick();

            sdltimer.delay(17); // 60 FPS
        }

        virt.run_flag.store(false, Ordering::Relaxed);
    });
}

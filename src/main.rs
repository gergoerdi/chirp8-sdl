use std::sync::{Arc,Mutex};
use std::sync::atomic::{Ordering,AtomicBool};

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

extern crate crossbeam;

mod peripherals;
use peripherals::*;

mod engine;

mod lcd;
use lcd::*;

struct SDLVirt {
    framebuf: Arc<Mutex<FrameBuf>>,
    run_flag: Arc<AtomicBool>,
}

impl Peripherals for SDLVirt {
    fn keep_running(&self) -> bool {
        self.run_flag.load(Ordering::Relaxed)
    }

    fn set_pixel(&self, x: u8, y: u8, v: bool) {
        self.framebuf.lock().unwrap()[y as usize][x as usize] = v;
    }       
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut timer = sdl.timer().unwrap();

    let vidsys = sdl.video().unwrap();
    let mut window = vidsys.window("RUST-8", SCREEN_WIDTH as u32 * 8, SCREEN_HEIGHT as u32 * 8)
        // .position_centered()
        .build()
        .unwrap();

    let mut events = sdl.event_pump().unwrap();

    let pixel_format = window.surface(&events).unwrap().pixel_format();
    
    let framebuf = Arc::new(Mutex::new([[false; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize]));
    let ref mut draw_surface = lcd::new_draw_surface(pixel_format);
       
    let run_flag = Arc::new(AtomicBool::new(true));

    let peripherals = SDLVirt{ run_flag: run_flag.clone(), framebuf: framebuf.clone() };
    
    crossbeam::scope(|scope| {
        let thr = scope.spawn(|| {
            engine::run(peripherals);
        });
        
        'main: loop {
            for event in events.poll_iter() {
                match event {                
                    Event::Quit {..} => break 'main,
                    
                    Event::KeyDown {keycode: Some(keycode), ..} => {
                        if keycode == Keycode::Escape {
                            break 'main
                        }
                        // else if keycode == Keycode::Space {
                        //     println!("space down");
                        //     for i in 0..400 {
                        //         renderer.pixel(i as i16, i as i16, 0xFF000FFu32).unwrap();
                        //     }
                        //     renderer.present();
                        // }
                    }
                    
                    _ => {}
                }
            };
            
            {
                let mut screen_surface = window.surface_mut(&events).unwrap();
                let framebuf = framebuf.lock().unwrap();
                draw_lcd(&framebuf, draw_surface);
                draw_surface.blit_scaled(None, &mut screen_surface, None).unwrap();
            };
            window.update_surface().unwrap();
            timer.delay(17); // 60 FPS
        }

        run_flag.store(false, Ordering::Relaxed);
        thr.join();
    });
}

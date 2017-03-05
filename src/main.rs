use std::mem::transmute;
use std::thread;
use std::sync::{Arc,Mutex};
use std::sync::atomic::{Ordering,AtomicBool};
use std::time::Duration;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color,PixelFormat,PixelFormatEnum};
use sdl2::surface::{Surface,SurfaceRef};

extern crate crossbeam;
use crossbeam::Scope;

const SCREEN_WIDTH : u32 = 84;
const SCREEN_HEIGHT : u32 = 48;
const SCALE : u32 = 4;

type FrameBuf = [[bool; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];

const COLOR_ON       : Color = Color::RGB(0x00, 0x00, 0x00);
const COLOR_ON_GRID  : Color = Color::RGB(0x20, 0x38, 0x20);
const COLOR_OFF      : Color = Color::RGB(0x73, 0xbd, 0x71);
const COLOR_OFF_GRID : Color = Color::RGB(0x63, 0xad, 0x61);

fn draw_lcd(framebuf: &FrameBuf, surface: &mut SurfaceRef) {
    let pixel_format = surface.pixel_format();

    surface.with_lock_mut(|flat| {
        let pixbuf: &mut [u32] = unsafe{ transmute(flat) };
        
        for (y, rowi) in framebuf.iter().enumerate() {
            for (x, pxi) in rowi.iter().enumerate() {
                for i in 0..SCALE {
                    for j in 0..SCALE {
                        let grid_y = i == 0 || i == SCALE - 1;
                        let grid_x = j == 0 || j == SCALE - 1;
                        
                        pixbuf[(((y as u32 * SCALE + i) * SCREEN_WIDTH * SCALE) + (x as u32 * SCALE + j)) as usize] =
                            if grid_x || grid_y {              
                                if *pxi {COLOR_ON_GRID} else {COLOR_OFF_GRID}
                            } else {
                                if *pxi {COLOR_ON} else {COLOR_OFF}
                            }.to_u32(&pixel_format);
                    }
                }
            }
        }
    });
}

trait Peripherals {
    fn keep_running(&self) -> bool;
    fn set_pixel(&self, u8, u8, bool) -> ();
}

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

fn run_engine<P>(peripherals: P)
    where P: Peripherals
{
    let mut x = 0;
    
    while peripherals.keep_running() {
        peripherals.set_pixel(x, 0, false);
        x = x + 1;
        if x == SCREEN_WIDTH as u8 { x = 0 };
        peripherals.set_pixel(x, 0, true);
        thread::sleep(Duration::from_millis(500));
    };
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut timer = sdl.timer().unwrap();

    let vidsys = sdl.video().unwrap();
    let mut window = vidsys.window("RUST-8", SCREEN_WIDTH * 8, SCREEN_HEIGHT * 8)
        // .position_centered()
        // .opengl()
        .build()
        .unwrap();

    let mut events = sdl.event_pump().unwrap();

    let pixel_format = window.surface(&events).unwrap().pixel_format();
    
    let framebuf = Arc::new(Mutex::new([[false; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize]));
    let ref mut draw_surface = Surface::new(SCREEN_WIDTH * SCALE, SCREEN_HEIGHT * SCALE, PixelFormatEnum::from(pixel_format)).unwrap();
       
    let run_flag = Arc::new(AtomicBool::new(true));

    let peripherals = SDLVirt{ run_flag: run_flag.clone(), framebuf: framebuf.clone() };
    
    crossbeam::scope(|scope| {
        let thr = scope.spawn(|| {
            run_engine(peripherals);
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

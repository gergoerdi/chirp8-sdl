use std::sync::*;
use std::sync::atomic::{Ordering,AtomicBool};

extern crate sdl2;

use sdl2::keyboard::{KeyboardState, Scancode};

extern crate chirp8_engine as chirp8;
use chirp8::prelude::*;
use chirp8::peripherals::*;
use chirp8::cpu::CPU;

use lcd::*;

type KeyBuf = u16;
type RAM = [Byte; 1 << 12];

#[derive(Clone)]
pub struct SDLVirt {
    framebuf: Arc<Mutex<FrameBuf>>,
    redraw: Arc<AtomicBool>,
    run_flag: Arc<AtomicBool>,
    key_state: Arc<Mutex<KeyBuf>>,
    ram: Arc<Mutex<RAM>>,
}

impl SDLVirt {
    pub fn tick(&self, cpu: &mut CPU) {
        cpu.tick_frame()
    }

    pub fn process_keys(&self, key_state: KeyboardState) {
        let mut keys = self.key_state.lock().unwrap();
        *keys = 0;
        for (i, key) in [ Scancode::X,
                          Scancode::Num1, Scancode::Num2, Scancode::Num3,
                          Scancode::Q, Scancode::W, Scancode::E,
                          Scancode::A, Scancode::S, Scancode::D,

                          Scancode::Z, Scancode::C, Scancode::Num4, Scancode::R, Scancode::F, Scancode::V
                        ].iter().enumerate() {
            if key_state.is_scancode_pressed(*key) {
                *keys |= 1 << i;
            }
        }
    }

    // fn blit(&self, draw_surface: &SurfaceRef, dest: &mut SurfaceRef) {
    //     let ref framebuf = self.framebuf.lock().unwrap();
    //     draw_lcd(framebuf, dest);
    //     draw_surface.blit_scaled(None, dest, None).unwrap();
    // }

    pub fn new() -> SDLVirt {
        SDLVirt {
            run_flag: Arc::new(AtomicBool::new(true)),
            framebuf: Arc::new(Mutex::new([[false; LCD_WIDTH as usize]; LCD_HEIGHT as usize])),
            redraw: Arc::new(AtomicBool::new(true)),
            key_state: Arc::new(Mutex::new(0)),
            ram: Arc::new(Mutex::new([0; 1 << 12])),
        }
    }

    pub fn keep_running(&self) -> bool {
        self.run_flag.load(Ordering::Relaxed)
    }

    pub fn stop_running(&self) {
        self.run_flag.store(false, Ordering::Relaxed);
    }

    pub fn take_redraw(&self) -> bool {
        self.redraw.swap(false, Ordering::Relaxed)
    }

    pub fn lock_framebuf(&self) -> LockResult<MutexGuard<FrameBuf>> {
        self.framebuf.lock()
    }
}

impl Peripherals for SDLVirt {
    fn set_pixel_row(&mut self, y: ScreenY, mut row: ScreenRow) {
        let frame_row = &mut self.framebuf.lock().unwrap()[(y + 8) as usize];
        for x in 0..64 {
            let v = row & 1 != 0;
            row >>= 1;
            frame_row[(63-x + 10) as usize] = v;
        }
    }

    fn get_pixel_row(&self, y: ScreenY) -> ScreenRow {
        let frame_row = self.framebuf.lock().unwrap()[(y + 8) as usize];
        let mut row = 0;
        for x in 0..64 {
            row <<= 1;
            row |= frame_row[(x + 10) as usize] as u64;
        }
        row
    }

    fn get_keys(&self) -> u16 {
        *self.key_state.lock().unwrap()
    }

    fn redraw(&mut self) {
        self.redraw.store(true, Ordering::Relaxed);
    }

    fn read_ram(&self, addr: Addr) -> Byte {
        self.ram.lock().unwrap()[addr as usize]
    }

    fn write_ram(&mut self, addr: Addr, val: Byte) {
        self.ram.lock().unwrap()[addr as usize] = val;
    }

    fn set_sound(&mut self, _val: Byte) {
        // TODO
    }
}

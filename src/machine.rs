extern crate sdl2;

use sdl2::keyboard::{KeyboardState, Scancode};

extern crate chirp8_engine as chirp8;
use chirp8::prelude::*;
use chirp8::peripherals::*;
use chirp8::cpu::{CPU, Quirks};

use lcd::*;

type KeyBuf = u16;
type RAM = [Byte; 1 << 12];

#[derive(Clone)]
pub struct SDLVirt {
    framebuf: FrameBuf,
    key_buf: KeyBuf,
    ram: RAM,
}

impl SDLVirt {
    pub fn tick<Q: Quirks>(&self, cpu: &mut CPU<Q>) {
        cpu.tick_frame()
    }

    pub fn process_keys(&mut self, key_state: KeyboardState) {
        self.key_buf = 0;
        for (i, key) in [ Scancode::X,
                          Scancode::Num1, Scancode::Num2, Scancode::Num3,
                          Scancode::Q, Scancode::W, Scancode::E,
                          Scancode::A, Scancode::S, Scancode::D,

                          Scancode::Z, Scancode::C, Scancode::Num4, Scancode::R, Scancode::F, Scancode::V
                        ].iter().enumerate() {
            if key_state.is_scancode_pressed(*key) {
                self.key_buf |= 1 << i;
            }
        }
    }

    pub fn new() -> SDLVirt {
        SDLVirt {
            framebuf: [[false; LCD_WIDTH as usize]; LCD_HEIGHT as usize],
            key_buf: 0,
            ram: [0; 1 << 12],
        }
    }

    pub fn get_framebuf(&self) -> FrameBuf {
        self.framebuf
    }
}

impl Peripherals for SDLVirt {
    fn set_pixel_row(&mut self, y: ScreenY, mut row: ScreenRow) {
        let frame_row = &mut self.framebuf[(y + 8) as usize];
        for x in 0..64 {
            let v = row & 1 != 0;
            row >>= 1;
            frame_row[(63-x + 10) as usize] = v;
        }
    }

    fn get_pixel_row(&self, y: ScreenY) -> ScreenRow {
        let frame_row = self.framebuf[(y + 8) as usize];
        let mut row = 0;
        for x in 0..64 {
            row <<= 1;
            row |= frame_row[(x + 10) as usize] as u64;
        }
        row
    }

    fn get_keys(&self) -> u16 {
        self.key_buf
    }

    fn read_ram(&self, addr: Addr) -> Byte {
        self.ram[addr as usize]
    }

    fn write_ram(&mut self, addr: Addr, val: Byte) {
        self.ram[addr as usize] = val;
    }

    fn set_sound(&mut self, _val: Byte) {
        // TODO
    }
}

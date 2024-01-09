extern crate sdl2;

use sdl2::keyboard::{KeyboardState, Scancode};

extern crate chirp8_engine as chirp8;
use chirp8::prelude::*;
use chirp8::peripherals::*;
use chirp8::cpu::CPU;

use chirp8::graphics::lcd::*;

type KeyBuf = u16;
type RAM = [Byte; 1 << 12];

#[derive(Clone)]
pub struct SDLVirt {
    framebuf: FrameBuf,
    key_buf: KeyBuf,
    ram: RAM,
}

impl SDLVirt {
    pub fn tick(&self, cpu: &mut CPU) {
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
            framebuf: [0; SCREEN_HEIGHT as usize],
            key_buf: 0,
            ram: [0; 1 << 12],
        }
    }

    pub fn get_framebuf(&self) -> FrameBuf {
        self.framebuf
    }
}

impl Peripherals for SDLVirt {
    fn set_pixel_row(&mut self, y: ScreenY, row: ScreenRow) {
        self.framebuf[y as usize] = row;
    }

    fn get_pixel_row(&self, y: ScreenY) -> ScreenRow {
        self.framebuf[y as usize]
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

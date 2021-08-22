use sdl2::pixels::{Color,PixelFormat,PixelFormatEnum};
use sdl2::surface::{Surface,SurfaceRef};

use std::mem::transmute;

use chip8::peripherals::*;

const SCALE : u32 = 8;

pub const LCD_WIDTH : u8 = SCREEN_WIDTH + 20;
pub const LCD_HEIGHT : u8 = SCREEN_HEIGHT + 16;

pub type FrameBuf = [[bool; LCD_WIDTH as usize]; LCD_HEIGHT as usize];

const COLOR_ON       : Color = Color::RGB(0x00, 0x00, 0x00);
const COLOR_ON_GRID  : Color = Color::RGB(0x20, 0x38, 0x20);
const COLOR_OFF      : Color = Color::RGB(0x73, 0xbd, 0x71);
const COLOR_OFF_GRID : Color = Color::RGB(0x63, 0xad, 0x61);

pub fn draw_lcd(framebuf: &FrameBuf, surface: &mut SurfaceRef) {
    let pixel_format = surface.pixel_format();

    surface.with_lock_mut(|flat| {
        let pixbuf: &mut [u32] = unsafe{ transmute(flat) };

        for (y, rowi) in framebuf.iter().enumerate() {
            for (x, pxi) in rowi.iter().enumerate() {
                for i in 0..SCALE {
                    for j in 0..SCALE {
                        let grid_y = i == 0 || i == SCALE - 1;
                        let grid_x = j == 0 || j == SCALE - 1;

                        pixbuf[(((y as u32 * SCALE + i) * LCD_WIDTH as u32 * SCALE) + (x as u32 * SCALE + j)) as usize] =
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

pub fn new_draw_surface<'a> (pixel_format: PixelFormat) -> Surface<'a> {
    Surface::new(
        LCD_WIDTH as u32 * SCALE,
        LCD_HEIGHT as u32 * SCALE,
        PixelFormatEnum::from(pixel_format))
        .unwrap()
}

use sdl2::video::Window;
use sdl2::render::*;
use sdl2::pixels::*;

use std::mem::transmute;

use chirp8::prelude::*;

const SCALE : u32 = 8;

pub const LCD_WIDTH : u8 = SCREEN_WIDTH + 20;
pub const LCD_HEIGHT : u8 = SCREEN_HEIGHT + 16;

pub type FrameBuf = [[bool; LCD_WIDTH as usize]; LCD_HEIGHT as usize];

const COLOR_ON       : u32 = 0x00_00_00;
const COLOR_ON_GRID  : u32 = 0x20_38_20;
const COLOR_OFF      : u32 = 0x73_bd_71;
const COLOR_OFF_GRID : u32 = 0x63_ad_61;

const PIX_WIDTH  : u32 = SCALE * LCD_WIDTH as u32;
const PIX_HEIGHT : u32 = SCALE * LCD_HEIGHT as u32;

type PixBuf = [u32; PIX_WIDTH as usize * PIX_HEIGHT as usize];

pub fn draw_lcd(framebuf: &FrameBuf, canvas: &mut Canvas<Window>) {
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture(PixelFormatEnum::RGB888, TextureAccess::Streaming, PIX_WIDTH, PIX_HEIGHT)
        .unwrap();

    let mut pixbuf: PixBuf = [0x00_00_00; PIX_WIDTH as usize * PIX_HEIGHT as usize];
    for (y, rowi) in framebuf.iter().enumerate() {
        for (x, pxi) in rowi.iter().enumerate() {
            for i in 0..SCALE {
                for j in 0..SCALE {
                    let grid_y = i == 0 || i == SCALE - 1;
                    let grid_x = j == 0 || j == SCALE - 1;

                    pixbuf[(((y as u32 * SCALE + i) * PIX_WIDTH) + (x as u32 * SCALE + j)) as usize] =
                        if grid_x || grid_y {
                            if *pxi {COLOR_ON_GRID} else {COLOR_OFF_GRID}
                        } else {
                            if *pxi {COLOR_ON} else {COLOR_OFF}
                        }
                }
            }
        }
    }

    let pixbuf_bytes: [u8; PIX_WIDTH as usize * PIX_HEIGHT as usize * 4] = unsafe{ transmute(pixbuf) };

    texture.update(None, &pixbuf_bytes, PIX_WIDTH as usize * 4).unwrap();
    canvas.clear();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
}

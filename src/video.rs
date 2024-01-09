use sdl2::video::Window;
use sdl2::render::*;
use sdl2::pixels::*;

use std::mem::transmute;

use chirp8::prelude::*;
use chirp8::graphics::lcd::*;

const SCALE : usize = 8;
const PAD_X: usize = 10;
const PAD_Y: usize = 8;

pub const PIX_WIDTH  : u32 = SCALE as u32 * (SCREEN_WIDTH as u32 + 2 * PAD_X as u32);
pub const PIX_HEIGHT : u32 = SCALE as u32 * (SCREEN_HEIGHT as u32 + 2 * PAD_Y as u32);

pub fn render_lcd(framebuf: &FrameBuf, canvas: &mut Canvas<Window>) {
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture(PixelFormatEnum::RGB888, TextureAccess::Streaming, PIX_WIDTH, PIX_HEIGHT)
        .unwrap();

    let mut pixbuf: [u32; PIX_WIDTH as usize * PIX_HEIGHT as usize] = [0x00_00_00_00; PIX_WIDTH as usize * PIX_HEIGHT as usize];
    draw_lcd(framebuf, &mut pixbuf, (SCALE, SCALE), (PAD_X, PAD_Y));
    let pixbuf_bytes: [u8; PIX_WIDTH as usize * PIX_HEIGHT as usize * 4] = unsafe{ transmute(pixbuf) };

    texture.update(None, &pixbuf_bytes, PIX_WIDTH as usize * 4).unwrap();
    canvas.clear();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
}

use std::mem::transmute;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormat;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

const SCREEN_WIDTH : u32 = 84;
const SCREEN_HEIGHT : u32 = 48;

type FrameBuf = [[bool; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];
type PixBuf = [[u32; (SCREEN_WIDTH * 8) as usize]; (SCREEN_HEIGHT * 8) as usize];

const COLOR_ON       : Color = Color::RGB(0x00, 0x00, 0x00);
const COLOR_ON_GRID  : Color = Color::RGB(0x20, 0x38, 0x20);
const COLOR_OFF      : Color = Color::RGB(0x73, 0xbd, 0x71);
const COLOR_OFF_GRID : Color = Color::RGB(0x63, 0xad, 0x61);

fn draw_framebuf(framebuf: &FrameBuf, pixbuf: &mut PixBuf, pixel_format: &PixelFormat) {
    for (y, rowi) in framebuf.iter().enumerate() {
        for (x, pxi) in rowi.iter().enumerate() {
            for i in 0..8 {
                for j in 0..8 {
                    let grid_y = !(2 <= i && i <= 5);
                    let grid_x = !(2 <= j && j <= 5);
                    
                    pixbuf[y * 8 + i][x * 8 + j] =
                        if grid_x || grid_y {              
                            if *pxi {COLOR_ON_GRID} else {COLOR_OFF_GRID}
                        } else {
                            if *pxi {COLOR_ON} else {COLOR_OFF}
                        }.to_u32(&pixel_format);
                }
            }
        }
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut timer = sdl.timer().unwrap();

    let vidsys = sdl.video().unwrap();
    let mut window = vidsys.window("RUST_8", SCREEN_WIDTH * 8, SCREEN_HEIGHT * 8)
        // .position_centered()
        // .opengl()
        .build()
        .unwrap();

    let mut events = sdl.event_pump().unwrap();

    let mut framebuf : FrameBuf = [[false; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];
    let mut pixbuf : PixBuf = [[0; (SCREEN_WIDTH * 8) as usize]; (SCREEN_HEIGHT * 8) as usize];
    let pixel_format = {
        let screen_surface = window.surface(&events).unwrap();
        screen_surface.pixel_format()
    };

    let draw_surface = {
        let flat : &mut [u8; (4 * SCREEN_WIDTH * SCREEN_HEIGHT) as usize] = unsafe { transmute(&mut pixbuf) };
        Surface::from_data(
            flat, SCREEN_WIDTH * 8, SCREEN_HEIGHT * 8, SCREEN_WIDTH * 8 * 4, PixelFormatEnum::from(pixel_format)).unwrap()
    };

    
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
            let pixel_format = screen_surface.pixel_format();
            
            draw_framebuf(&framebuf, &mut pixbuf, &pixel_format);
            draw_surface.blit(None, &mut screen_surface, None).unwrap();
        };
        window.update_surface().unwrap();
        timer.delay(17); // 60 FPS
    }
}

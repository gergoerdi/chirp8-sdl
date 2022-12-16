extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

extern crate crossbeam;

extern crate chirp8_engine as chirp8;

mod engine;
mod machine;
mod lcd;

use machine::*;
use lcd::*;

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut events = sdl.event_pump().unwrap();

    let mut sdltimer = sdl.timer().unwrap();

    let vidsys = sdl.video().unwrap();
    let mut window = vidsys.window("CHIRP-8", LCD_WIDTH as u32 * 8, LCD_HEIGHT as u32 * 8)
        // .position_centered()
        .build()
        .unwrap();

    let ref mut draw_surface = lcd::new_draw_surface(window.surface(&events).unwrap().pixel_format());

    let virt = SDLVirt::new();

    crossbeam::scope(|scope| {
        scope.spawn(|| {
            engine::run(virt.clone());
        });

        'main: loop {
            for event in events.poll_iter() {
                match event {
                    Event::Quit {..} => break 'main,
                    Event::KeyDown {keycode: Some(keycode), ..} if keycode == Keycode::Escape => break 'main,
                    _ => {}
                }
            };

            virt.process_keys(events.keyboard_state());

            if virt.take_redraw() {
                let mut screen_surface = window.surface_mut(&events).unwrap();

                // virt.blit(&draw_surface, &mut screen_surface);

                let ref framebuf = virt.lock_framebuf().unwrap();
                draw_lcd(framebuf, draw_surface);
                draw_surface.blit_scaled(None, &mut screen_surface, None).unwrap();
            };
            window.update_surface().unwrap();

            virt.tick();

            sdltimer.delay(17); // 60 FPS
        }

        virt.stop_running();
    });
}

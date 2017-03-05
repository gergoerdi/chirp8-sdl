use peripherals::*;
use chip8::ops::*;

use std::thread;
use std::time::Duration;

pub fn run<P>(peripherals: P)
    where P: Peripherals
{
    let mut x = 0;

    while peripherals.keep_running() {
        for y in 0..4 {
            peripherals.set_pixel(x, y, false);
        }

        x = x + 1;
        if x == SCREEN_WIDTH as u8 { x = 0 };

        for y in 0..4 {
            let row = peripherals.scan_key_row(y);
            if row != 0 {
                peripherals.set_pixel(x, y, true);
            }
        }

        peripherals.redraw();

        thread::sleep(Duration::from_millis(50));
    };
}

use peripherals::*;

use std::thread;
use std::time::Duration;

pub fn run<P>(peripherals: P)
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

pub const SCREEN_WIDTH : u8 = 84;
pub const SCREEN_HEIGHT : u8 = 48;

pub trait Peripherals {
    fn keep_running(&self) -> bool;

    fn clear_pixels(&self);
    fn set_pixel(&self, u8, u8, bool);
    fn get_pixel(&self, u8, u8) -> bool;

    fn scan_key_row(&self, u8) -> u8;

    fn set_timer(&self, u8);
    fn get_timer(&self) -> u8;
}

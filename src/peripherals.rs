pub const SCREEN_WIDTH : u8 = 84;
pub const SCREEN_HEIGHT : u8 = 48;

pub trait Peripherals {
    fn keep_running(&self) -> bool;
    fn set_pixel(&self, u8, u8, bool) -> ();
}

use chirp8::peripherals::*;
use chirp8::prelude::*;
use chirp8::font::*;

use std::fs::File;
use std::io::Read;

pub fn setup<P>(file_name: &std::path::PathBuf, io: &mut P)
    where P: Peripherals
{
    for (addr, b) in FONT_HEX.iter().enumerate() {
        io.write_ram(addr as Addr, *b);
    }

    let mut file = File::open(file_name).unwrap();
    let mut buf = [0; 4 * 2 << 10];
    let mut ptr = 0x0200;
    'load: loop {
        let read_bytes = file.read(&mut buf[..]).unwrap();

        if read_bytes == 0 {
            break 'load;
        }

        for offset in 0..read_bytes {
            io.write_ram(ptr + offset as Addr, buf[offset]);
        }
        ptr += read_bytes as Addr;
    }
}

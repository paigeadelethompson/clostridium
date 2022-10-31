use std::fs::File;
use std::io::{Read};
use crate::memory::{LOCATIONS_BEGIN, LOCATIONS_END, Memory};

pub enum ROM_SIZES {
    M5XX_ROM_SIZE = (4 * 0x100000),
    M500_ROM_SIZE = (8 * 0x100000),
    M515_ROM_SIZE = (16 * 0x100000)
}

pub fn load_ROM(mem: &mut Memory) {
    let mut f = File::open("rom/Palm-m515-4.1-en.rom").unwrap();
    let mut buf: Vec<u8> = vec![];

    f.read_to_end(&mut buf).unwrap();

    for (index, byte_data) in
    std::iter::zip(LOCATIONS_BEGIN::ROM as usize..LOCATIONS_END::ROM as usize, buf) {
        mem.memory.insert(index, byte_data);
    }
}
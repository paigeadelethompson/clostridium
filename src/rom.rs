use std::error::Error;
use std::fs::File;
use std::io::{Read};

pub enum ROM_SIZES {
    M5XX_ROM_SIZE = (4 * 0x100000),
    M500_ROM_SIZE = (8 * 0x100000),
    M515_ROM_SIZE = (16 * 0x100000)
}

pub enum ROM_BASE {
    VERSION_3_ROM_BASE = 0x10C00000,
    VERSION_4_ROM_BASE = 0x10000000
}

pub struct Rom {
    pub(crate) memory: Box<[u8]>
}

pub fn load_ROM() -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buf: Vec<u8> = vec![];

    match File::open("rom/Palm-m515-4.1-en.rom") {
        Ok(mut f) => {
            match f.read_to_end(&mut buf) {
                Ok(_) => {
                    Ok(buf.to_vec())
                },
                Err(e) => Err(Box::new(e))
            }
        },
        Err(e) => Err(Box::new(e))
    }
}
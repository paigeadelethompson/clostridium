use std::error::Error;
use sdl2::video::Window;
use sdl2::VideoSubsystem;
use crate::memory::{LOCATION_RAM_BEGIN, LOCATION_RAM_END, LOCATION_SED1376_FB_BEGIN, LOCATION_SED1376_FB_END, LOCATION_SED1376_REGISTERS_BEGIN, LOCATION_SED1376_REGISTERS_END};

pub struct Display {
    frame_buffer: Box<[u8]>,
    window: Window,
    registers: Box<[u8]>
}

pub fn init_video() -> Result<Display, Box<dyn Error>> {
    let d = sdl2::init()?
        .video()?
        .window("PalmOS", 100, 100)
        .resizable()
        .build()?;

    Ok(Display {
        frame_buffer: vec![0; (LOCATION_SED1376_FB_END - LOCATION_SED1376_FB_BEGIN) as usize].into_boxed_slice(),
        registers: vec![0; (LOCATION_SED1376_REGISTERS_END - LOCATION_SED1376_REGISTERS_BEGIN) as usize].into_boxed_slice(),
        window: d
    })
}
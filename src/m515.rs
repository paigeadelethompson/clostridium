use sdl2::video::FullscreenType::True;
use crate::bootloader::load_boot_loader;
use crate::cpu::initialize_cpu;
use crate::memory::{initialize_memory, Memory, LOCATIONS_BEGIN};
use crate::rom::load_ROM;

pub fn start() {
    let mut memory = initialize_memory();
    load_ROM(&mut memory);
    load_boot_loader(&mut memory);
    let mut cpu = initialize_cpu(&mut memory);

    loop {
        cpu.interpreter(&mut memory);
    }
}
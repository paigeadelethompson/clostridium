extern crate core;

mod display;
mod m515;
mod memory;
mod rom;
mod bootloader;
mod cpu;
mod dram;
mod sram;

fn main() {
    m515::start()
}

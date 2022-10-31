mod display;
mod m515;
mod memory;
mod rom;
mod bootloader;
mod cpu;

fn main() {
    m515::start()
}

#![allow(arithmetic_overflow)]

use crate::cpu::initialize_cpu;
use crate::memory::{initialize_memory};

pub fn start() {
    let mut memory = match initialize_memory() {
        Ok(m) => m,
        Err(e) => panic!("failed to initialize memory {}", e.to_string())
    };

    let mut cpu = initialize_cpu();

    loop {
        println!("PC {:#08x} ", cpu.regs.pc);
        println!("cycles {}", cpu.interpreter(&mut memory));
    }
}
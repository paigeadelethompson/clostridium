use m68000::M68000;
use crate::memory::{Memory};

pub fn initialize_cpu (mem: &mut Memory) -> M68000 {
    let mut cpu = M68000::new();
    return cpu;
}
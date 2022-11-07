use m68000::M68000;

pub fn initialize_cpu () -> M68000 {
    let mut cpu = M68000::new();
    return cpu;
}
use std::borrow::Borrow;
use std::ptr;
use std::slice::Iter;
use m68000::MemoryAccess;
use stable_vec::StableVec;

pub enum LOCATIONS_BEGIN {
    ROM = 0x10000000,
    RAM = 0x00000000,
    SED1376_REGISTERS = 0x1FF80000,
    SED1376_FB = (0x1FF80000 + 0x20000),
    HW_REGISTERS = 0xFFFFF000,
    BOOTLOADER = 0xFFFFFE00,
    USB = 0x10400000 
}

pub enum LOCATIONS_END {
    ROM = 0x103FFFFF,
    RAM = 0x00FFFFFF,
    SED1376_REGISTERS = 0x1FF800B3,
    SED1376_FB = 0x1FFB3FFF,
    HW_REGISTERS = 0xFFFFFDFF,
    BOOTLOADER = 0xFFFFFFFF,
    USB = 0x10400003 
}

const MEM_SIZE: usize = 0xffffffff;

pub struct Memory {
    pub memory: StableVec<u8>
}

impl MemoryAccess for Memory {
    fn get_byte(&mut self, addr: u32) -> Option<u8> {
        Some(self.memory[addr as usize])
    }

    fn get_word(&mut self, addr: u32) -> Option<u16> {
        Some(u16::from_be_bytes([self.memory[addr as usize],
            self.memory[(addr + 1) as usize]]))
    }

    fn set_byte(&mut self, addr: u32, value: u8) -> Option<()> {
        self.memory[addr as usize] = value;
        Some(())
    }

    fn set_word(&mut self, addr: u32, value: u16) -> Option<()> {
        let d: &mut Vec<u8> = &mut value.to_be_bytes().to_vec();
        write_bytes(d, addr,  &mut self.memory);
        Some(())
    }

    fn reset_instruction(&mut self) {
        panic!()
    }
}

pub fn write_bytes(data: &mut Vec<u8>, offset: u32, mem: &mut StableVec<u8>) {
    for index in 0..data.len() {
        mem[offset as usize + index] = data[index]
    }
}

pub fn zero_memory(start: LOCATIONS_BEGIN, end: LOCATIONS_END, mem: &mut Memory) {
    for index in start as usize .. end as usize {
        mem.memory.insert(index, 0x0);
    }
}

pub fn initialize_memory()  -> Memory {
    let mut mem: Memory = Memory {
        memory: StableVec::with_capacity(MEM_SIZE)
    };

    zero_memory(LOCATIONS_BEGIN::RAM, LOCATIONS_END::RAM, &mut mem);
    zero_memory(LOCATIONS_BEGIN::ROM, LOCATIONS_END::ROM, &mut mem);
    zero_memory(LOCATIONS_BEGIN::SED1376_REGISTERS, LOCATIONS_END::SED1376_REGISTERS, &mut mem);
    zero_memory(LOCATIONS_BEGIN::SED1376_FB, LOCATIONS_END::SED1376_FB, &mut mem);
    zero_memory(LOCATIONS_BEGIN::HW_REGISTERS, LOCATIONS_END::HW_REGISTERS, &mut mem);
    zero_memory(LOCATIONS_BEGIN::BOOTLOADER, LOCATIONS_END::BOOTLOADER, &mut mem);

    let mut initial_ssp: Vec<u8> = Vec::new();


    initial_ssp.push(0x4E);
    initial_ssp.push(0xF9);
    initial_ssp.push(0x00);
    initial_ssp.push(0x10);
    initial_ssp.push(0x00);
    initial_ssp.push(0x00);

    write_bytes(&mut initial_ssp, 0x00, &mut mem.memory);



    return mem
}
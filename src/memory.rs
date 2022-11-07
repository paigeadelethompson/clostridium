use std::error::Error;
use m68000::assembler::add;
use m68000::MemoryAccess;
use crate::dram::Dram;
use crate::rom::{Rom, ROM_BASE};
use crate::sram::Sram;
use crate::rom::load_ROM;
use crate::bootloader::{Bootloader, init_boot_loader};
use crate::display::{Display, init_video};

pub const LOCATION_ROM_BEGIN: u32 = 0x10000000;
pub const LOCATION_RAM_BEGIN: u32 = 0x00000000;
pub const LOCATION_SED1376_REGISTERS_BEGIN: u32 = 0x1FF80000;
pub const LOCATION_SED1376_FB_BEGIN: u32 = 0x1FF80000 + 0x20000;
pub const LOCATION_HW_REGISTERS_BEGIN: u32 = 0xFFFFF000;
pub const LOCATION_BOOTLOADER_BEGIN: u32 = 0xFFFFFE00;
pub const LOCATION_USB_BEGIN: u32 = 0x10400000;

pub const LOCATION_ROM_END: u32 = 0x103FFFFF;
pub const LOCATION_RAM_END: u32 = 0x00FFFFFF;
pub const LOCATION_SED1376_REGISTERS_END: u32 = 0x1FF800B3;
pub const LOCATION_SED1376_FB_END: u32 = 0x1FFB3FFF;
pub const LOCATION_HW_REGISTERS_END: u32 = 0xFFFFFDFF;
pub const LOCATION_BOOTLOADER_END: u32 = 0xFFFFFFFF;
pub const LOCATION_USB_END: u32 = 0x10400003;

pub struct Memory {
    DRAM: Dram,
    ROM: Rom,
    SRAM: Sram,
    BOOTLOADER: Bootloader,
    VIDEO: Display
}

impl MemoryAccess for Memory {
    fn get_byte(&mut self, addr: u32) -> Option<u8> {
        Some(if addr >= LOCATION_RAM_BEGIN && addr <= LOCATION_RAM_END {
            let mask = addr - LOCATION_RAM_BEGIN;
            println!("read {:#08x} from RAM {:#08x}", self.DRAM.memory[mask as usize], addr);
            self.DRAM.memory[mask as usize]
        }
        else if addr >= LOCATION_ROM_BEGIN && addr <= LOCATION_ROM_END {
            let mask = addr - LOCATION_ROM_BEGIN;
            println!("read {:#08x} from ROM {:#08x}", self.ROM.memory[mask as usize], addr);
            self.ROM.memory[mask as usize]
        }
        else if addr >= LOCATION_HW_REGISTERS_BEGIN  && addr <= LOCATION_HW_REGISTERS_END {
            todo!("read from hw registers {:#08x}", addr)
        }
        else if addr >= LOCATION_BOOTLOADER_BEGIN && addr <= LOCATION_BOOTLOADER_END {
            let mask = addr - LOCATION_BOOTLOADER_BEGIN;
            println!("read {:#08x} from bootloader {:#08x}", self.BOOTLOADER.memory[mask as usize], addr);
            self.BOOTLOADER.memory[mask as usize]
        }
        else {
            //todo!("read from unknown memory range {:#08x}", addr)
            print!("out of range {:#08x}", addr);
            0
        })
    }

    fn get_word(&mut self, addr: u32) -> Option<u16> {
        let ret = [self.get_byte(addr).unwrap(), self.get_byte(addr + 1).unwrap()];
        println!("read word {:#08x} from offset {:#08x}", u16::from_be_bytes(ret), addr);
        Some(u16::from_be_bytes(ret))
    }

    fn set_byte(&mut self, addr: u32, value: u8) -> Option<()> {
        let _ = if addr >= LOCATION_RAM_BEGIN && addr <= LOCATION_RAM_END {
            let mask = addr - LOCATION_RAM_BEGIN;
            println!("wrote {:#08x} to RAM {:#08x}", value, mask);
            self.DRAM.memory[mask as usize] = value;
        }
        else if addr >= LOCATION_ROM_BEGIN && addr <= LOCATION_ROM_END {
            let mask = addr - LOCATION_ROM_BEGIN;
            println!("wrote {:#08x} to ROM? {:#08x}", value, mask);
            self.ROM.memory[mask as usize] = value;
        }
        else if addr >= LOCATION_HW_REGISTERS_BEGIN  && addr <= LOCATION_HW_REGISTERS_END {
            todo!("write to HW registers {:#08x}", addr)
        }
        else if addr >= LOCATION_BOOTLOADER_BEGIN && addr <= LOCATION_BOOTLOADER_END {
            println!("wrote {:#08x} to bootloader? {:#08x}", value, addr);
            let mask = addr - LOCATION_BOOTLOADER_BEGIN;
            self.BOOTLOADER.memory[mask as usize] = value;
        }
        else {
            //todo!("write to unknown memory range {:#08x}", addr)
            print!("out of range {:#08x}", addr);
            ()
        };
        Some(())
    }

    fn set_word(&mut self, addr: u32, value: u16) -> Option<()> {
        let b = value.to_be_bytes().to_vec();
        self.set_byte(addr, b[0])?;
        self.set_byte(addr + 1, b[1])?;
        Some(())
    }

    fn reset_instruction(&mut self) {
        panic!()
    }
}

fn initialize_ssp(mem: &mut Memory) {
    let mut entry: Vec<u8> = [0x4e, 0xf9].to_vec();
    let mut jump_addr: Vec<u8> = (ROM_BASE::VERSION_4_ROM_BASE as u32).to_be_bytes().to_vec();

    entry.append(&mut jump_addr);

    for index in 0..entry.len() {
        mem.DRAM.memory[index] = entry[index];
    }
}

pub fn initialize_memory() -> Result<Memory, Box<dyn Error>> {
    let rom = load_ROM()?;

    let mut ret = Memory {
        DRAM: Dram {
            memory: vec![0; (LOCATION_RAM_END - LOCATION_RAM_BEGIN) as usize].into_boxed_slice()
        },
        ROM: Rom {
            memory: rom.into_boxed_slice(),
        },
        SRAM: Sram {
            memory: vec![0; (LOCATION_RAM_END - LOCATION_RAM_BEGIN) as usize].into_boxed_slice()
        },
        BOOTLOADER: init_boot_loader(),
        VIDEO: init_video()?
    };

    initialize_ssp(&mut ret);

    Ok(ret)
}
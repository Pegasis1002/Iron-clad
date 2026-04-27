use crate::{bus::ram::write_mtime, models::bus::BUS};
use crate::bus::mmio;
use crate::bus::ram::clint;

impl BUS {
    #[inline(always)]
    pub(crate) fn write(&mut self, addr: u32, data: u32, size: usize) {

        // Key board check
        if addr == mmio::MMIO_KEY {
            self.key_buff = data;
            return;
        }

        // CLINT check
        if addr >= clint::CLINT_START  && addr <= clint::CLINT_END {
            clint(self, addr, data);
            return;
        }

        // UART check
        if addr == mmio::MMIO_UART {
            print!("{}", (data & 0xFF) as u8 as char);
            use std::io::{self, Write};
            let _ = io::stdout().flush();
            return; // Don't try to write this to RAM!
        }

        // VRAM
        if addr >= mmio::MMIO_VRAM_START && addr < mmio::MMIO_VRAM_END {
            let index = ((addr - mmio::MMIO_VRAM_START) >> 2) as usize;
            if index < self.vram.len() {
                self.vram[index] = data;
            }
            return;
        }

        if addr < mmio::MMIO_RAM_START {
            println!("Memory Access Error: Address {:#X} is below RAM start.", addr);
            return;
        }

        let bytes = data.to_le_bytes();
        let index = (addr - mmio::MMIO_RAM_START) as usize;
        if index + 3 >= self.ram.len() {
            println!("Memory Access Error: Address {:#X} is out of RAM bounds.", addr);
            return;
        }

        for i in 0..size {
            if index + i < self.ram.len() {
                self.ram[index + i] = bytes[i];
            }
        }

    }
}

fn clint(bus: &mut BUS, addr: u32, val: u32) {
    match addr {
        // Handle mtimecmp halves
        clint::CLINT_MTIMECMP_L => bus.mtimecmp = (bus.mtimecmp & !0xFFFFFFFF) | (val as u64),
        clint::CLINT_MTIMECMP_H => bus.mtimecmp = (bus.mtimecmp & 0x00000000FFFFFFFF) | ((val as u64) << 32),
        
        // Handle mtime halves
        clint::CLINT_MTIME_L => bus.mtime = (bus.mtime & !0xFFFFFFFF) | (val as u64),
        clint::CLINT_MTIME_H => bus.mtime = (bus.mtime & 0x00000000FFFFFFFF) | ((val as u64) << 32),
        
        _ => {}
    }
}

use crate::{bus::ram::write_mtime, models::bus::BUS};

impl BUS {
    #[inline(always)]
    pub(crate) fn write(&mut self, addr: u32, data: u32, size: usize) {

        // CLINT check
        if addr >= 0x0200_0000  && addr <= 0x0200_FFFF {
            clint(self, addr, data);
            return;
        }

        // UART check
        if addr == 0x1000_0000 {
            print!("{}", (data & 0xFF) as u8 as char);
            use std::io::{self, Write};
            let _ = io::stdout().flush();
            return; // Don't try to write this to RAM!
        }

        // VRAM
        if addr >= 0x1100_0000 && addr < 0x1104_B000 {
            let index = ((addr - 0x1100_0000) >> 2) as usize;
            if index < self.vram.len() {
                self.vram[index] = data;
            }
            return;
        }

        if addr < 0x8000_0000 {
            println!("Memory Access Error: Address {:#X} is below RAM start.", addr);
            return;
        }

        let bytes = data.to_le_bytes();
        let index = (addr - 0x8000_0000) as usize;
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
        0x0200_4000 => bus.mtimecmp = (bus.mtimecmp & !0xFFFFFFFF) | (val as u64),
        0x0200_4004 => bus.mtimecmp = (bus.mtimecmp & 0x00000000FFFFFFFF) | ((val as u64) << 32),
        
        // Handle mtime halves
        0x0200_BFF8 => bus.mtime = (bus.mtime & !0xFFFFFFFF) | (val as u64),
        0x0200_BFFC => bus.mtime = (bus.mtime & 0x00000000FFFFFFFF) | ((val as u64) << 32),
        
        _ => {}
    }
}

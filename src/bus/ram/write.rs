use crate::models::bus::BUS;

impl BUS {
    #[inline(always)]
    pub(crate) fn write(&mut self, addr: u32, data: u32, size: usize) {

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

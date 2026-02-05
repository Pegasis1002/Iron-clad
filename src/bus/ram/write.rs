use crate::models::bus::BUS;

impl BUS {
    pub(crate) fn write(&mut self, addr: u32, data: u32) {
        let bytes = data.to_le_bytes();

        // VRAM check
        if addr >= 0x1100_0000 && addr < 0x1104_B000 {
            let index = ((addr - 0x1100_0000) / 4) as usize;
            if index < self.vram.len() {
                self.vram[index] = data;
            }
                println!("VRAM Write detected! Color: {:#010X}", data);
            return;
        }

        // UART check
        if addr == 0x1000_0000 {
            print!("{}", (data & 0xFF) as u8 as char);
            use std::io::{self, Write};
            let _ = io::stdout().flush();
            return; // Don't try to write this to RAM!
        }

        if addr < 0x8000_0000 {
            println!("Memory Access Error: Address {:#X} is below RAM start.", addr);
            return;
        }

        let index = (addr - 0x8000_0000) as usize;
        if index + 3 >= self.ram.len() {
            println!("Memory Access Error: Address {:#X} is out of RAM bounds.", addr);
            return;
        }

        self.ram[index] = bytes[0];
        self.ram[index + 1] = bytes[1];
        self.ram[index + 2] = bytes[2];
        self.ram[index + 3] = bytes[3];
    }
}

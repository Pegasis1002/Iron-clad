use crate::bus::BUS;
use crate::bus::ram::clint::{CLINT_START, CLINT_END, CLINT_MTIME_H, CLINT_MTIME_L, CLINT_MTIMECMP_H, CLINT_MTIMECMP_L};
use crate::bus::mmio;

impl BUS{
    pub(crate) fn read(&self, addr: u32, bytes: usize) -> u32 {
        // UART
        if addr == 0x1000_0060 {
            return self.key_buff;
        }

        // CLINT
        if addr >= CLINT_START && addr <= CLINT_END {
            return match addr {
                CLINT_MTIME_L => (self.mtime & 0xFFFFFFFF) as u32,       // Low 32 bits
                CLINT_MTIME_H => (self.mtime >> 32) as u32,              // High 32 bits
                CLINT_MTIMECMP_L => (self.mtimecmp & 0xFFFFFFFF) as u32,    // Low 32 bits
                CLINT_MTIMECMP_H => (self.mtimecmp >> 32) as u32,           // High 32 bits
                _ => 0,
            };
        }

        // RAM Lower bound check
        if addr < mmio::MMIO_RAM_START {
            println!("memory access error: address {:#x} is below ram start.", addr);
            return 0x0;
        }
        if bytes > 4 {
            println!("Bytes cannot be more than 4. bytes = {}", bytes);
            return 0x0;
        }

        let index = (addr - mmio::MMIO_RAM_START) as usize;
        if index + bytes > self.ram.len() {
            println!("memory access error: address {:#x} is out of ram bounds.", addr);
            return 0x0;
        }

        let mut buff: [u8; 4] = [0u8; 4];
        for i in 0..bytes {
            buff[i] = self.ram[index + i];
        }

        u32::from_le_bytes(buff)
    }
}

use crate::bus::BUS;

impl BUS{
    pub(crate) fn read(&self, addr: u32, bytes: usize) -> u32 {
        if addr >= 0x0200_0000 && addr <= 0x0200_BFFF {
            return match addr {
                0x0200_BFF8 => (self.mtime & 0xFFFFFFFF) as u32,       // Low 32 bits
                0x0200_BFFC => (self.mtime >> 32) as u32,              // High 32 bits
                0x0200_4000 => (self.mtimecmp & 0xFFFFFFFF) as u32,    // Low 32 bits
                0x0200_4004 => (self.mtimecmp >> 32) as u32,           // High 32 bits
                _ => 0,
            };
        }

        if addr < 0x8000_0000 {
            println!("memory access error: address {:#x} is below ram start.", addr);
            return 0x0;
        }
        if bytes > 4 {
            println!("Bytes cannot be more than 4. bytes = {}", bytes);
            return 0x0;
        }

        let index = (addr - 0x8000_0000) as usize;
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

use crate::bus::BUS;

impl BUS {
    pub(crate) fn read_x64(&self, addr: u32, bytes: usize) -> u64 {
        if addr < 0x8000_0000 {
            println!("memory access error: address {:#x} is below ram start.", addr);
            return 0x0;
        }
        if bytes > 8 {
            println!("Bytes cannot be more than 8. bytes = {}", bytes);
            return 0x0;
        }

        let index = (addr - 0x8000_0000) as usize;
        if index + bytes > self.ram.len() {
            println!("memory access error: address {:#x} is out of ram bounds.", addr);
            return 0x0;
        }

        let mut buff: [u8; 8] = [0u8; 8];
        for i in 0..bytes {
            buff[i] = self.ram[index + i];
        }

        u64::from_le_bytes(buff)
    }
}

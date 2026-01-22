use crate::bus::BUS;

impl BUS{
    pub(crate) fn read(&self, addr: u32, bytes: usize) -> u32 {
        if addr < 0x8000_0000 {
            println!("memory access error: address {:#x} is below ram start.", addr);
            return 0x0;
        }
        if bytes > 4 {
            println!("Bytes cannot be more than 4. bytes = {}", bytes);
            return 0x0;
        }

        let mut index = (addr - 0x8000_0000) as usize;
        if index + bytes >= self.ram.len() {
            println!("memory access error: address {:#x} is out of ram bounds.", addr);
            return 0x0;
        }

        let mut buff = vec![0u8; 4];
        for i in 0..bytes {
            buff[i] = self.ram[index + 1];
        }

        u32::from_le_bytes(buff);
    }
}

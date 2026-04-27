use crate::models::bus::BUS;
use crate::bus::mmio;

impl BUS {
    pub(crate) fn read_word( &self, addr: u32) -> u32 {
        if addr < mmio::MMIO_RAM_START {
            println!("Memory Access Error: Address {:#X} is below RAM start.", addr);
            return 0x0;
        }

        let index = (addr - mmio::MMIO_RAM_START) as usize;
        if index + 3 >= self.ram.len() {
            println!("Memory Access Error: Address {:#X} is out of RAM bounds.", addr);
            return 0x0;
        }

        let b0 = self.ram[index] as u32;
        let b1 = self.ram[index + 1] as u32;
        let b2 = self.ram[index + 2] as u32;
        let b3 = self.ram[index + 3] as u32;

        return (b3 << 24) | (b2 << 16) | (b1 << 8) | b0;
    }
}

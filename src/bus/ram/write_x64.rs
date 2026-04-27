use crate::bus::BUS;
use crate::bus::mmio;

impl BUS {
    pub(crate) fn write_x64(bus: &mut Self, addr: u32, value: u64) {
        if addr < mmio::MMIO_RAM_START {
            println!("Memory Access Error: Address {:#X} is below RAM start.", addr);
            return;
        }

        let bytes = value.to_le_bytes();
        let index = (addr - mmio::MMIO_RAM_START) as usize;
        let size = 8;
        if index + size >= bus.ram.len() {
            println!("Memory Access Error: Address {:#X} is out of RAM bounds.", addr);
            return;
        }

        for i in 0..size {
            if index + i < bus.ram.len() {
                bus.ram[index + i] = bytes[i];
            }
        }
    }
}

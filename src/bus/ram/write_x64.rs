use crate::bus::BUS;

impl BUS {
    pub(crate) fn write_x64(bus: &mut Self, addr: u32, value: u64) {
        if addr < 0x8000_0000 {
            println!("Memory Access Error: Address {:#X} is below RAM start.", addr);
            return;
        }

        let bytes = value.to_le_bytes();
        let index = (addr - 0x8000_0000) as usize;
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

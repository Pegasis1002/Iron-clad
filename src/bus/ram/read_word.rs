use crate::models::bus::BUS;

impl BUS {
    pub(crate) fn read_word( &self, addr: u32) -> u32 {
        let addr = addr as usize;

        let b0 = self.ram[addr] as u32;
        let b1 = self.ram[addr + 1] as u32;
        let b2 = self.ram[addr + 2] as u32;
        let b3 = self.ram[addr + 3] as u32;

        return (b3 << 24) | (b2 << 16) | (b1 << 8) | b0;
    }
}

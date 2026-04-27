use crate::models::bus::BUS;

pub mod ram;
pub mod mmio;

const RAM_SIZE: usize = 128 * 1024 * 1024;

impl BUS {
    pub fn new() -> Self {
        return Self {
            ram: vec![0; RAM_SIZE],
            vram: vec![0; 320 * 240],
            mtime: 0x0,
            mtimecmp: 0x0,
            key_buff: 0x0,
        }
    }
}

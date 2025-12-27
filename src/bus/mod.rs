use crate::models::bus::BUS;

pub mod ram;

const RAM_SIZE: usize = 128 * 1024 * 1024;

impl BUS {
    pub fn new() -> Self {
        return Self {
            ram: vec![0; RAM_SIZE],
        }
    }
}

pub mod step;

use crate::models::bus::BUS;
use crate::models::cpu::CPU;

impl CPU {
    pub fn new(bus: BUS) -> Self{
        return Self {
            pc: 0x8000_0000,
            reg: [0; 32],
            bus,
        }
    }
}

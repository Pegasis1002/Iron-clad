use crate::models::bus::BUS;
use crate::models::cpu::CPU;

const KERNEL_START: u32 = 0x80000000;

impl CPU {
    pub fn new(bus: BUS) -> Self{
        return Self {
            pc: KERNEL_START,
            reg: [0; 32],
            bus,
        }
    }
}

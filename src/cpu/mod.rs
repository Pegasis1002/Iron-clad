pub mod step;
pub mod decode;
pub mod exec;

use crate::models::bus::BUS;
use crate::models::cpu::CPU;
use crate::models::cpu::Mode;

impl CPU {
    pub fn new(bus: BUS, mode: Mode) -> Self{
        let mut csr = [0; 4096];

        // Set MISA to show I and M extensions are implemented;
        csr[0x301] = (1 << 30) | (1 << 8) | (1 << 12);

        return Self {
            pc: 0x8000_0000,
            reg: [0; 32],
            bus,
            mode,
            csr,
        }
    }
}

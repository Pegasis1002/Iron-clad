pub mod step;
pub mod decode;
pub mod exec;
pub mod csr;

use crate::models::bus::BUS;
use crate::models::cpu::CPU;
use crate::models::cpu::Mode;

impl CPU {
    pub fn new(bus: BUS, mode: Mode) -> Self{
        let mut reg = [0; 32];
        reg[2] = 0x8000_0000 + (128 * 1024 * 1024);

        // Init CSR and Set MISA to show I and M extensions are implemented;
        let mut csr = [0; 4096];
        csr[0x301] = (1 << 30) | (1 << 8) | (1 << 12);

        return Self {
            pc: 0x8000_0000,
            reg,
            bus,
            mode,
            csr,
            cycles: 0x0,
        }
    }
}

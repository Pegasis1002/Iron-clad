use crate::models::cpu::CPU;
use crate::cpu::decode::DecodeInst;

mod imm;
mod lui;
mod reg;
mod store;

use imm::exec_imm;
use lui::exec_lui;
use reg::exec_reg;
use store::exec_store;

impl CPU {
    pub(crate) fn execute(&mut self, inst: DecodeInst) {
        match inst.op_code {
            0x13 => exec_imm(self, inst),
            0x33 => exec_reg(self, inst),
            0x37 => exec_lui(self, inst),
            0x23 => exec_store(self, inst),
            0x63 => {},
            0x6F => {},
            _ => panic!("UnImplemented opcode: {:#010X}",inst.op_code),
        }
    }
}

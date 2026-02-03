use crate::models::cpu::CPU;
use crate::cpu::decode::DecodeInst;

mod imm;
mod lui;
mod reg;
mod store;
mod load;
mod auipc;
mod branch;
mod jal;
mod jalr;
mod system;

use imm::exec_imm;
use lui::exec_lui;
use reg::exec_reg;
use store::exec_store;
use load::exec_load;
use auipc::exec_auipc;
use branch::exec_branch;
use jal::exec_jal;
use jalr::exec_jalr;
use system::exec_system;

impl CPU {
    pub(crate) fn execute(&mut self, inst: DecodeInst) {
        match inst.op_code {
            0x3 => exec_load(self, inst),
            0x13 => exec_imm(self, inst),
            0x17 => exec_auipc(self, inst),
            0x33 => exec_reg(self, inst),
            0x37 => exec_lui(self, inst),
            0x23 => exec_store(self, inst),
            0x63 => exec_branch(self, inst),
            0x6F => exec_jal(self, inst),
            0x67 => exec_jalr(self, inst),
            0x73 => exec_system(self, inst),
            _ => panic!("UnImplemented opcode: {:#010X}",inst.op_code),
        }
    }
}

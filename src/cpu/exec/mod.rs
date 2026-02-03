use crate::models::cpu::CPU;
use crate::cpu::decode::DecodeInst;

mod imm;
mod lui;
mod reg;
mod store;
mod load;
mod auipc;

use imm::exec_imm;
use lui::exec_lui;
use reg::exec_reg;
use store::exec_store;
use load::exec_load;
use auipc::exec_auipc;

impl CPU {
    pub(crate) fn execute(&mut self, inst: DecodeInst) {
        println!("{:#010X}", inst.op_code);
        match inst.op_code {
            0x3 => exec_load(self, inst),
            0x13 => exec_imm(self, inst),
            0x17 => {
                exec_auipc(self, inst);
                println!("hehehe")
            },
            0x33 => exec_reg(self, inst),
            0x37 => exec_lui(self, inst),
            0x23 => exec_store(self, inst),
            0x63 => {},
            0x6F => {},
            _ => panic!("UnImplemented opcode: {:#010X}",inst.op_code),
        }
    }
}

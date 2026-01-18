use crate::models::cpu::CPU;
use crate::cpu::decode::DecodeInst;

pub(crate) fn exec_lui(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        cpu.reg[rd] = (inst.imm << 12) as u32;
        println!("r{} = {:#010X}", rd, cpu.reg[rd])
    }
}

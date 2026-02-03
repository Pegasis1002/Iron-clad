use crate::{cpu::decode::DecodeInst, models::cpu::CPU};

pub(crate) fn exec_jal(cpu: &mut CPU, inst: DecodeInst) {
    let link_addr = cpu.pc + 4;

    if inst.rd != 0 {
        cpu.reg[inst.rd as usize] = link_addr;
    }

    let target_addr = ((cpu.pc as i32 + inst.imm) as u32).wrapping_sub(4);

    if target_addr % 2 != 0{
        println!("ERROR: Target address missaligned!");
        return;
    }

    cpu.pc = target_addr;
    println!("JAL jump to {:#010X}", target_addr);
    println!("r{} = {:#010X}", inst.rd as usize, cpu.reg[inst.rd as usize]);
}

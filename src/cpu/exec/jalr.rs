use crate::{cpu::decode::DecodeInst, models::cpu::CPU};

pub(crate) fn exec_jalr(cpu: &mut CPU, inst: DecodeInst) {
    let addr = ((inst.imm + cpu.reg[inst.rs1 as usize] as i32) as u32) & 0xFFFFFFFE;
    let link_addr = cpu.pc + 4;

    if inst.rd != 0 {
        cpu.reg[inst.rd as usize] = link_addr;
    }

    if addr % 4 != 0{
        println!("ERROR: Target address missaligned!");
        return;
    }

    cpu.pc = addr.wrapping_sub(4);
    println!("JALR jump to {:#010X}", addr);
    println!("r{} = {:#010X}", inst.rd as usize, cpu.reg[inst.rd as usize]);
}

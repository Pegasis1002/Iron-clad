use crate::{cpu::decode::DecodeInst, models::cpu::CPU};

pub(crate) fn exec_auipc(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd == 0 {
        println!("Error: rd cannot be 0.");
        return;
    }
    let offset: u32 = ((inst.imm << 12) + cpu.pc as i32) as u32;
    cpu.reg[rd] = offset;
    println!("r{} = {:#010X}", rd, cpu.reg[rd]);
}

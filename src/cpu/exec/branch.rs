use crate::{cpu::decode::DecodeInst, models::cpu::CPU};

pub(crate) fn exec_branch(cpu: &mut CPU, inst: DecodeInst) {
    match inst.funct3 {
        0x0 => beq(cpu, inst),
        0x1 => bne(cpu, inst),
        0x4 => blt(cpu, inst),
        0x5 => bge(cpu, inst),
        0x7 => bgeu(cpu, inst),
        0x6 => bltu(cpu, inst),

        _ => panic!("Invalid B-type funct3 for Imm: {}", inst.funct3),
    }
}

fn beq(cpu: &mut CPU, inst: DecodeInst) {
    let addr = (inst.imm + (cpu.pc as i32)) as u32;
    let rs1 = inst.rs1 as usize;
    let rs2 = inst.rs2 as usize;

    if cpu.reg[rs1] == cpu.reg[rs2] {
        cpu.pc = addr.wrapping_sub(4);
        println!("Branch jump to {:#010X}", addr);
    }
}

fn bne(cpu: &mut CPU, inst: DecodeInst) {
    let addr = (inst.imm + (cpu.pc as i32)) as u32;
    let rs1 = inst.rs1 as usize;
    let rs2 = inst.rs2 as usize;

    if cpu.reg[rs1] != cpu.reg[rs2] {
        cpu.pc = addr.wrapping_sub(4);
        println!("Branch jump to {:#010X}", addr);
    }
}

fn blt(cpu: &mut CPU, inst: DecodeInst) {
    let addr = (inst.imm + (cpu.pc as i32)) as u32;
    let rs1 = inst.rs1 as usize;
    let rs2 = inst.rs2 as usize;

    if (cpu.reg[rs1] as i32) < (cpu.reg[rs2] as i32){
        cpu.pc = addr.wrapping_sub(4);
        println!("Branch jump to {:#010X}", addr);
    }
}

fn bltu(cpu: &mut CPU, inst: DecodeInst) {
    let addr = (inst.imm + (cpu.pc as i32)) as u32;
    let rs1 = inst.rs1 as usize;
    let rs2 = inst.rs2 as usize;

    if (cpu.reg[rs1] as u32) < (cpu.reg[rs2] as u32){
        cpu.pc = addr.wrapping_sub(4);
        println!("Branch jump to {:#010X}", addr);
    }
}

fn bge(cpu: &mut CPU, inst: DecodeInst) {
    let addr = (inst.imm + (cpu.pc as i32)) as u32;
    let rs1 = inst.rs1 as usize;
    let rs2 = inst.rs2 as usize;

    if (cpu.reg[rs1] as i32) >= (cpu.reg[rs2] as i32){
        cpu.pc = addr.wrapping_sub(4);
        println!("Branch jump to {:#010X}", addr);
    }
}

fn bgeu(cpu: &mut CPU, inst: DecodeInst) {
    let addr = (inst.imm + (cpu.pc as i32)) as u32;
    let rs1 = inst.rs1 as usize;
    let rs2 = inst.rs2 as usize;

    if (cpu.reg[rs1] as u32) >= (cpu.reg[rs2] as u32){
        cpu.pc = addr.wrapping_sub(4);
        println!("Branch jump to {:#010X}", addr);
    }
}

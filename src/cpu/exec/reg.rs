use crate::cpu::decode::DecodeInst;
use crate::models::cpu::CPU;

pub(crate) fn exec_reg(cpu: &mut CPU, inst: DecodeInst){
    match inst.funct3 {
        0x0 => match inst.funct7 {
            0x0 => add(cpu, inst),
            0x20 => sub(cpu, inst),
            _ => panic!("Invalid R-type funct7: {}", inst.funct7),
        },
        0x7 => and(cpu, inst),
        0x6 => or(cpu, inst),
        _ => panic!("Invalid R-type funct3 for Imm: {}", inst.funct3),
    }
}

fn add(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1 = inst.rs1 as usize;
        let rs2 = inst.rs2 as usize;
        cpu.reg[rd] = cpu.reg[rs1] + cpu.reg[rs2];
        println!("r{} = {}", rd, cpu.reg[rd])
    }
}

fn sub(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1 = inst.rs1 as usize;
        let rs2 = inst.rs2 as usize;

        cpu.reg[rd] = cpu.reg[rs1] - cpu.reg[rs2];
        println!("r{} = {}", rd, cpu.reg[rd])
    }
}

fn and(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1 = inst.rs1 as usize;
        let rs2 = inst.rs2 as usize;

        cpu.reg[rd] = cpu.reg[rs1] & cpu.reg[rs2];
        println!("r{} = {}", rd, cpu.reg[rd])
    }
}

fn or(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1 = inst.rs1 as usize;
        let rs2 = inst.rs2 as usize;

        cpu.reg[rd] = cpu.reg[rs1] | cpu.reg[rs2];
        println!("r{} = {}", rd, cpu.reg[rd])
    }
}

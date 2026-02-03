use crate::cpu::decode::DecodeInst;
use crate::models::cpu::CPU;
use crate::cpu::exec::m_ext::exec_m_ext;

pub(crate) fn exec_reg(cpu: &mut CPU, inst: DecodeInst){
    if inst.funct7 == 0x1 {
        exec_m_ext(cpu, inst);
        return;
    }

    match inst.funct3 {
        0x0 => match inst.funct7 {
            0x0 => add(cpu, inst),
            0x20 => sub(cpu, inst),
            _ => panic!("Invalid R-type funct7: {}", inst.funct7),
        },
        0x1 => sll(cpu, inst),
        0x2 => slt(cpu, inst),
        0x3 => sltu(cpu, inst),
        0x4 => xor(cpu, inst),
        0x5 => {
            match inst.funct7 {
                0x0 => slr(cpu, inst),
                0x20 => sla(cpu, inst),
                _ => panic!("Invalid R-type funct7: {}", inst.funct7),
            }
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
        //cpu.reg[rd] = (cpu.reg[rs1] as i32 + cpu.reg[rs2] as i32) as u32;
        cpu.reg[rd] = cpu.reg[rs1].wrapping_add(cpu.reg[rs2]);
        // println!("r{} = {}", rd, cpu.reg[rd])
    }
}

fn sub(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1 = inst.rs1 as usize;
        let rs2 = inst.rs2 as usize;

        //cpu.reg[rd] = cpu.reg[rs1] - cpu.reg[rs2];
        cpu.reg[rd] = cpu.reg[rs1].wrapping_sub(cpu.reg[rs2]);
        // println!("r{} = {}", rd, cpu.reg[rd])
    }
}

fn slt(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1 = inst.rs1 as usize;
        let rs2 = inst.rs2 as usize;
        if (cpu.reg[rs1] as i32) < (cpu.reg[rs2] as i32) {
            cpu.reg[rd] = 0x1;
        } else {
            cpu.reg[rd] = 0x0;
        }
        // println!("r{} = {}", rd, cpu.reg[rd])
    }
}

fn sltu(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1 = inst.rs1 as usize;
        let rs2 = inst.rs2 as usize;
        if (cpu.reg[rs1] as u32) < (cpu.reg[rs2] as u32) {
            cpu.reg[rd] = 0x1;
        } else {
            cpu.reg[rd] = 0x0;
        }
        // println!("r{} = {}", rd, cpu.reg[rd])
    }
}

fn sll(cpu: &mut CPU, inst: DecodeInst){
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1 = inst.rs1 as usize;
        let rs2 = inst.rs2 as usize;

        cpu.reg[rd] = (cpu.reg[rs1] << (cpu.reg[rs2] & 0x1F)) as u32;
        // println!("r{} = {}", rd, cpu.reg[rd])
    }
}

fn slr(cpu: &mut CPU, inst: DecodeInst){
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1 = inst.rs1 as usize;
        let rs2 = inst.rs2 as usize;

        cpu.reg[rd] = (cpu.reg[rs1] >> (cpu.reg[rs2] & 0x1F)) as u32;
        // println!("r{} = {}", rd, cpu.reg[rd])
    }
}

fn sla(cpu: &mut CPU, inst: DecodeInst){
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1 = inst.rs1 as usize;
        let rs2 = inst.rs2 as usize;

        cpu.reg[rd] = ((cpu.reg[rs1] as i32) >> (cpu.reg[rs2] as i32 & 0x1F)) as u32;
        // println!("r{} = {}", rd, cpu.reg[rd])
    }
}

fn and(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1 = inst.rs1 as usize;
        let rs2 = inst.rs2 as usize;

        cpu.reg[rd] = cpu.reg[rs1] & cpu.reg[rs2];
        // println!("r{} = {}", rd, cpu.reg[rd])
    }
}

fn or(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1 = inst.rs1 as usize;
        let rs2 = inst.rs2 as usize;

        cpu.reg[rd] = cpu.reg[rs1] | cpu.reg[rs2];
        // println!("r{} = {}", rd, cpu.reg[rd])
    }
}

fn xor(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1 = inst.rs1 as usize;
        let rs2 = inst.rs2 as usize;

        cpu.reg[rd] = cpu.reg[rs1] ^ cpu.reg[rs2];
        // println!("r{} = {}", rd, cpu.reg[rd])
    }
}

use crate::cpu::decode::DecodeInst;
use crate::models::cpu::CPU;

pub(crate) fn exec_imm(cpu: &mut CPU, inst: DecodeInst){
    match inst.funct3 {
        0x0 => addi(cpu, inst),
        0x2 => slti(cpu, inst),
        0x3 => sltiu(cpu, inst),
        0x4 => xori(cpu, inst),
        0x6 => ori(cpu, inst),
        0x7 => andi(cpu, inst),
        0x1 => slli(cpu, inst),
        0x5 => {
            match inst.imm >> 5 {
                0x0 => srli(cpu, inst),
                0x20 => srai(cpu, inst),
                _ => panic!("Invalid I-type funct7 for Imm: {}", inst.funct3)
            }
        },
        _ => panic!("Invalid I-type funct3 for Imm: {}", inst.funct3),
    }
}

fn addi(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        // This will panic if a large number is added
        // cpu.reg[rd] = ((cpu.reg[inst.rs1 as usize] as i32) + inst.imm) as u32;
        //
        // This will not do that, and instead wraparoud like the hardware does
        let rs1_val = cpu.reg[inst.rs1 as usize] as i32;
        cpu.reg[rd] = rs1_val.wrapping_add(inst.imm) as u32;
        // println!("r{} = {:#010X}", rd, cpu.reg[rd]);
    }
}

fn xori(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1_val = cpu.reg[inst.rs1 as usize] as i32;
        cpu.reg[rd] = (rs1_val ^ inst.imm) as u32;
        // println!("r{} = {:#010X}", rd, cpu.reg[rd]);
    }
}

fn slti(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1_val = cpu.reg[inst.rs1 as usize] as i32;
        cpu.reg[rd] = (rs1_val < inst.imm) as u32;

        // println!("r{} = {:#010X}", rd, cpu.reg[rd]);
    }
}

fn sltiu(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1_val = cpu.reg[inst.rs1 as usize] as u32;
        cpu.reg[rd] = (rs1_val < inst.imm as u32) as u32;

        // println!("r{} = {:#010X}", rd, cpu.reg[rd]);
    }
}

fn ori(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1_val = cpu.reg[inst.rs1 as usize] as i32;
        cpu.reg[rd] = (rs1_val | inst.imm) as u32;
        // println!("r{} = {:#010X}", rd, cpu.reg[rd]);
    }
}

fn andi(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1_val = cpu.reg[inst.rs1 as usize] as i32;
        cpu.reg[rd] = (rs1_val & inst.imm) as u32;
        // println!("r{} = {:#010X}", rd, cpu.reg[rd]);
    }
}

fn slli(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1_val = cpu.reg[inst.rs1 as usize] as i32;
        let shamt = inst.imm & 0x1F;
        cpu.reg[rd] = (rs1_val << shamt) as u32;
        // println!("r{} = {:#010X}", rd, cpu.reg[rd]);
    }
}

fn srli(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1_val = cpu.reg[inst.rs1 as usize] as u32;
        let shamt = (inst.imm & 0x1F) as u32;
        cpu.reg[rd] = rs1_val >> shamt;
        // println!("r{} = {:#010X}", rd, cpu.reg[rd]);
    }
}

fn srai(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd != 0 {
        let rs1_val = cpu.reg[inst.rs1 as usize] as i32;
        let shamt = inst.imm & 0x1F as i32;
        cpu.reg[rd] = (rs1_val >> shamt) as u32;
        // println!("r{} = {:#010X}", rd, cpu.reg[rd]);
    }
}

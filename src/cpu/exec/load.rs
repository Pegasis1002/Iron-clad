use crate::cpu::decode::DecodeInst;
use crate::models::cpu::CPU;

pub fn exec_load(cpu: &mut CPU, inst: DecodeInst) {
    match inst.funct3 {
        0x0 => lb(cpu, inst),
        0x1 => lh(cpu, inst),
        0x2 => lw(cpu, inst),
        0x4 => lbu(cpu, inst),
        0x5 => lhu(cpu, inst),
        _ => println!("Invalid I-type funct3 for imm: {:010X}", inst.imm),
    }
}

fn lb(cpu: &mut CPU, inst: DecodeInst) {
    let rd = (inst.rd) as usize;
    if rd != 0 {
        let addr = (cpu.reg[inst.rs1 as usize] as i32 + inst.imm) as u32;
        let data = (cpu.bus.read(addr, 1) & 0xFF) as u8;

        let se_data: i32 = ((data as i32) << 24 ) >> 24;
        cpu.reg[rd] = se_data as u32;
        // println!("r{:?} = {:#010X}", rd, cpu.reg[rd]);
    } else {
        println!("Error: rd cannot be x0");
    }
}

fn lh(cpu: &mut CPU, inst: DecodeInst) {
    let rd = (inst.rd) as usize;
    if rd != 0 {
        let addr = (cpu.reg[inst.rs1 as usize] as i32 + inst.imm) as u32;
        let data = (cpu.bus.read(addr, 2) & 0xFFFF) as u16;

        let se_data: i32 = ((data as i32) << 16) >> 16;
        cpu.reg[rd] = se_data as u32;
        // println!("r{:?} = {:#010X}", rd, cpu.reg[rd]);
    } else {
        println!("Error: rd cannot be x0");
    }
}

fn lw(cpu: &mut CPU, inst: DecodeInst) {
    let rd = (inst.rd) as usize;
    if rd != 0 {
        let addr = (cpu.reg[inst.rs1 as usize] as i32 + inst.imm) as u32;
        let data = cpu.bus.read(addr, 4);

        cpu.reg[rd] = data as u32;
        // println!("r{:?} = {:#010X}", rd, cpu.reg[rd]);
    } else {
        println!("Error: rd cannot be x0");
    }
}

fn lbu(cpu: &mut CPU, inst: DecodeInst) {
    let rd = (inst.rd) as usize;
    if rd != 0{
        let addr = (cpu.reg[inst.rs1 as usize] as i32 + inst.imm) as u32;
        let data = (cpu.bus.read(addr, 4) & 0xFF) as u8;

        cpu.reg[rd] = data as u32;
        // println!("r{:?} = {:#010X}", rd, cpu.reg[rd]);
    } else {
        println!("Error: rd cannot be x0");
    }
}

fn lhu(cpu: &mut CPU, inst: DecodeInst) {
    let rd = (inst.rd) as usize;
    if rd != 0{
        let addr = (cpu.reg[inst.rs1 as usize] as i32 + inst.imm) as u32;
        let data = (cpu.bus.read(addr, 4) & 0xFFFF) as u16;

        cpu.reg[rd] = data as u32;
        // println!("r{:?} = {:#010X}", rd, cpu.reg[rd]);
    } else {
        println!("Error: rd cannot be x0");
    }
}

use crate::cpu::decode::DecodeInst;
use crate::models::bus::BUS;
use crate::models::cpu::CPU;

pub(crate) fn exec_store(cpu: &mut CPU, inst: DecodeInst){
    match inst.funct3 {
        0x0 => sb(cpu, inst),
        0x1 => sh(cpu, inst),
        0x2 => sw(cpu, inst),
        _ => panic!("Invalid S-type funct3: {}", inst.funct3)
    }
}

fn sw(cpu: &mut CPU, inst: DecodeInst) {
    let rs1_val = cpu.reg[inst.rs1 as usize];
    let rs2 = inst.rs2 as usize;

    let addr = (inst.imm + rs1_val as i32) as u32;
    let data = cpu.reg[rs2] as u32;

    BUS::write(&mut cpu.bus, addr, data);
    // println!("r{} = {:#010X}", rs2, cpu.reg[rs2 as usize]);
}

fn sh(cpu: &mut CPU, inst: DecodeInst) {
    let rs1_val = cpu.reg[inst.rs1 as usize];
    let rs2 = inst.rs2 as usize;

    let addr = (inst.imm + rs1_val as i32) as u32;
    let data = (cpu.reg[rs2] & 0xFFFF) as u32;

    BUS::write(&mut cpu.bus, addr, data);
    // println!("r{} = {:#010X}", rs2, cpu.reg[rs2]);
}

fn sb(cpu: &mut CPU, inst: DecodeInst) {
    let rs1_val = cpu.reg[inst.rs1 as usize];
    let rs2 = inst.rs2 as usize;

    let addr = (inst.imm + rs1_val as i32) as u32;
    let data = (cpu.reg[rs2] & 0xFF) as u32;

    BUS::write(&mut cpu.bus, addr, data);
    // println!("r{} = {:#010X}", rs2, cpu.reg[rs2]);
}

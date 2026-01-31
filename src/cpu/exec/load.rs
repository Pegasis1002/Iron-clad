use crate::cpu::decode::DecodeInst;
use crate::models::cpu::CPU;

pub fn exec_load(cpu: &mut CPU, inst: DecodeInst) {
    match inst.funct3 {
        0x0 => lb(cpu, inst),
        0x1 => lh(cpu, inst),
        0x2 => lw(cpu, inst),
        0x3 => lbu(cpu, inst),
        0x4 => lhu(cpu, inst),
        _ => println!("Invalid I-type funct3 for imm: {:?}", inst.imm),
    }
}

fn lb(cpu: &mut CPU, inst: DecodeInst) {
    let addr = (cpu.reg[inst.rs1 as usize] as i32 + inst.imm) as u32;
    let data = cpu.bus.read(addr, 1);
}
fn lh(cpu: &mut CPU, inst: DecodeInst) {}
fn lw(cpu: &mut CPU, inst: DecodeInst) {
    // Load whole imm into reg from memory
}
fn lbu(cpu: &mut CPU, inst: DecodeInst) {}
fn lhu(cpu: &mut CPU, inst: DecodeInst) {}

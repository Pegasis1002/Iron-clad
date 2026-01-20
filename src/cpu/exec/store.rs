use crate::cpu::decode::DecodeInst;
use crate::models::bus::BUS;
use crate::models::cpu::CPU;

pub(crate) fn exec_store(cpu: &mut CPU, inst: DecodeInst){
    match inst.funct3 {
        0x2 => sw(cpu, inst),
        _ => panic!("Invalid S-type funct3: {}", inst.funct3)
    }
}

fn sw(cpu: &mut CPU, inst: DecodeInst) {
    let rs1_val = cpu.reg[inst.rs1 as usize];
    let addr = (inst.imm + rs1_val as i32) as u32;
    let data = inst.imm as u32;

    BUS::write(&mut cpu.bus, addr, data);
}

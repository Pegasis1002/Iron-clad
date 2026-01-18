use crate::cpu::decode::DecodeInst;
use crate::models::cpu::CPU;

pub(crate) fn exec_store(cpu: &mut CPU, inst: DecodeInst){
    match inst.funct3 {
        0x2 => sw(cpu, inst),
        _ => panic!("Invalid S-type funct3: {}", inst.funct3)
    }
}

fn sw(cpu: &mut CPU, inst: DecodeInst) {
    let rs1_val = cpu.reg[inst.rs1 as usize];
    let addr = inst.imm + rs1_val as i32;

    println!("Memory at {:#010X} = {:#010X}", addr as u32, cpu.bus.ram[addr as usize])
}

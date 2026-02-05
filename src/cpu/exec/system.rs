use crate::{cpu::decode::DecodeInst, models::cpu::{CPU, Mode}};

pub(crate) fn exec_system(cpu: &mut CPU, inst: DecodeInst) {
    match inst.funct3 {
        0b000 => {
            match inst.imm {
                0x000 => ecall(cpu),
                0x302 => mret(cpu),
                _ => panic!("Unkown system instruction")
            }
        },
        0b001 => csrrw(cpu, inst),
        _ => {}
    }
}

fn ecall(cpu: &mut CPU) {
    let syscall_no = cpu.reg[17];

    match syscall_no {
        93 => {
            println!("INFO: Program exited via ECALL (93).");
            std::process::exit(0);
        },
        _ => println!("Unhandled ECALL: syscall number {}", syscall_no)
    }
}

fn mret(cpu: &mut CPU) {
    cpu.pc = cpu.csr[0x341];

    cpu.mode = Mode::User;
    cpu.pc = cpu.pc.wrapping_sub(4);
}

fn csrrw(cpu: &mut CPU, inst: DecodeInst){
    let csr_addr = inst.imm as usize;
    let val = cpu.reg[inst.rs1 as usize];

    let old_val = cpu.csr[csr_addr];
    cpu.csr[csr_addr] = val;

    if inst.rd != 0 {
        cpu.reg[inst.rd as usize] = old_val
    }
}

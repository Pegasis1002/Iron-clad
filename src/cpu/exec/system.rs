use crate::{cpu::{self, decode::DecodeInst}, models::cpu::{CPU, Mode}};

pub(crate) fn exec_system(cpu: &mut CPU, inst: DecodeInst) {
    match inst.funct3 {
        0x0 => {
            match inst.imm {
                0x000 => ecall(cpu),
                0x302 => mret(cpu),
                _ => panic!("Unkown system instruction")
            }
        },
        0x1 => csrrw(cpu, inst),
        0x2 => csrrs(cpu, inst),
        _ => {}
    }
}

fn ecall(cpu: &mut CPU) {
    let syscall_no = cpu.reg[17];

    match syscall_no {
        93 => {
            cpu.csr[0x341] = cpu.pc;
            cpu.csr[0x342] = 8;
            
            cpu.mode = Mode::Machine;
            cpu.pc = cpu.csr[0x305].wrapping_sub(4);
            println!("TRAP: ECALL caught. Jumping to Kernel at {:#X}", cpu.csr[0x305]);
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

// This is the instruction that 'csrr' actually maps to
fn csrrs(cpu: &mut CPU, inst: DecodeInst) {
    let csr_addr = (inst.imm as usize) & 0xFFF;
    let old_val = cpu.csr[csr_addr];

    cpu.csr[csr_addr] = old_val | cpu.reg[inst.rs1 as usize];
    
    // We only care about reading for the benchmark
    if inst.rd != 0 {
        cpu.reg[inst.rd as usize] = old_val;
    }
}

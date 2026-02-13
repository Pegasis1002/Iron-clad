use crate::{cpu::{csr::{MCAUSE, MEPC, MSTATUS, MTVEC}, decode::DecodeInst}, models::cpu::{CPU, Mode}};

use crate::cpu::csr::{read, write};

pub(crate) fn exec_system(cpu: &mut CPU, inst: DecodeInst) {
    match inst.funct3 {
        0x0 => {
            match inst.imm {
                0x000 => ecall(cpu),
                0x302 => mret(cpu),
                _ => panic!("Unkown system instruction")
            }
        },
        0x1 => execute_csr(cpu, &inst, CSROp::Write),
        0x2 => execute_csr(cpu, &inst, CSROp::Set),
        0x3 => execute_csr(cpu, &inst, CSROp::Clear),
        _ => {}
    }
}

pub enum CSROp {
    Write, Set, Clear
}

fn ecall(cpu: &mut CPU) {
    write::csr_write(cpu, MEPC, cpu.pc);

    write::csr_write(cpu, MCAUSE, 8);
    let mstatus = read::csr_read(cpu, MSTATUS);
    let mut new_mstatus = mstatus;

    let mie = (mstatus >> 3) & 1;
    new_mstatus = (new_mstatus & !(1<<7)) | (mie << 7);

    new_mstatus &= !(1<<3);

    let mode = match cpu.mode {
        Mode::User => 0,
        Mode::Machine => 3,
        Mode::Supervisor => 1,
    };

    new_mstatus = (new_mstatus & !(3<<11)) | (mode << 11);

    write::csr_write(cpu, MSTATUS, new_mstatus);

    cpu.mode = Mode::Machine;
    let mtvec = read::csr_read(cpu, MTVEC);
    cpu.pc = mtvec.wrapping_sub(4);

    println!("\nTRAP: ECALL caught. Jumping to Kernel at {:#010X}", mtvec);
}

fn mret(cpu: &mut CPU) {
    let mepc = read::csr_read(cpu, MEPC);
    let mstatus = read::csr_read(cpu, MSTATUS);

    let mpp = (mstatus >> 11) & 3;
    cpu.mode = match mpp {
        0 => Mode::User,
        _ => Mode::Machine,
    };

    let mpie = (mstatus >> 7) & 1;
    let mut new_mstatus = (mstatus & !(1<<3)) | (mpie << 3);

    new_mstatus |= 1 << 7;
    write::csr_write(cpu, MSTATUS, new_mstatus);

    cpu.pc = mepc.wrapping_sub(4);
}

pub fn execute_csr(cpu: &mut CPU, inst: &DecodeInst, op: CSROp) {
    let csr_addr = inst.imm as u16;
    let old_val = read::csr_read(cpu, csr_addr);

    let rs1_val = cpu.reg[inst.rs1 as usize];
    let new_val = match op {
        CSROp::Write => rs1_val,
        CSROp::Set => old_val | rs1_val,
        CSROp::Clear => old_val & !rs1_val,
    };

    write::csr_write(cpu, csr_addr, new_val);

    if inst.rd != 0 {
        cpu.reg[inst.rd as usize] = old_val;
    }
}

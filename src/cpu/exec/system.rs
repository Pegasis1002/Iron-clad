use crate::{cpu::decode::DecodeInst, models::cpu::CPU};

pub(crate) fn exec_system(cpu: &mut CPU, inst: DecodeInst) {
    match inst.imm {
        0 => ecall(cpu),
        _ => panic!("Unknown System immediate: {}", inst.imm)
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

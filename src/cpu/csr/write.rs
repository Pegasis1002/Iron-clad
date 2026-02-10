use crate::{cpu::csr::{MSTATUS, mstatus::mstatus}, models::cpu::CPU};

pub(crate) fn csr_write(cpu: &mut CPU, addr: u16, val: u32){
    match addr {
        MSTATUS => mstatus(cpu, addr, val),
        _ => cpu.csr[addr as usize] = val,
    }
}

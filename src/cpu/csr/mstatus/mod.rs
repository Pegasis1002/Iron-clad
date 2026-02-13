use crate::{cpu::csr::{MSTATUS, MSTATUS_MIE, MSTATUS_MPIE, MSTATUS_MPP}, models::cpu::CPU};

pub(crate) fn mstatus(cpu: &mut CPU, addr: u16, val: u32){
    let mask = MSTATUS_MIE | MSTATUS_MPIE | MSTATUS_MPP;
    cpu.csr[MSTATUS as usize] = val & mask
}

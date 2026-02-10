use crate::{cpu::csr::{MISA, misa::get_misa}, models::cpu::CPU};

pub(crate) fn csr_read(cpu: &mut CPU, addr: u16) -> u32 {
    match addr {
        MISA => get_misa(cpu),
        _ => cpu.csr[addr as usize],
    }
}

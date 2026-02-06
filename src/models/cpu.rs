use crate::models::bus::BUS;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mode {
    User = 0b00,
    Supervisor = 0b01,
    Machine = 0b11,
}

#[allow(dead_code)]
pub(crate) struct CPU {
    pub(crate) pc: u32,
    pub(crate) reg: [u32; 32],
    pub(crate) bus: BUS,
    pub(crate) mode: Mode,
    pub(crate) csr: [u32; 4096],
    pub(crate) cycles: u64,
}

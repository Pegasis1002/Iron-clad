pub(crate) mod write;
pub(crate) mod read;
pub(crate) mod mstatus;
pub(crate) mod misa;

pub const MSTATUS: u16 = 0x300;
pub const MISA: u16 = 0x301;
pub const MTVEC: u16 = 0x305;
pub const MEPC: u16 = 0x341;
pub const MCAUSE: u16 = 0x342;

// Mstatus bit masks
pub const MSTATUS_MIE: u32 = 1 << 3;
pub const MSTATUS_MPIE: u32 = 1 << 7;
pub const MSTATUS_MPP: u32 = 3 << 11;

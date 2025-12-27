use crate::models::bus::BUS;

pub(crate) struct CPU {
    pub(crate) pc: u32,
    pub(crate) reg: [u32; 32],
    pub(crate) bus: BUS,
}


pub struct CPU {
    pc: u32,
    reg: [u32; 32],
    bus: BUS;
}

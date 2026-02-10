use crate::models::cpu::CPU;

pub(crate) fn get_misa(cpu: &mut CPU) -> u32 {
    // RV32IM:
    // bit 31 = 1 (32-bit)
    // bit 8 = 1 (I extension)
    // bit 12 = 1 (M extension)
    (1 << 31) | (1 << 12) | (1 << 8)
}

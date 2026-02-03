use crate::models::cpu::CPU;

#[allow(dead_code)]
#[derive(Debug)]
pub struct DecodeInst {
    pub(crate) op_code: u8,
    pub(crate) rd: u8,
    pub(crate) rs1: u8,
    pub(crate) rs2: u8,
    pub(crate) funct3: u8,
    pub(crate) funct7: u8,
    pub(crate) imm: i32,
}

impl CPU {
    pub(crate) fn decode(&self, inst: u32) -> DecodeInst {
        // Extract op_code
        let op_code = (inst & 0x7F) as u8;

        // match The opcode to a format
        match op_code {
            // I-type
            0x13|0x03|0x67|0x73 => {
                return DecodeInst {
                    op_code: op_code,
                    rd: ((inst >> 7) & 0x1F) as u8,
                    rs1: ((inst >> 15) & 0x1f) as u8, 
                    imm: ((inst as i32)>> 20),
                    funct3: ((inst >> 12) & 0x7) as u8,
                    // parts not used
                    rs2: 0x0,
                    funct7: 0x0,
                }
            },
            // R-type
            0x33 => {
                return DecodeInst {
                    op_code,
                    rd: ((inst >> 7) & 0x1F) as u8,
                    rs1: ((inst >> 15) & 0x1F) as u8,
                    rs2: ((inst >> 20) & 0x1F) as u8,
                    funct3: ((inst >> 12) & 0x7) as u8,
                    funct7: ((inst >> 25) & 0x7F) as u8,
                    imm: 0x0
                }
            },
            // U-type
            0x17|0x37 => {
                return DecodeInst {
                    op_code,
                    rd: ((inst >> 7) & 0x1F) as u8,
                    rs1: 0x0,
                    rs2: 0x0,
                    funct3: 0x0,
                    funct7: 0x0,
                    imm: (inst >> 12) as i32,
                };
            },
            // S-type
            0x23 => {
                let imm_low = (inst >> 7) & 0x1F;
                let imm_high = (inst >> 25) & 0x7F;
                let imm = (imm_high << 5) | imm_low;
                return DecodeInst {
                    op_code,
                    rd: 0x0,
                    rs1: ((inst >> 15) & 0x1F) as u8,
                    rs2: ((inst >> 20) & 0x1F) as u8,
                    funct3: ((inst >> 12) & 0x7) as u8,
                    funct7: 0x0,
                    imm: ((imm as i32) << 20) >> 20,
                };
            },
            // B-type
            0x63 => {
                let sign = (inst >> 31) as u16;
                let hitch_hicker = ((inst >> 7) & 0x1) as u16;
                let imm_high_mid = ((inst >> 25) & 0x3F) as u16;
                let imm_low_mid  = ((inst >> 8) & 0xF) as u16;
                let imm = ((((((sign << 1) | hitch_hicker) << 6) | imm_high_mid) << 4) | imm_low_mid) << 1;

                return DecodeInst {
                    op_code,
                    funct3: ((inst >> 12) & 0x7) as u8,
                    rs1: ((inst >> 15) & 0x1F) as u8,
                    rs2: ((inst >> 20) & 0x1F) as u8,
                    imm: ((imm as i32) << 19) >> 19,
                    rd: 0x0,
                    funct7: 0x0
                }
            },
            // J-Type
            0x6F => {
                let sign = (inst >> 31) as u32;
                let hitch_hicker = ((inst >> 20) & 0x1) as u32;
                let imm_high_mid = ((inst >> 12) & 0xFF) as u32;
                let imm_low_mid = ((inst >> 21) & 0x3FF) as u32;
                let imm = ((((((sign << 8) | imm_high_mid) << 1) | hitch_hicker) << 10) | imm_low_mid) << 1;

                return DecodeInst {
                    op_code,
                    rd: ((inst >> 7) & 0x1F) as u8,
                    imm: ((imm as i32) << 11) >> 11,
                    rs1: 0x0,
                    rs2: 0x0,
                    funct3: 0x0,
                    funct7: 0x0,
                }
            },
            _ => {
                println!("ERROR: Invalid or UnImplemented Instruction.");
                return DecodeInst { op_code: 0x0, rd: 0x0, rs1: 0x0, rs2: 0x0, funct3: 0x0, funct7: 0x0, imm: 0x0 };
            }
        }
    }
}

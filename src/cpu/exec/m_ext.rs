use crate::{cpu::decode::DecodeInst, models::cpu::CPU};

pub(crate) fn exec_m_ext(cpu: &mut CPU, inst: DecodeInst) {
    match inst.funct3 {
        0x0 => mul(cpu, inst),
        0x1 => mulh(cpu, inst),
        0x2 => mulhsu(cpu, inst),
        0x3 => mulhu(cpu, inst),
        0x4 => div(cpu, inst),
        0x5 => divu(cpu, inst),
        0x6 => rem(cpu, inst),
        0x7 => remu(cpu, inst),
        _ => panic!("Invalid M-extension funct3: {}", inst.funct3),
    }
}

fn mul(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd == 0 {
        return;
    }

    let rs1 = inst.rs1 as usize;
    let rs2 = inst.rs2 as usize;

    cpu.reg[rd] = cpu.reg[rs1].wrapping_mul(cpu.reg[rs2]);
}

fn mulh(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd == 0 {
        return;
    }

    let rs1 = cpu.reg[inst.rs1 as usize];
    let rs2 = cpu.reg[inst.rs2 as usize];

    let res = (rs1 as i32 as i64).wrapping_mul(rs2 as i32 as i64);
    cpu.reg[rd] = (res >> 32) as u32;
}

fn mulhsu(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd == 0 {
        return;
    }

    let rs1 = cpu.reg[inst.rs1 as usize];
    let rs2 = cpu.reg[inst.rs2 as usize];

    let res = (rs1 as i32 as i64).wrapping_mul(rs2 as u64 as i64);
    cpu.reg[rd] = (res >> 32) as u32;
}

fn mulhu(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd == 0 {
        return;
    }

    let rs1 = cpu.reg[inst.rs1 as usize];
    let rs2 = cpu.reg[inst.rs2 as usize];

    let res = (rs1 as u64).wrapping_mul(rs2 as u64);
    cpu.reg[rd] = (res >> 32) as u32;
}

fn div(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd == 0 {
        return;
    }

    let rs1 = cpu.reg[inst.rs1 as usize];
    let rs2 = cpu.reg[inst.rs2 as usize];

    let s1 = rs1 as i32;
    let s2 = rs2 as i32;

    if s2 == 0 {
        cpu.reg[rd] = 0xFFFFFFFF;
    } else if s1 == i32::MIN && s2 == -1 {
        cpu.reg[rd] = s1 as u32;
    } else {
        cpu.reg[rd] = (s1 /s2) as u32;
    }
}

fn divu(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd == 0 {
        return;
    }

    let rs1 = cpu.reg[inst.rs1 as usize];
    let rs2 = cpu.reg[inst.rs2 as usize];
    if rs2 == 0 {
        cpu.reg[rd] = 0xFFFFFFFF;
    }else {
        cpu.reg[rd] = rs1 / rs2;
    }
}

fn rem(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd == 0 {
        return;
    }

    let rs1 = cpu.reg[inst.rs1 as usize];
    let rs2 = cpu.reg[inst.rs2 as usize];

    let s1 = rs1 as i32;
    let s2 = rs2 as i32;

    if s2 == 0 {
        cpu.reg[rd] = rs1;
    } else if s1 == i32::MIN && s2 == -1 {
        cpu.reg[rd] = 0;
    } else {
        cpu.reg[rd] = (s1 % s2) as u32;
    }
}

fn remu(cpu: &mut CPU, inst: DecodeInst) {
    let rd = inst.rd as usize;
    if rd == 0 {
        return;
    }

    let rs1 = cpu.reg[inst.rs1 as usize];
    let rs2 = cpu.reg[inst.rs2 as usize];

    if rs2 == 0 {
        cpu.reg[rd] = rs1;
    } else {
        cpu.reg[rd] = rs1 % rs2;
    }
}

use std::ops::{BitAnd, BitOr, BitXor};

use crate::{
    bus::Bus,
    utils::{bitflags::Bitflag, ops::ExtendedOps},
};

use super::{
    Arm7tdmi,
    common::{DataType, Operand, ToOperand},
    psr::Psr,
};

impl<B: Bus> Arm7tdmi<B> {
    pub fn lsl(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        self.shift_op(u32::wrapping_shl, lhs, rhs, dst);
    }

    pub fn lsr(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        self.shift_op(u32::wrapping_shr, lhs, rhs, dst);
    }

    pub fn asr(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        self.shift_op(u32::wrapping_asr, lhs, rhs, dst);
    }

    pub fn ror(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        self.shift_op(u32::rotate_right, lhs, rhs, dst);
    }

    pub fn add(&mut self, lhs: u8, rhs: Operand, dst: u8, update: bool) {
        self.add_sub_op(lhs.register(), rhs, dst.into(), 0, update);
    }

    pub fn adc(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        let carry = self.cpsr.get(Psr::C);
        self.add_sub_op(lhs.register(), rhs, dst.into(), carry, true);
    }

    pub fn sub(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        self.add_sub_op(lhs.register(), rhs.not(), dst.into(), 1, true);
    }

    pub fn sbc(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        let carry = self.cpsr.get(Psr::C);
        self.add_sub_op(lhs.register(), rhs.not(), dst.into(), carry, true);
    }

    pub fn cmp(&mut self, lhs: u8, rhs: Operand) {
        self.add_sub_op(lhs.register(), rhs.not(), None, 1, true);
    }

    pub fn cmn(&mut self, lhs: u8, rhs: Operand) {
        self.add_sub_op(lhs.register(), rhs, None, 0, true);
    }

    pub fn neg(&mut self, rd: u8, rs: u8) {
        let lhs = 0_u32.immediate();
        self.add_sub_op(lhs, rs.register().not(), rd.into(), 1, true);
    }

    pub fn and(&mut self, rd: u8, rs: u8) {
        self.logical_op(u32::bitand, rs, rd.register(), rd.into());
    }

    pub fn eor(&mut self, rd: u8, rs: u8) {
        self.logical_op(u32::bitxor, rs, rd.register(), rd.into());
    }

    pub fn orr(&mut self, rd: u8, rs: u8) {
        self.logical_op(u32::bitor, rs, rd.register(), rd.into());
    }

    pub fn tst(&mut self, rd: u8, rs: u8) {
        self.logical_op(u32::bitand, rd, rs.register(), None);
    }

    pub fn bic(&mut self, rd: u8, rs: u8) {
        self.logical_op(u32::bitand, rd, rs.register().not(), rd.into());
    }

    pub fn ldrb(&mut self, rd: u8, addr: u32) {
        self.ldr_op(rd, addr, DataType::Byte, false);
    }

    pub fn ldrh(&mut self, rd: u8, addr: u32) {
        self.ldr_op(rd, addr, DataType::HalfWord, false);
    }

    pub fn ldr(&mut self, rd: u8, addr: u32) {
        self.ldr_op(rd, addr, DataType::Word, false);
    }

    pub fn ldsb(&mut self, rd: u8, addr: u32) {
        self.ldr_op(rd, addr, DataType::Byte, true);
    }

    pub fn ldsh(&mut self, rd: u8, addr: u32) {
        self.ldr_op(rd, addr, DataType::HalfWord, true);
    }

    pub fn strb(&mut self, rd: u8, addr: u32) {
        self.str_op(rd, addr, DataType::Byte);
    }

    pub fn strh(&mut self, rd: u8, addr: u32) {
        self.str_op(rd, addr, DataType::HalfWord);
    }

    pub fn str(&mut self, rd: u8, addr: u32) {
        self.str_op(rd, addr, DataType::Word);
    }

    pub fn mvn(&mut self, rd: u8, rs: u8) {
        self.mov(rd, rs.register().not(), true);
    }

    pub fn mov(&mut self, rd: u8, operand: Operand, update: bool) {
        let value = self.get_operand(operand);

        if update {
            self.cpsr.update_zn(value);
        }

        self.set_reg(rd, value);
    }

    pub fn mul(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        let lhs = self.get_reg(lhs);
        let rhs = self.get_operand(rhs);
        let result = lhs.wrapping_mul(rhs);

        self.cpsr.update_zn(result);
        self.cpsr.update(Psr::C, false);

        self.set_reg(dst, result);
    }

    pub fn bx(&mut self, rs: u8) {
        let value = self.get_reg(rs);

        if value.get(0) == 0 {
            self.cpsr.set_arm_mode();
        }

        self.set_pc(value);
    }
}

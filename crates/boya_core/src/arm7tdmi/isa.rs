use std::ops::{BitAnd, BitOr, BitXor};

use crate::{bus::Bus, utils::ops::ExtendedOps};

use super::{
    common::{Carry, DataType, Operand, ToOperand},
    Arm7tdmi,
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
        self.add_sub_op(lhs.reg(), rhs, dst.into(), Carry::None, update);
    }

    pub fn adc(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        self.add_sub_op(lhs.reg(), rhs, dst.into(), Carry::Flag, true);
    }

    pub fn sub(&mut self, lhs: u8, rhs: Operand, dst: u8, update: bool) {
        self.add_sub_op(lhs.reg(), rhs.not(), dst.into(), Carry::One, update);
    }

    pub fn sbc(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        self.add_sub_op(lhs.reg(), rhs.not(), dst.into(), Carry::Flag, true);
    }

    pub fn cmp(&mut self, lhs: u8, rhs: Operand) {
        self.add_sub_op(lhs.reg(), rhs.not(), None, Carry::One, true);
    }

    pub fn cmn(&mut self, lhs: u8, rhs: Operand) {
        self.add_sub_op(lhs.reg(), rhs, None, Carry::None, true);
    }

    pub fn neg(&mut self, rd: u8, rs: u8) {
        self.add_sub_op(0_u32.imm(), rs.reg().not(), rd.into(), Carry::One, true);
    }

    pub fn and(&mut self, rd: u8, rs: u8) {
        self.logical_op(u32::bitand, rs, rd.reg(), rd.into());
    }

    pub fn eor(&mut self, rd: u8, rs: u8) {
        self.logical_op(u32::bitxor, rs, rd.reg(), rd.into());
    }

    pub fn orr(&mut self, rd: u8, rs: u8) {
        self.logical_op(u32::bitor, rs, rd.reg(), rd.into());
    }

    pub fn tst(&mut self, rd: u8, rs: u8) {
        self.logical_op(u32::bitand, rd, rs.reg(), None);
    }

    pub fn bic(&mut self, rd: u8, rs: u8) {
        self.logical_op(u32::bitand, rd, rs.reg().not(), rd.into());
    }

    pub fn ldrb(&mut self, rd: u8, addr: u32) {
        self.ldr_op(rd, addr, DataType::Byte, false);
    }

    pub fn ldrh(&mut self, rd: u8, addr: u32) {
        self.ldr_op(rd, addr, DataType::HWord, false);
    }

    pub fn ldr(&mut self, rd: u8, addr: u32) {
        self.ldr_op(rd, addr, DataType::Word, false);
    }

    pub fn ldsb(&mut self, rd: u8, addr: u32) {
        self.ldr_op(rd, addr, DataType::Byte, true);
    }

    pub fn ldsh(&mut self, rd: u8, addr: u32) {
        self.ldr_op(rd, addr, DataType::HWord, true);
    }

    pub fn strb(&mut self, rd: u8, addr: u32) {
        self.str_op(rd, addr, DataType::Byte);
    }

    pub fn strh(&mut self, rd: u8, addr: u32) {
        self.str_op(rd, addr, DataType::HWord);
    }

    pub fn str(&mut self, rd: u8, addr: u32) {
        self.str_op(rd, addr, DataType::Word);
    }

    pub fn mvn(&mut self, rd: u8, rs: u8) {
        self.mov_op(rd, rs.reg().not(), true);
    }

    pub fn mov(&mut self, rd: u8, operand: Operand, update: bool) {
        self.mov_op(rd, operand, update);
    }

    pub fn mul(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        self.mul_op(lhs, rhs, dst);
    }

    pub fn bx(&mut self, rs: u8) {
        self.bx_op(rs);
    }

    pub fn push(&mut self, rlist: u8, lr: bool) {
        self.push_op(rlist, lr);
    }

    pub fn pop(&mut self, rlist: u8, pc: bool) {
        self.pop_op(rlist, pc);
    }
}

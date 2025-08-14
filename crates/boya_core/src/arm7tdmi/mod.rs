mod common;
mod psr;
mod thumb;

use std::ops::{BitAnd, BitOr, BitXor};

use common::{AddSubOp, Operand, OperandKind, ToOperand};
use psr::{OperatingMode, Psr};

use crate::{
    bus::Bus,
    utils::{ops, ringbuffer::RingBuffer},
};

#[derive(Debug)]
pub struct Arm7tdmi<B: Bus> {
    reg: [u32; 16],    //  R0-R15
    reg_fiq: [u32; 7], //  R8-R14
    reg_svc: [u32; 2], // R13-R14
    reg_abt: [u32; 2], // R13-R14
    reg_irq: [u32; 2], // R13-R14
    reg_und: [u32; 2], // R13-R14

    cpsr: Psr,
    spsr: [Psr; 5],

    cycles: u64,
    bus: B,
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn new(bus: B) -> Self {
        Self {
            reg: [0; 16],
            reg_fiq: [0; 7],
            reg_svc: [0; 2],
            reg_abt: [0; 2],
            reg_irq: [0; 2],
            reg_und: [0; 2],
            cpsr: Psr::new(),
            spsr: [Psr::default(); 5],
            cycles: 0,
            bus,
        }
    }

    #[inline(always)]
    fn pc(&self) -> u32 {
        self.reg[15]
    }

    #[inline(always)]
    fn increment_pc(&mut self, value: u32) {
        self.reg[15] = self.pc().wrapping_add(value);
    }

    fn get_reg<I: Into<usize>>(&self, index: I) -> u32 {
        let mode = self.cpsr.operating_mode();
        let index = index.into();

        match (mode, index) {
            (OperatingMode::Fiq, 8..14) => self.reg_fiq[index - 8],
            (OperatingMode::Svc, 13..14) => self.reg_svc[index - 13],
            (OperatingMode::Abt, 13..14) => self.reg_abt[index - 13],
            (OperatingMode::Irq, 13..14) => self.reg_irq[index - 13],
            (OperatingMode::Und, 13..14) => self.reg_und[index - 13],
            (_, _) => self.reg[index],
        }
    }

    fn get_reg_mut<I: Into<usize>>(&mut self, index: I) -> &mut u32 {
        let mode = self.cpsr.operating_mode();
        let index = index.into();

        match (mode, index) {
            (OperatingMode::Fiq, 8..14) => &mut self.reg_fiq[index - 8],
            (OperatingMode::Svc, 13..14) => &mut self.reg_svc[index - 13],
            (OperatingMode::Abt, 13..14) => &mut self.reg_abt[index - 13],
            (OperatingMode::Irq, 13..14) => &mut self.reg_irq[index - 13],
            (OperatingMode::Und, 13..14) => &mut self.reg_und[index - 13],
            (_, _) => &mut self.reg[index],
        }
    }

    #[inline(always)]
    fn set_reg<I>(&mut self, index: I, value: u32)
    where
        I: Into<usize> + Copy,
    {
        *self.get_reg_mut(index) = value;
    }

    fn get_operand(&self, operand: Operand) -> u32 {
        let value = match operand.kind {
            OperandKind::Immediate => operand.value,
            OperandKind::Register => self.get_reg(operand.value as usize),
        };

        if operand.negate { !value } else { value }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn lsl(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        self.shift_op(u32::wrapping_shl, lhs, rhs, dst);
    }

    pub fn lsr(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        self.shift_op(u32::wrapping_shr, lhs, rhs, dst);
    }

    pub fn asr(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        self.shift_op(ops::asr_u32, lhs, rhs, dst);
    }

    pub fn ror(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        self.shift_op(u32::rotate_right, lhs, rhs, dst);
    }

    pub fn add(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        self.add_sub_op(AddSubOp::Add, lhs.register(), rhs, dst.into(), false);
    }

    pub fn adc(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        self.add_sub_op(AddSubOp::Add, lhs.register(), rhs, dst.into(), true);
    }

    pub fn sub(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        self.add_sub_op(AddSubOp::Sub, lhs.register(), rhs, dst.into(), false);
    }

    pub fn sbc(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        self.add_sub_op(AddSubOp::Sub, lhs.register(), rhs, dst.into(), true);
    }

    pub fn cmp(&mut self, lhs: u8, rhs: Operand) {
        self.add_sub_op(AddSubOp::Sub, lhs.register(), rhs, None, false);
    }

    pub fn cmn(&mut self, lhs: u8, rhs: Operand) {
        self.add_sub_op(AddSubOp::Add, lhs.register(), rhs, None, false);
    }

    pub fn neg(&mut self, rd: u8, rs: u8) {
        let lhs = 0_u32.immediate();
        self.add_sub_op(AddSubOp::Sub, lhs, rs.register(), rd.into(), false);
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
        self.logical_op(u32::bitand, rd, rs.register().not(), None);
    }

    pub fn mov(&mut self, rd: u8, operand: Operand) {
        let value = self.get_operand(operand);

        self.cpsr.update_zero(value);
        self.cpsr.update_sign(value);

        self.set_reg(rd, value);
    }

    pub fn mvn(&mut self, rd: u8, rs: u8) {
        self.mov(rd, rs.register().not());
    }

    pub fn mul(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        let lhs = self.get_reg(lhs);
        let rhs = self.get_operand(rhs);
        let result = lhs.wrapping_mul(rhs);

        self.cpsr.update_zero(result);
        self.cpsr.update_sign(result);
        self.cpsr.update_carry(false);

        self.set_reg(dst, result);
    }

    #[inline(always)]
    pub fn add_sub_op(
        &mut self,
        op: AddSubOp,
        lhs: Operand,
        rhs: Operand,
        dst: Option<u8>,
        carry: bool,
    ) {
        let lhs = self.get_operand(lhs);
        let rhs = self.get_operand(rhs);

        let (result, overflow, carry) = if carry {
            let carry_bit = self.cpsr.carry_bit();
            let (res1, of1, cr1) = self.add_sub_op_inner(op, lhs, rhs);
            let (res2, of2, cr2) = self.add_sub_op_inner(op, res1, !carry_bit);
            (res2, of1 || of2, cr1 || cr2)
        } else {
            self.add_sub_op_inner(op, lhs, rhs)
        };

        self.cpsr.update_zero(result);
        self.cpsr.update_sign(result);
        self.cpsr.update_carry(carry);
        self.cpsr.update_overflow(overflow);

        if let Some(rd) = dst {
            self.set_reg(rd, result);
        }
    }

    pub fn add_sub_op_inner(&self, op: AddSubOp, lhs: u32, rhs: u32) -> (u32, bool, bool) {
        let (result, overflow) = match op {
            AddSubOp::Add => lhs.overflowing_add(rhs),
            AddSubOp::Sub => lhs.overflowing_sub(rhs),
        };

        let carry = match op {
            AddSubOp::Add => result < lhs, // C=1 if unsigned overflow
            AddSubOp::Sub => lhs >= rhs,   // C=1 if no borrow
        };

        (result, overflow, carry)
    }

    #[inline(always)]
    pub fn shift_op<F>(&mut self, func: F, lhs: u8, rhs: Operand, dst: u8)
    where
        F: Fn(u32, u32) -> u32,
    {
        let lhs = self.get_reg(lhs);
        let rhs = self.get_operand(rhs) & 0xFF;
        let result = func(lhs, rhs);

        self.cpsr.update_zero(result);
        self.cpsr.update_sign(result);
        self.cpsr.update_carry(rhs > 0);

        self.set_reg(dst, result);
    }

    #[inline(always)]
    pub fn logical_op<F>(&mut self, func: F, lhs: u8, rhs: Operand, dst: Option<u8>)
    where
        F: Fn(u32, u32) -> u32,
    {
        let lhs = self.get_reg(lhs);
        let rhs = self.get_operand(rhs);
        let result = func(lhs, rhs);

        self.cpsr.update_zero(result);
        self.cpsr.update_sign(result);

        if let Some(rd) = dst {
            self.set_reg(rd, result);
        }
    }
}

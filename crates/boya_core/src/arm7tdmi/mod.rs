mod bank;
mod common;
mod pipeline;
mod psr;
mod thumb;

use std::ops::{BitAnd, BitOr, BitXor};

use bank::Bank;
use common::{Operand, OperandKind, ToOperand};
use pipeline::Pipeline;
use psr::{Exception, OperatingMode, Psr};

use crate::{
    bus::Bus,
    utils::{bitflags::Bitflag, ops::ExtendedOps, ringbuffer::RingBuffer},
};

#[derive(Debug)]
pub struct Arm7tdmi<B: Bus> {
    reg: [u32; 16], // R0-R15
    bank: Bank,
    cpsr: Psr,
    pipeline: Pipeline,
    cycles: u64,
    bus: B,
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn new(bus: B) -> Self {
        let mut cpu = Self {
            reg: [0; 16],
            bank: Bank::default(),
            cpsr: Psr::new(),
            pipeline: Pipeline::default(),
            cycles: 0,
            bus,
        };

        cpu.handle_exception(Exception::Reset);
        cpu
    }

    pub fn step(&mut self) {
        let instruction = self.fetch_pipeline();

        if self.cpsr.has(Psr::T) {
            let decoded = self.decode_thumb(instruction);
            self.pre_fetch();
            self.exec_thumb(decoded); // PC is PC + 4
        } else {
            todo!()
        }
    }

    #[inline(always)]
    pub fn fetch(&mut self) -> u32 {
        self.bus.read_u32(self.pc())
    }

    #[inline(always)]
    fn pc(&self) -> u32 {
        self.reg[15]
    }

    #[inline(always)]
    fn set_pc(&mut self, value: u32) {
        self.reg[15] = value;
    }

    #[inline(always)]
    fn increment_pc(&mut self) {
        println!("thumb: {}", self.cpsr.has(Psr::T));
        self.reg[15] += if self.cpsr.has(Psr::T) { 2 } else { 4 };
    }

    fn get_reg<I>(&self, index: I) -> u32
    where
        I: Into<usize> + Copy,
    {
        let mode = self.cpsr.operating_mode();

        self.bank
            .get_reg(mode, index.into())
            .unwrap_or_else(|| self.reg[index.into()])
    }

    fn get_reg_mut<I>(&mut self, index: I) -> &mut u32
    where
        I: Into<usize> + Copy,
    {
        let mode = self.cpsr.operating_mode();

        self.bank
            .get_reg_mut(mode, index.into())
            .unwrap_or_else(|| &mut self.reg[index.into()])
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

    pub fn mov(&mut self, rd: u8, operand: Operand, update: bool) {
        let value = self.get_operand(operand);

        if update {
            self.cpsr.update_zn(value);
        }

        self.set_reg(rd, value);
    }

    pub fn mvn(&mut self, rd: u8, rs: u8) {
        self.mov(rd, rs.register().not(), true);
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
        let mut value = self.get_reg(rs);

        if value.get(0) == 0 {
            self.cpsr.set_arm_mode();
            value.clear(1); // 32 word alignement
        }

        self.set_pc(value);
    }

    #[inline(always)]
    pub fn add_sub_op(
        &mut self,
        lhs: Operand,
        rhs: Operand,
        dst: Option<u8>,
        carry: u32,
        update: bool,
    ) {
        let lhs = self.get_operand(lhs);
        let rhs = self.get_operand(rhs);
        let (res1, ov1) = lhs.overflowing_add(rhs);
        let (res2, ov2) = res1.overflowing_add(carry);
        let overflow = ((res2 ^ lhs) & (res2 ^ rhs)).has(31);

        if update {
            self.cpsr.update_zn(res2);
            self.cpsr.update(Psr::C, ov1 || ov2);
            self.cpsr.update(Psr::V, overflow);
        }

        if let Some(rd) = dst {
            self.set_reg(rd, res2);
        }
    }

    #[inline(always)]
    pub fn shift_op<F>(&mut self, func: F, lhs: u8, rhs: Operand, dst: u8)
    where
        F: Fn(u32, u32) -> u32,
    {
        let lhs = self.get_reg(lhs);
        let rhs = self.get_operand(rhs) & 0xFF;
        let result = func(lhs, rhs);

        self.cpsr.update_zn(result);
        self.cpsr.update(Psr::C, rhs > 0);

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

        self.cpsr.update_zn(result);

        if let Some(rd) = dst {
            self.set_reg(rd, result);
        }
    }
}

#[cfg(test)]
impl<B: Bus> Arm7tdmi<B> {
    pub fn new_thumb(bus: B) -> Self {
        let mut cpu = Self::new(bus);

        cpu.cpsr.set_thumb_mode();
        cpu.set_pc(0x00);
        cpu.reload_pipeline();

        cpu
    }

    pub fn assert_mem(&self, assertions: Vec<(u32, u32)>) {
        for (address, expected) in assertions {
            let value = self.bus.read_u32(address);
            assert_eq!(
                value, expected,
                "expected 0x{expected:x} at @{address:x}, got 0x{value:x}"
            )
        }
    }

    pub fn assert_reg(&self, assertions: Vec<(usize, u32)>) {
        for (index, expected) in assertions {
            let value = self.get_reg(index);
            assert_eq!(
                value, expected,
                "expected 0x{expected:x} at R{index}, got 0x{value:x}"
            )
        }
    }

    pub fn assert_flag(&self, assertions: Vec<(u32, bool)>) {
        for (flag, expected) in assertions {
            let value = self.cpsr.has(flag);
            let name = Psr::format_flag(flag);
            let status = if expected { "set" } else { "cleared" };
            assert_eq!(
                value, expected,
                "expected flag {name} to be {status}, flags: {:?}",
                self.cpsr
            )
        }
    }
}

#[cfg(test)]
pub mod utils {
    pub use super::psr::Psr;
}

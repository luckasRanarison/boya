use crate::{
    bus::Bus,
    utils::bitflags::{BitIter, Bitflag},
};

use super::{
    common::{Carry, DataType, Operand},
    psr::Psr,
    Arm7tdmi,
};

impl<B: Bus> Arm7tdmi<B> {
    #[inline(always)]
    pub fn add_sub_op(
        &mut self,
        lhs: Operand,
        rhs: Operand,
        dst: Option<u8>,
        carry: Carry,
        update: bool,
    ) {
        let lhs = self.get_operand(lhs);
        let rhs = self.get_operand(rhs);

        let carry = match carry {
            Carry::One => 1,
            Carry::None => 0,
            Carry::Flag => self.cpsr.get(Psr::C),
        };

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

    #[inline(always)]
    pub fn mov_op(&mut self, rd: u8, operand: Operand, update: bool) {
        let value = self.get_operand(operand);

        if update {
            self.cpsr.update_zn(value);
        }

        self.set_reg(rd, value);
    }

    #[inline(always)]
    pub fn bx_op(&mut self, rs: u8) {
        let value = self.get_reg(rs);

        if value.get(0) == 0 {
            self.cpsr.set_arm_mode();
        }

        self.set_pc(value);
    }

    #[inline(always)]
    pub fn mul_op(&mut self, lhs: u8, rhs: Operand, dst: u8) {
        let lhs = self.get_reg(lhs);
        let rhs = self.get_operand(rhs);
        let result = lhs.wrapping_mul(rhs);

        self.cpsr.update_zn(result);
        self.cpsr.update(Psr::C, false);

        self.set_reg(dst, result);
    }

    #[inline(always)]
    pub fn ldr_op(&mut self, rd: u8, addr: u32, kind: DataType, signed: bool) {
        let value = match kind {
            DataType::Byte if signed => self.bus.read_byte(addr) as i8 as i32 as u32,
            DataType::HWord if signed => self.bus.read_hword(addr) as i8 as i32 as u32,
            DataType::Byte => self.bus.read_byte(addr).into(),
            DataType::HWord => self.bus.read_hword(addr).into(),
            DataType::Word => self.bus.read_word(addr),
        };

        self.set_reg(rd, value)
    }

    #[inline(always)]
    pub fn str_op(&mut self, rs: u8, addr: u32, kind: DataType) {
        let value = self.get_reg(rs);

        match kind {
            DataType::Byte => self.bus.write_byte(addr, (value & 0xFF) as u8),
            DataType::HWord => self.bus.write_hword(addr, (value & 0xFFFF) as u16),
            DataType::Word => self.bus.write_word(addr, value),
        }
    }

    #[inline(always)]
    pub fn push_op<I: BitIter>(&mut self, rlist: I, lr: bool) {
        for (idx, bit) in rlist.iter_lsb() {
            if bit == 1 {
                self.push_sp(idx);
            }
        }

        if lr {
            self.push_sp(Self::LR);
        }
    }

    #[inline(always)]
    pub fn pop_op<I: BitIter>(&mut self, rlist: I, pc: bool) {
        for (idx, bit) in rlist.iter_msb() {
            if bit == 1 {
                self.pop_sp(idx);
            }
        }

        if pc {
            self.pop_sp(Self::PC);
        }
    }
}

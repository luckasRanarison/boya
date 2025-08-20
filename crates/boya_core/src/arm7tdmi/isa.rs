use std::ops::{BitAnd, BitOr, BitXor};

use crate::{
    arm7tdmi::common::{Cycle, Exception},
    bus::Bus,
    utils::ops::ExtendedOps,
};

use super::{
    Arm7tdmi,
    common::{Carry, DataType, Operand, RegisterFx, ToOperand},
};

pub trait Executable<B: Bus> {
    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle;
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn lsl(&mut self, lhs: u8, rhs: Operand, dst: u8) -> Cycle {
        self.shift_op(u32::wrapping_shl, lhs, rhs, dst)
    }

    pub fn lsr(&mut self, lhs: u8, rhs: Operand, dst: u8) -> Cycle {
        self.shift_op(u32::wrapping_shr, lhs, rhs, dst)
    }

    pub fn asr(&mut self, lhs: u8, rhs: Operand, dst: u8) -> Cycle {
        self.shift_op(u32::wrapping_asr, lhs, rhs, dst)
    }

    pub fn ror(&mut self, lhs: u8, rhs: Operand, dst: u8) -> Cycle {
        self.shift_op(u32::rotate_right, lhs, rhs, dst)
    }

    pub fn add(&mut self, lhs: u8, rhs: Operand, dst: u8, update: bool) -> Cycle {
        self.add_sub_op(lhs.reg(), rhs, dst.into(), Carry::None, update)
    }

    pub fn adc(&mut self, lhs: u8, rhs: Operand, dst: u8) -> Cycle {
        self.add_sub_op(lhs.reg(), rhs, dst.into(), Carry::Flag, true)
    }

    pub fn sub(&mut self, lhs: u8, rhs: Operand, dst: u8, update: bool) -> Cycle {
        self.add_sub_op(lhs.reg(), rhs.not(), dst.into(), Carry::One, update)
    }

    pub fn sbc(&mut self, lhs: u8, rhs: Operand, dst: u8) -> Cycle {
        self.add_sub_op(lhs.reg(), rhs.not(), dst.into(), Carry::Flag, true)
    }

    pub fn cmp(&mut self, lhs: u8, rhs: Operand) -> Cycle {
        self.add_sub_op(lhs.reg(), rhs.not(), None, Carry::One, true)
    }

    pub fn cmn(&mut self, lhs: u8, rhs: Operand) -> Cycle {
        self.add_sub_op(lhs.reg(), rhs, None, Carry::None, true)
    }

    pub fn neg(&mut self, rd: u8, rs: u8) -> Cycle {
        self.add_sub_op(0_u32.imm(), rs.reg().not(), rd.into(), Carry::One, true)
    }

    pub fn and(&mut self, rd: u8, rs: u8) -> Cycle {
        self.logical_op(u32::bitand, rs, rd.reg(), rd.into())
    }

    pub fn eor(&mut self, rd: u8, rs: u8) -> Cycle {
        self.logical_op(u32::bitxor, rs, rd.reg(), rd.into())
    }

    pub fn orr(&mut self, rd: u8, rs: u8) -> Cycle {
        self.logical_op(u32::bitor, rs, rd.reg(), rd.into())
    }

    pub fn tst(&mut self, rd: u8, rs: u8) -> Cycle {
        self.logical_op(u32::bitand, rd, rs.reg(), None)
    }

    pub fn bic(&mut self, rd: u8, rs: u8) -> Cycle {
        self.logical_op(u32::bitand, rd, rs.reg().not(), rd.into())
    }

    pub fn ldrb(&mut self, rd: u8, addr: u32) -> Cycle {
        self.ldr_op(rd, addr, DataType::Byte, false)
    }

    pub fn ldrh(&mut self, rd: u8, addr: u32) -> Cycle {
        self.ldr_op(rd, addr, DataType::HWord, false)
    }

    pub fn ldr(&mut self, rd: u8, addr: u32) -> Cycle {
        self.ldr_op(rd, addr, DataType::Word, false)
    }

    pub fn ldsb(&mut self, rd: u8, addr: u32) -> Cycle {
        self.ldr_op(rd, addr, DataType::Byte, true)
    }

    pub fn ldsh(&mut self, rd: u8, addr: u32) -> Cycle {
        self.ldr_op(rd, addr, DataType::HWord, true)
    }

    pub fn strb(&mut self, rd: u8, addr: u32) -> Cycle {
        self.str_op(rd, addr, DataType::Byte)
    }

    pub fn strh(&mut self, rd: u8, addr: u32) -> Cycle {
        self.str_op(rd, addr, DataType::HWord)
    }

    pub fn str(&mut self, rd: u8, addr: u32) -> Cycle {
        self.str_op(rd, addr, DataType::Word)
    }

    pub fn mvn(&mut self, rd: u8, rs: u8) -> Cycle {
        self.mov_op(rd, rs.reg().not(), true)
    }

    pub fn mov(&mut self, rd: u8, operand: Operand, update: bool) -> Cycle {
        self.mov_op(rd, operand, update)
    }

    pub fn mul(&mut self, lhs: u8, rhs: Operand, dst: u8) -> Cycle {
        self.mul_op(lhs, rhs, dst)
    }

    pub fn bx(&mut self, rs: u8) -> Cycle {
        self.bx_op(rs)
    }

    pub fn push(&mut self, rlist: u8, lr: bool) -> Cycle {
        self.stm_op(Self::SP, rlist, lr.then_some(Self::LR), RegisterFx::DecB)
    }

    pub fn pop(&mut self, rlist: u8, pc: bool) -> Cycle {
        self.ldm_op(Self::SP, rlist, pc.then_some(Self::PC), RegisterFx::IncA)
    }

    pub fn stmia(&mut self, rlist: u8, rb: u8) -> Cycle {
        self.stm_op(rb.into(), rlist, None, RegisterFx::IncA)
    }

    pub fn ldmia(&mut self, rlist: u8, rb: u8) -> Cycle {
        self.ldm_op(rb.into(), rlist, None, RegisterFx::IncA)
    }

    pub fn beq(&mut self, offset: i16) -> Cycle {
        self.branch_op(self.cpsr.z(), offset)
    }

    pub fn bne(&mut self, offset: i16) -> Cycle {
        self.branch_op(!self.cpsr.z(), offset)
    }

    pub fn bcs(&mut self, offset: i16) -> Cycle {
        self.branch_op(self.cpsr.c(), offset)
    }

    pub fn bcc(&mut self, offset: i16) -> Cycle {
        self.branch_op(!self.cpsr.c(), offset)
    }

    pub fn bmi(&mut self, offset: i16) -> Cycle {
        self.branch_op(self.cpsr.s(), offset)
    }

    pub fn bpl(&mut self, offset: i16) -> Cycle {
        self.branch_op(!self.cpsr.s(), offset)
    }

    pub fn bvs(&mut self, offset: i16) -> Cycle {
        self.branch_op(self.cpsr.v(), offset)
    }

    pub fn bvc(&mut self, offset: i16) -> Cycle {
        self.branch_op(!self.cpsr.v(), offset)
    }

    pub fn bhi(&mut self, offset: i16) -> Cycle {
        self.branch_op(self.cpsr.c() && !self.cpsr.z(), offset)
    }

    pub fn bls(&mut self, offset: i16) -> Cycle {
        self.branch_op(!self.cpsr.c() && self.cpsr.z(), offset)
    }

    pub fn bge(&mut self, offset: i16) -> Cycle {
        self.branch_op(self.cpsr.s() == self.cpsr.v(), offset)
    }

    pub fn blt(&mut self, offset: i16) -> Cycle {
        self.branch_op(self.cpsr.s() != self.cpsr.v(), offset)
    }

    pub fn bgt(&mut self, offset: i16) -> Cycle {
        self.branch_op(!self.cpsr.z() && self.cpsr.s() == self.cpsr.v(), offset)
    }

    pub fn ble(&mut self, offset: i16) -> Cycle {
        self.branch_op(self.cpsr.z() && self.cpsr.s() != self.cpsr.v(), offset)
    }

    pub fn swi(&mut self) -> Cycle {
        self.handle_exception(Exception::SoftwareInterrupt)
    }

    pub fn b(&mut self, offset: i16) -> Cycle {
        self.branch_op(true, offset)
    }
}

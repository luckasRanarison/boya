pub mod prelude {
    pub use std::fmt::Debug;

    pub use crate::arm7tdmi::Arm7tdmi;
    pub use crate::arm7tdmi::common::*;
    pub use crate::arm7tdmi::isa::Executable;
    pub use crate::bus::Bus;
    pub use crate::utils::bitflags::Bitflag;

    #[cfg(test)]
    pub use crate::{arm7tdmi::test::Psr, test::AsmTestBuilder};
}

use std::ops::{BitAnd, BitOr, BitXor};

use prelude::*;

use crate::utils::ops::ExtendedOps;

pub trait Executable<B: Bus>: Sized {
    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle;

    fn condition(&self) -> Condition {
        Condition::AL
    }

    fn dispatch_checked(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        if cpu.cpsr.matches(self.condition()) {
            self.dispatch(cpu)
        } else {
            Cycle { i: 0, s: 1, n: 0 }
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn lsl(&mut self, dst: u8, lhs: u8, rhs: Operand) -> Cycle {
        self.shift_op(u32::wrapping_shl, dst, lhs, rhs)
    }

    pub fn lsr(&mut self, dst: u8, lhs: u8, rhs: Operand) -> Cycle {
        self.shift_op(u32::wrapping_shr, dst, lhs, rhs)
    }

    pub fn asr(&mut self, dst: u8, lhs: u8, rhs: Operand) -> Cycle {
        self.shift_op(u32::wrapping_asr, dst, lhs, rhs)
    }

    pub fn ror(&mut self, dst: u8, lhs: u8, rhs: Operand) -> Cycle {
        self.shift_op(u32::rotate_right, dst, lhs, rhs)
    }

    pub fn add(&mut self, dst: u8, lhs: u8, rhs: Operand, update: bool) -> Cycle {
        self.add_sub_op(dst.into(), lhs.reg(), rhs, Carry::None, update)
    }

    pub fn adc(&mut self, dst: u8, lhs: u8, rhs: Operand, update: bool) -> Cycle {
        self.add_sub_op(dst.into(), lhs.reg(), rhs, Carry::Flag, update)
    }

    pub fn sub(&mut self, dst: u8, lhs: u8, rhs: Operand, update: bool) -> Cycle {
        self.add_sub_op(dst.into(), lhs.reg(), rhs.not(), Carry::One, update)
    }

    pub fn rsb(&mut self, dst: u8, lhs: u8, rhs: Operand, update: bool) -> Cycle {
        self.add_sub_op(dst.into(), rhs, lhs.reg().not(), Carry::One, update)
    }

    pub fn sbc(&mut self, dst: u8, lhs: u8, rhs: Operand, update: bool) -> Cycle {
        self.add_sub_op(dst.into(), lhs.reg(), rhs.not(), Carry::Flag, update)
    }

    pub fn rsc(&mut self, dst: u8, lhs: u8, rhs: Operand, update: bool) -> Cycle {
        self.add_sub_op(dst.into(), rhs, lhs.reg().not(), Carry::Flag, update)
    }

    pub fn cmp(&mut self, lhs: u8, rhs: Operand, update: bool) -> Cycle {
        self.add_sub_op(None, lhs.reg(), rhs.not(), Carry::One, update)
    }

    pub fn cmn(&mut self, lhs: u8, rhs: Operand, update: bool) -> Cycle {
        self.add_sub_op(None, lhs.reg(), rhs, Carry::None, update)
    }

    pub fn neg(&mut self, rd: u8, rs: u8) -> Cycle {
        self.add_sub_op(rd.into(), 0_u32.imm(), rs.reg().not(), Carry::One, true)
    }

    pub fn and(&mut self, dst: u8, lhs: u8, rhs: Operand, update: bool) -> Cycle {
        self.logical_op(u32::bitand, dst.into(), lhs, rhs, update)
    }

    pub fn eor(&mut self, dst: u8, lhs: u8, rhs: Operand, update: bool) -> Cycle {
        self.logical_op(u32::bitxor, dst.into(), lhs, rhs, update)
    }

    pub fn orr(&mut self, dst: u8, lhs: u8, rhs: Operand, update: bool) -> Cycle {
        self.logical_op(u32::bitor, dst.into(), lhs, rhs, update)
    }

    pub fn tst(&mut self, dst: u8, op: Operand, update: bool) -> Cycle {
        self.logical_op(u32::bitand, None, dst, op, update)
    }

    pub fn teq(&mut self, dst: u8, op: Operand, update: bool) -> Cycle {
        self.logical_op(u32::bitxor, None, dst, op, update)
    }

    pub fn bic(&mut self, dst: u8, lhs: u8, rhs: Operand, update: bool) -> Cycle {
        self.logical_op(u32::bitand, dst.into(), lhs, rhs.not(), update)
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

    pub fn mvn(&mut self, rd: u8, op: Operand, update: bool) -> Cycle {
        self.mov_op(rd, op.not(), update)
    }

    pub fn mov(&mut self, rd: u8, op: Operand, update: bool) -> Cycle {
        self.mov_op(rd, op, update)
    }

    pub fn mul(&mut self, dst: u8, lhs: u8, rhs: Operand) -> Cycle {
        self.mul_op(dst, lhs, rhs)
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
        self.branch_op(Condition::EQ, offset)
    }

    pub fn bne(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::NE, offset)
    }

    pub fn bcs(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::CS, offset)
    }

    pub fn bcc(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::CC, offset)
    }

    pub fn bmi(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::MI, offset)
    }

    pub fn bpl(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::PL, offset)
    }

    pub fn bvs(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::VS, offset)
    }

    pub fn bvc(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::VC, offset)
    }

    pub fn bhi(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::HI, offset)
    }

    pub fn bls(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::LS, offset)
    }

    pub fn bge(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::GE, offset)
    }

    pub fn blt(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::LT, offset)
    }

    pub fn bgt(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::GT, offset)
    }

    pub fn ble(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::LE, offset)
    }

    pub fn swi(&mut self) -> Cycle {
        self.handle_exception(Exception::SoftwareInterrupt)
    }

    pub fn b(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::AL, offset)
    }
}

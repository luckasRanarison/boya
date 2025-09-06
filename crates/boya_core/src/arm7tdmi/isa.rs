pub mod prelude {
    pub use std::fmt::Debug;

    pub use crate::arm7tdmi::Arm7tdmi;
    pub use crate::arm7tdmi::common::*;
    pub use crate::arm7tdmi::isa::Executable;
    pub use crate::arm7tdmi::psr::*;
    pub use crate::utils::bitflags::Bitflag;

    #[cfg(test)]
    pub use crate::{bus::Bus, test::*};
}

use std::ops::{BitAnd, BitOr, BitXor};

use prelude::*;

use crate::{
    arm7tdmi::{arm::ArmInstr, thumb::ThumbInstr},
    utils::{bitflags::BitIter, ops::ExtendedOps},
};

pub enum Instruction {
    Arm(ArmInstr),
    Thumb(ThumbInstr),
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Arm(op) => write!(f, "{op:?}"),
            Instruction::Thumb(op) => write!(f, "{op:?}"),
        }
    }
}

pub trait Executable: Sized {
    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle;

    fn condition(&self) -> Condition {
        Condition::AL
    }

    fn dispatch_checked(self, cpu: &mut Arm7tdmi) -> Cycle {
        if cpu.cpsr.matches(self.condition()) {
            self.dispatch(cpu)
        } else {
            Cycle { i: 0, s: 1, n: 0 }
        }
    }
}

impl Arm7tdmi {
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

    pub fn ldrb(&mut self, rd: u8, base: u8, offset: RegisterOffset) -> Cycle {
        self.ldr_op(rd, base, DataType::Byte, false, offset)
    }

    pub fn ldrh(&mut self, rd: u8, base: u8, offset: RegisterOffset) -> Cycle {
        self.ldr_op(rd, base, DataType::HWord, false, offset)
    }

    pub fn ldr(&mut self, rd: u8, base: u8, offset: RegisterOffset) -> Cycle {
        self.ldr_op(rd, base, DataType::Word, false, offset)
    }

    pub fn ldsb(&mut self, rd: u8, base: u8, offset: RegisterOffset) -> Cycle {
        self.ldr_op(rd, base, DataType::Byte, true, offset)
    }

    pub fn ldsh(&mut self, rd: u8, base: u8, offset: RegisterOffset) -> Cycle {
        self.ldr_op(rd, base, DataType::HWord, true, offset)
    }

    pub fn strb(&mut self, rd: u8, base: u8, offset: RegisterOffset) -> Cycle {
        self.str_op(rd, base, DataType::Byte, offset)
    }

    pub fn strh(&mut self, rd: u8, base: u8, offset: RegisterOffset) -> Cycle {
        self.str_op(rd, base, DataType::HWord, offset)
    }

    pub fn str(&mut self, rd: u8, base: u8, offset: RegisterOffset) -> Cycle {
        self.str_op(rd, base, DataType::Word, offset)
    }

    pub fn mvn(&mut self, rd: u8, op: Operand, update: bool) -> Cycle {
        self.mov_op(rd, op.not(), update)
    }

    pub fn mov(&mut self, rd: u8, op: Operand, update: bool) -> Cycle {
        self.mov_op(rd, op, update)
    }

    pub fn mul(&mut self, dst: u8, lhs: u8, rhs: u8, update: bool) -> Cycle {
        self.mul_op(dst.into(), lhs, rhs, None, update, false)
    }

    pub fn mla(&mut self, dst: u8, lhs: u8, rhs: u8, acc: u8, update: bool) -> Cycle {
        self.mul_op(dst.into(), lhs, rhs, Some(acc.into()), update, false)
    }

    pub fn umull(&mut self, dst: LongOperand, lhs: u8, rhs: u8, update: bool) -> Cycle {
        self.mul_op(dst, lhs, rhs, None, update, false)
    }

    pub fn umula(&mut self, dst: LongOperand, lhs: u8, rhs: u8, update: bool) -> Cycle {
        self.mul_op(dst, lhs, rhs, Some(dst), update, false)
    }

    pub fn smull(&mut self, dst: LongOperand, lhs: u8, rhs: u8, update: bool) -> Cycle {
        self.mul_op(dst, lhs, rhs, None, update, true)
    }

    pub fn smula(&mut self, dst: LongOperand, lhs: u8, rhs: u8, update: bool) -> Cycle {
        self.mul_op(dst, lhs, rhs, Some(dst), update, true)
    }

    pub fn bx(&mut self, rs: u8) -> Cycle {
        self.bx_op(rs)
    }

    #[rustfmt::skip]
    pub fn push(&mut self, rlist: u8, lr: bool) -> Cycle {
        self.stm_op(Self::SP, rlist, lr.then_some(Self::LR), AddrMode::DB, true, false)
    }

    #[rustfmt::skip]
    pub fn pop(&mut self, rlist: u8, pc: bool) -> Cycle {
        self.ldm_op(Self::SP, rlist, pc.then_some(Self::PC), AddrMode::IA, true, false)
    }

    pub fn stm<I: BitIter>(&mut self, rl: I, rb: u8, amod: AddrMode, wb: bool, usr: bool) -> Cycle {
        self.stm_op(rb.into(), rl, None, amod, wb, usr)
    }

    pub fn ldm<I: BitIter>(&mut self, rl: I, rb: u8, amod: AddrMode, wb: bool, usr: bool) -> Cycle {
        self.ldm_op(rb.into(), rl, None, amod, wb, usr)
    }

    pub fn beq(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::EQ, offset.into())
    }

    pub fn bne(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::NE, offset.into())
    }

    pub fn bcs(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::CS, offset.into())
    }

    pub fn bcc(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::CC, offset.into())
    }

    pub fn bmi(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::MI, offset.into())
    }

    pub fn bpl(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::PL, offset.into())
    }

    pub fn bvs(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::VS, offset.into())
    }

    pub fn bvc(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::VC, offset.into())
    }

    pub fn bhi(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::HI, offset.into())
    }

    pub fn bls(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::LS, offset.into())
    }

    pub fn bge(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::GE, offset.into())
    }

    pub fn blt(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::LT, offset.into())
    }

    pub fn bgt(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::GT, offset.into())
    }

    pub fn ble(&mut self, offset: i16) -> Cycle {
        self.branch_op(Condition::LE, offset.into())
    }

    pub fn swi(&mut self) -> Cycle {
        self.handle_exception(Exception::SoftwareInterrupt)
    }

    pub fn b(&mut self, offset: i32) -> Cycle {
        self.branch_op(Condition::AL, offset)
    }

    pub fn swp(&mut self, rd: u8, rm: u8, rn: u8, byte: bool) -> Cycle {
        self.swap_op(rd, rm, rn, byte)
    }
}

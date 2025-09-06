mod thumb_01;
mod thumb_02;
mod thumb_03;
mod thumb_04;
mod thumb_05;
mod thumb_06;
mod thumb_07;
mod thumb_08;
mod thumb_09;
mod thumb_10;
mod thumb_11;
mod thumb_12;
mod thumb_13;
mod thumb_14;
mod thumb_15;
mod thumb_16;
mod thumb_17;
mod thumb_18;
mod thumb_19;

use crate::{arm7tdmi::common::Cycle, utils::bitflags::BitArray};

use super::isa::prelude::*;

pub enum ThumbInstr {
    /// Move shifted register
    Format01(thumb_01::Instruction),
    /// Add/Substract
    Format02(thumb_02::Instruction),
    /// Move/Compare/Add/Substract immediate
    Format03(thumb_03::Instruciton),
    /// ALU operations
    Format04(thumb_04::Instruction),
    /// Hi register operations/branch exchange
    Format05(thumb_05::Instruction),
    /// Load PC-relative
    Format06(thumb_06::Instruction),
    /// Load/Store with register offset
    Format07(thumb_07::Instruction),
    /// Load/store sign-extended byte/halfword
    Format08(thumb_08::Instruction),
    /// Load/store with immediate offset
    Format09(thumb_09::Instruction),
    /// Load/store halfword
    Format10(thumb_10::Instruction),
    /// Load/store SP-relative
    Format11(thumb_11::Instruction),
    /// Get relative address
    Format12(thumb_12::Instruction),
    /// Add offset to stack pointer
    Format13(thumb_13::Instruction),
    /// Push/Pop registers
    Format14(thumb_14::Instruction),
    /// Multiple load/store
    Format15(thumb_15::Instruction),
    /// Conditional branch
    Format16(thumb_16::Instruction),
    /// Software interrupt
    Format17(thumb_17::Instruction),
    /// Unconditional branch
    Format18(thumb_18::Instruction),
    /// Long branch with link
    Format19(thumb_19::Instruction),
    /// Undefined THUMB instruction
    Undefined(u16),
}

impl Debug for ThumbInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThumbInstr::Format01(op) => write!(f, "{op:?} ; thumb 01"),
            ThumbInstr::Format02(op) => write!(f, "{op:?} ; thumb 02"),
            ThumbInstr::Format03(op) => write!(f, "{op:?} ; thumb 03"),
            ThumbInstr::Format04(op) => write!(f, "{op:?} ; thumb 04"),
            ThumbInstr::Format05(op) => write!(f, "{op:?} ; thumb 05"),
            ThumbInstr::Format06(op) => write!(f, "{op:?} ; thumb 06"),
            ThumbInstr::Format07(op) => write!(f, "{op:?} ; thumb 07"),
            ThumbInstr::Format08(op) => write!(f, "{op:?} ; thumb 08"),
            ThumbInstr::Format09(op) => write!(f, "{op:?} ; thumb 09"),
            ThumbInstr::Format10(op) => write!(f, "{op:?} ; thumb 10"),
            ThumbInstr::Format11(op) => write!(f, "{op:?} ; thumb 11"),
            ThumbInstr::Format12(op) => write!(f, "{op:?} ; thumb 12"),
            ThumbInstr::Format13(op) => write!(f, "{op:?} ; thumb 13"),
            ThumbInstr::Format14(op) => write!(f, "{op:?} ; thumb 14"),
            ThumbInstr::Format15(op) => write!(f, "{op:?} ; thumb 15"),
            ThumbInstr::Format16(op) => write!(f, "{op:?} ; thumb 16"),
            ThumbInstr::Format17(op) => write!(f, "{op:?} ; thumb 17"),
            ThumbInstr::Format18(op) => write!(f, "{op:?} ; thumb 18"),
            ThumbInstr::Format19(op) => write!(f, "{op:?} ; thumb 19"),

            ThumbInstr::Undefined(op) => write!(f, "{op:x} ; thumb undefined"),
        }
    }
}

impl Arm7tdmi {
    pub fn decode_thumb(&self, word: u32) -> ThumbInstr {
        let word_aligned = self.pc() & 0b1 == 0;
        let (lsb, msb) = if word_aligned { (0, 15) } else { (16, 31) };
        let instr = word.get_bits(lsb, msb) as u16;
        let bit_array = instr.to_bit_array(8);

        match bit_array {
            [1, 1, 1, 1, _, _, _, _] => ThumbInstr::Format19(instr.into()),
            [1, 1, 1, 0, 0, _, _, _] => ThumbInstr::Format18(instr.into()),
            [1, 1, 0, 1, 1, 1, 1, 1] => ThumbInstr::Format17(instr.into()),
            [1, 1, 0, 1, _, _, _, _] => ThumbInstr::Format16(instr.into()),
            [1, 1, 0, 0, _, _, _, _] => ThumbInstr::Format15(instr.into()),
            [1, 0, 1, 1, _, 1, 0, _] => ThumbInstr::Format14(instr.into()),
            [1, 0, 1, 1, 0, 0, 0, 0] => ThumbInstr::Format13(instr.into()),
            [1, 0, 1, 0, _, _, _, _] => ThumbInstr::Format12(instr.into()),
            [1, 0, 0, 1, _, _, _, _] => ThumbInstr::Format11(instr.into()),
            [1, 0, 0, 0, _, _, _, _] => ThumbInstr::Format10(instr.into()),
            [0, 1, 1, _, _, _, _, _] => ThumbInstr::Format09(instr.into()),
            [0, 1, 0, 1, _, _, 1, _] => ThumbInstr::Format08(instr.into()),
            [0, 1, 0, 1, _, _, _, _] => ThumbInstr::Format07(instr.into()),
            [0, 1, 0, 0, 1, _, _, _] => ThumbInstr::Format06(instr.into()),
            [0, 1, 0, 0, 0, 1, _, _] => ThumbInstr::Format05(instr.into()),
            [0, 1, 0, 0, 0, 0, _, _] => ThumbInstr::Format04(instr.into()),
            [0, 0, 1, _, _, _, _, _] => ThumbInstr::Format03(instr.into()),
            [0, 0, 0, 1, 1, _, _, _] => ThumbInstr::Format02(instr.into()),
            [0, 0, 0, _, _, _, _, _] => ThumbInstr::Format01(instr.into()),
            _ => ThumbInstr::Undefined(instr),
        }
    }

    pub fn exec_thumb(&mut self, instruction: ThumbInstr) -> Cycle {
        match instruction {
            ThumbInstr::Format01(op) => op.dispatch(self),
            ThumbInstr::Format02(op) => op.dispatch(self),
            ThumbInstr::Format03(op) => op.dispatch(self),
            ThumbInstr::Format04(op) => op.dispatch(self),
            ThumbInstr::Format05(op) => op.dispatch(self),
            ThumbInstr::Format06(op) => op.dispatch(self),
            ThumbInstr::Format07(op) => op.dispatch(self),
            ThumbInstr::Format08(op) => op.dispatch(self),
            ThumbInstr::Format09(op) => op.dispatch(self),
            ThumbInstr::Format10(op) => op.dispatch(self),
            ThumbInstr::Format11(op) => op.dispatch(self),
            ThumbInstr::Format12(op) => op.dispatch(self),
            ThumbInstr::Format13(op) => op.dispatch(self),
            ThumbInstr::Format14(op) => op.dispatch(self),
            ThumbInstr::Format15(op) => op.dispatch(self),
            ThumbInstr::Format16(op) => op.dispatch(self),
            ThumbInstr::Format17(op) => op.dispatch(self),
            ThumbInstr::Format18(op) => op.dispatch(self),
            ThumbInstr::Format19(op) => op.dispatch(self),

            ThumbInstr::Undefined(_) => self.handle_exception(Exception::Undefined),
        }
    }
}

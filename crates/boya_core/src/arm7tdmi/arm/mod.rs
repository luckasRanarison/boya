mod format_01;
mod format_02;
mod format_03;
mod format_04;
mod format_05;
mod format_06;

use std::fmt::Debug;

use crate::utils::bitflags::BitArray;

use super::isa::prelude::*;

pub enum ArmInstr {
    /// Data processing
    Format01(format_01::Instruction),
    /// PSR transfer
    Format02(format_02::Instruction),
    /// Branch X
    Format03(format_03::Instruction),
    /// Multiply and Multiply-Accumulate
    Format04(format_04::Instruction),
    /// Multiply long and Multiply-Accumulate long
    Format05(format_05::Instruction),
    /// Single data swap
    Format06(format_06::Instruction),
    /// Undefined ARM instruction
    Undefined(u32),
}

impl Debug for ArmInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArmInstr::Format01(op) => write!(f, "{op:?} ; arm 01"),
            ArmInstr::Format02(op) => write!(f, "{op:?} ; arm 02"),
            ArmInstr::Format03(op) => write!(f, "{op:?} ; arm 03"),
            ArmInstr::Format04(op) => write!(f, "{op:?} ; arm 04"),
            ArmInstr::Format05(op) => write!(f, "{op:?} ; arm 05"),
            ArmInstr::Format06(op) => write!(f, "{op:?} ; arm 06"),
            ArmInstr::Undefined(op) => write!(f, "{op:x} ; arm undefined"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    #[rustfmt::skip]
    pub fn decode_arm(&self, word: u32) -> ArmInstr {
        let bit_array = word.to_bit_array(4);

        match bit_array {
            // 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 09 08 07 06 05 04
            [0, 0, 0, 1, 0, _, 0, 0, _, _, _, _, _, _, _, _, 0, 0, 0, 0, 1, 0, 0, 1] => ArmInstr::Format06(word.into()),
            [0, 0, 0, 0, 1, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, 1, 0, 0, 1] => ArmInstr::Format05(word.into()),
            [0, 0, 0, 0, 0, 0, _, _, _, _, _, _, _, _, _, _, _, _, _, _, 1, 0, 0, 1] => ArmInstr::Format04(word.into()),
            [0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1] => ArmInstr::Format03(word.into()),
            [0, 0, _, 1, 0, _, _, 0, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => ArmInstr::Format02(word.into()),
            [0, 0, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => ArmInstr::Format01(word.into()),
            _ => ArmInstr::Undefined(word),
        }
    }

    pub fn exec_arm(&mut self, instruction: ArmInstr) -> Cycle {
        match instruction {
            ArmInstr::Format01(op) => op.dispatch_checked(self),
            ArmInstr::Format02(op) => op.dispatch_checked(self),
            ArmInstr::Format03(op) => op.dispatch_checked(self),
            ArmInstr::Format04(op) => op.dispatch_checked(self),
            ArmInstr::Format05(op) => op.dispatch_checked(self),
            ArmInstr::Format06(op) => op.dispatch_checked(self),
            ArmInstr::Undefined(_) => self.handle_exception(Exception::UndefinedInstruction),
        }
    }
}

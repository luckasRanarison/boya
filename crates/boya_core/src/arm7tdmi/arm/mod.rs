mod format_01;
mod format_02;
mod format_03;
mod format_04;
mod format_05;
mod format_06;
mod format_07;
mod format_08;
mod format_09;
mod format_10;

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
    /// Halfword and Signed data transfer
    Format07(format_07::Instruction),
    /// Block data transfer
    Format08(format_08::Instruction),
    /// Block data transfer
    Format09(format_09::Instruction),
    /// Software interrupt
    Format10(format_10::Instruction),
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
            ArmInstr::Format07(op) => write!(f, "{op:?} ; arm 07"),
            ArmInstr::Format08(op) => write!(f, "{op:?} ; arm 08"),
            ArmInstr::Format09(op) => write!(f, "{op:?} ; arm 09"),
            ArmInstr::Format10(op) => write!(f, "{op:?} ; arm 10"),
            ArmInstr::Undefined(op) => write!(f, "{op:x} ; arm undefined"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    #[rustfmt::skip]
    pub fn decode_arm(&self, word: u32) -> ArmInstr {
        let bit_array = word.to_bit_array(4);

        match bit_array {
            [1, 1, 1, 1, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => ArmInstr::Format10(word.into()),
            [1, 0, 1, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => ArmInstr::Format09(word.into()),
            [1, 0, 0, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => ArmInstr::Format08(word.into()),
            [0, 0, 0, _, _, _, _, 0, _, _, _, _, _, _, _, _, _, _, _, _, 1, 0, 1, 1] |
            [0, 0, 0, _, _, _, _, 1, _, _, _, _, _, _, _, _, _, _, _, _, 1, _, _, 1] => ArmInstr::Format07(word.into()),
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
            ArmInstr::Format07(op) => op.dispatch_checked(self),
            ArmInstr::Format08(op) => op.dispatch_checked(self),
            ArmInstr::Format09(op) => op.dispatch_checked(self),
            ArmInstr::Format10(op) => op.dispatch_checked(self),
            ArmInstr::Undefined(_) => self.handle_exception(Exception::Undefined),
        }
    }
}

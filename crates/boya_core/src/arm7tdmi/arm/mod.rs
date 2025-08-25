mod format_01;
mod format_02;

use std::fmt::Debug;

use crate::{
    arm7tdmi::{
        Arm7tdmi,
        arm::{format_01::Format1, format_02::Format2},
        common::{Cycle, Exception},
        isa::Executable,
    },
    bus::Bus,
    utils::bitflags::BitArray,
};

pub enum ArmInstr {
    /// Data processing
    Format1(Format1),
    /// PSR transfer
    Format2(Format2),
    /// Undefined ARM instruction
    Undefined(u32),
}

impl Debug for ArmInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArmInstr::Format1(op) => write!(f, "{op:?} ; arm 1"),
            ArmInstr::Format2(op) => write!(f, "{op:?} ; arm 2"),
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
            [0, 0, _, 1, 0, _, _, 0, 1, 1, 1, 1, _, _, _, _, _, _, _, _, _, _, _, _] |
            [0, 0, _, 1, 0, _, _, 0, _, _, _, _, 1, 1, 1, 1, _, _, _, _, _, _, _, _] |
            [0, 0, _, 1, 0, _, _, 0, _, _, _, _, _, _, _, _, 0, 0, 0, 0, 0, 0, 0, 0] => ArmInstr::Format2(word.into()),
            [0, 0, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => ArmInstr::Format1(word.into()),
            _ => ArmInstr::Undefined(word),
        }
    }

    pub fn exec_arm(&mut self, instruction: ArmInstr) -> Cycle {
        match instruction {
            ArmInstr::Format1(op) => op.dispatch_checked(self),
            ArmInstr::Format2(op) => op.dispatch_checked(self),
            ArmInstr::Undefined(_) => self.handle_exception(Exception::UndefinedInstruction),
        }
    }
}

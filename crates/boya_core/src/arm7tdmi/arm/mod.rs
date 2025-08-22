mod format_01;

use std::fmt::Debug;

use crate::{
    arm7tdmi::{
        Arm7tdmi,
        arm::format_01::Format1,
        common::{Cycle, Exception},
        isa::Executable,
    },
    bus::Bus,
    utils::bitflags::BitArray,
};

pub enum ArmInstr {
    /// Data processing
    Format1(Format1),
    /// Undefined ARM instruction
    Undefined(u32),
}

impl Debug for ArmInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArmInstr::Format1(op) => write!(f, "{op:?} ; arm 1"),
            ArmInstr::Undefined(op) => write!(f, "{op:x} ; arm undefined"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn decode_arm(&self, word: u32) -> ArmInstr {
        let bit_array = word.to_bit_array(25);

        match bit_array {
            [0, 0, _] => ArmInstr::Format1(word.into()),
            _ => ArmInstr::Undefined(word),
        }
    }

    pub fn exec_arm(&mut self, instruction: ArmInstr) -> Cycle {
        match instruction {
            ArmInstr::Format1(op) => op.dispatch_checked(self),
            ArmInstr::Undefined(_) => self.handle_exception(Exception::UndefinedInstruction),
        }
    }
}

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
mod format_11;
mod format_12;
mod format_13;
mod format_14;
mod format_15;
mod format_16;
mod format_17;
mod format_18;
mod format_19;

mod prelude {
    pub use std::fmt::Debug;

    pub use crate::arm7tdmi::Arm7tdmi;
    pub use crate::arm7tdmi::common::{Operand, ToOperand};
    pub use crate::bus::Bus;
    pub use crate::utils::bitflags::Bitflag;

    #[cfg(test)]
    pub use crate::{arm7tdmi::test::Psr, test::AsmTestBuilder};
}

use format_01::Format1;
use format_02::Format2;
use format_03::Format3;
use format_04::Format4;
use format_05::Format5;
use format_06::Format6;
use format_07::Format7;
use format_08::Format8;
use format_09::Format9;

use format_10::Format10;
use format_11::Format11;
use format_12::Format12;
use format_13::Format13;
use format_14::Format14;
use format_15::Format15;
use format_16::Format16;
use format_17::Format17;
use format_18::Format18;

use prelude::*;

use crate::{arm7tdmi::thumb::format_19::Format19, utils::bitflags::BitArray};

pub enum ThumbInstr {
    /// Move shifted register
    Format1(Format1),
    /// Add/Substract
    Format2(Format2),
    /// Move/Compare/Add/Substract immediate
    Format3(Format3),
    /// ALU operations
    Format4(Format4),
    /// Hi register operations/branch exchange
    Format5(Format5),
    /// Load PC-relative
    Format6(Format6),
    /// Load/Store with register offset
    Format7(Format7),
    /// Load/store sign-extended byte/halfword
    Format8(Format8),
    /// Load/store with immediate offset
    Format9(Format9),
    /// Load/store halfword
    Format10(Format10),
    /// Load/store SP-relative
    Format11(Format11),
    /// Get relative address
    Format12(Format12),
    /// Add offset to stack pointer
    Format13(Format13),
    /// Push/Pop registers
    Format14(Format14),
    /// Multiple load/store
    Format15(Format15),
    /// Conditional branch
    Format16(Format16),
    /// Software interrupt
    Format17(Format17),
    /// Unconditional branch
    Format18(Format18),
    /// Long branch with link
    Format19(Format19),
}

impl Debug for ThumbInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThumbInstr::Format1(op) => write!(f, "{op:?} ; thumb 1"),
            ThumbInstr::Format2(op) => write!(f, "{op:?} ; thumb 2"),
            ThumbInstr::Format3(op) => write!(f, "{op:?} ; thumb 3"),
            ThumbInstr::Format4(op) => write!(f, "{op:?} ; thumb 4"),
            ThumbInstr::Format5(op) => write!(f, "{op:?} ; thumb 5"),
            ThumbInstr::Format6(op) => write!(f, "{op:?} ; thumb 6"),
            ThumbInstr::Format7(op) => write!(f, "{op:?} ; thumb 7"),
            ThumbInstr::Format8(op) => write!(f, "{op:?} ; thumb 8"),
            ThumbInstr::Format9(op) => write!(f, "{op:?} ; thumb 9"),

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
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn decode_thumb(&self, word: u32) -> ThumbInstr {
        let word_aligned = self.pc() & 0b1 == 0;
        let (lsb, msb) = if word_aligned { (0, 15) } else { (16, 31) };
        let instr = word.get_bits(lsb, msb) as u16;
        let bit_array = instr.to_bit_array(8);

        match bit_array {
            [0, 0, 0, 1, 1, _, _, _] => ThumbInstr::Format2(Format2::from(instr)),
            [0, 0, 0, _, _, _, _, _] => ThumbInstr::Format1(Format1::from(instr)),
            [0, 0, 1, _, _, _, _, _] => ThumbInstr::Format3(Format3::from(instr)),
            [0, 1, 0, 0, 0, 0, _, _] => ThumbInstr::Format4(Format4::from(instr)),
            [0, 1, 0, 0, 0, 1, _, _] => ThumbInstr::Format5(Format5::from(instr)),
            [0, 1, 0, 0, 1, _, _, _] => ThumbInstr::Format6(Format6::from(instr)),
            [0, 1, 0, 1, _, _, 1, _] => ThumbInstr::Format8(Format8::from(instr)),
            [0, 1, 0, 1, _, _, _, _] => ThumbInstr::Format7(Format7::from(instr)),
            [0, 1, 1, _, _, _, _, _] => ThumbInstr::Format9(Format9::from(instr)),
            [1, 0, 0, 0, _, _, _, _] => ThumbInstr::Format10(Format10::from(instr)),
            [1, 0, 0, 1, _, _, _, _] => ThumbInstr::Format11(Format11::from(instr)),
            [1, 0, 1, 0, _, _, _, _] => ThumbInstr::Format12(Format12::from(instr)),
            [1, 0, 1, 1, 0, 0, 0, 0] => ThumbInstr::Format13(Format13::from(instr)),
            [1, 0, 1, 1, _, 1, 0, _] => ThumbInstr::Format14(Format14::from(instr)),
            [1, 1, 0, 0, _, _, _, _] => ThumbInstr::Format15(Format15::from(instr)),
            [1, 1, 0, 1, 1, 1, 1, 1] => ThumbInstr::Format17(Format17::from(instr)),
            [1, 1, 0, 1, _, _, _, _] => ThumbInstr::Format16(Format16::from(instr)),
            [1, 1, 1, 0, 0, _, _, _] => ThumbInstr::Format18(Format18::from(instr)),
            [1, 1, 1, 1, _, _, _, _] => ThumbInstr::Format19(Format19::from(instr)),
            _ => todo!(),
        }
    }

    pub fn exec_thumb(&mut self, instruction: ThumbInstr) {
        match instruction {
            ThumbInstr::Format1(op) => self.exec_thumb_format1(op),
            ThumbInstr::Format2(op) => self.exec_thumb_format2(op),
            ThumbInstr::Format3(op) => self.exec_thumb_format3(op),
            ThumbInstr::Format4(op) => self.exec_thumb_format4(op),
            ThumbInstr::Format5(op) => self.exec_thumb_format5(op),
            ThumbInstr::Format6(op) => self.exec_thumb_format6(op),
            ThumbInstr::Format7(op) => self.exec_thumb_format7(op),
            ThumbInstr::Format8(op) => self.exec_thumb_format8(op),
            ThumbInstr::Format9(op) => self.exec_thumb_format9(op),

            ThumbInstr::Format10(op) => self.exec_thumb_format10(op),
            ThumbInstr::Format11(op) => self.exec_thumb_format11(op),
            ThumbInstr::Format12(op) => self.exec_thumb_format12(op),
            ThumbInstr::Format13(op) => self.exec_thumb_format13(op),
            ThumbInstr::Format14(op) => self.exec_thumb_format14(op),
            ThumbInstr::Format15(op) => self.exec_thumb_format15(op),
            ThumbInstr::Format16(op) => self.exec_thumb_format16(op),
            ThumbInstr::Format17(op) => self.exec_thumb_format17(op),
            ThumbInstr::Format18(op) => self.exec_thumb_format18(op),
            ThumbInstr::Format19(op) => self.exec_thumb_format19(op),
        }
    }
}

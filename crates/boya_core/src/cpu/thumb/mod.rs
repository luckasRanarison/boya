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

use crate::utils::bitflags::BitArray;

use super::isa::prelude::*;

pub enum ThumbInstr {
    /// Move shifted register
    Format01(thumb_01::Instruction),
    /// Add/Substract
    Format02(thumb_02::Instruction),
    /// Move/Compare/Add/Substract immediate
    Format03(thumb_03::Instruction),
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

impl From<ThumbInstr> for DebuggableInstruction {
    fn from(value: ThumbInstr) -> Self {
        match value {
            ThumbInstr::Format01(op) => op.into(),
            ThumbInstr::Format02(op) => op.into(),
            ThumbInstr::Format03(op) => op.into(),
            ThumbInstr::Format04(op) => op.into(),
            ThumbInstr::Format05(op) => op.into(),
            ThumbInstr::Format06(op) => op.into(),
            ThumbInstr::Format07(op) => op.into(),
            ThumbInstr::Format08(op) => op.into(),
            ThumbInstr::Format09(op) => op.into(),
            ThumbInstr::Format10(op) => op.into(),
            ThumbInstr::Format11(op) => op.into(),
            ThumbInstr::Format12(op) => op.into(),
            ThumbInstr::Format13(op) => op.into(),
            ThumbInstr::Format14(op) => op.into(),
            ThumbInstr::Format15(op) => op.into(),
            ThumbInstr::Format16(op) => op.into(),
            ThumbInstr::Format17(op) => op.into(),
            ThumbInstr::Format18(op) => op.into(),
            ThumbInstr::Format19(op) => op.into(),
            ThumbInstr::Undefined(op) => UndefinedInstruction::Thumb(op).into(),
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_REG: usize = 7;
    const TEST_START: u32 = 0x0800_00FC;
    const TEST_END: u32 = 0x0800_0930;

    const TEST_FILE: &[u8] = include_bytes!("../../../../../submodules/gba-tests/thumb/thumb.gba");

    #[test]
    fn test_thumb_suite() {
        GbaTestBuilder::new()
            .bytes(TEST_FILE)
            .setup(|cpu| {
                cpu.cpsr.update(Psr::T, true);
                cpu.override_pc(TEST_START);
            })
            .assert_fn(|cpu| {
                let test = cpu.registers.get(TEST_REG, cpu.cpsr.op_mode());
                assert_eq!(test, 0, "test t{test} failed");
            })
            .run_while(|cpu| cpu.pc() < TEST_END);
    }
}

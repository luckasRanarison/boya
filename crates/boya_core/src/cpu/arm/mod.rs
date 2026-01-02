mod arm_03;
mod arm_04;
mod arm_05;
mod arm_06;
mod arm_07;
mod arm_08;
mod arm_09;
mod arm_10;
mod arm_11;
mod arm_12;
mod arm_13;

use crate::utils::bitflags::BitArray;

use super::isa::prelude::*;

pub enum Arm {
    /// Branch X
    Arm03(arm_03::Instruction),
    /// Branch and Branch with Link
    Arm04(arm_04::Instruction),
    /// Data processing
    Arm05(arm_05::Instruction),
    /// PSR transfer
    Arm06(arm_06::Instruction),
    /// Multiply and Multiply-Accumulate
    Arm07(arm_07::Instruction),
    /// Multiply long and Multiply-Accumulate long
    Arm08(arm_08::Instruction),
    /// Single data transfer
    Arm09(arm_09::Instruction),
    /// Halfword and Signed data transfer
    Arm10(arm_10::Instruction),
    /// Block data transfer
    Arm11(arm_11::Instruction),
    /// Single data swap
    Arm12(arm_12::Instruction),
    /// Software interrupt
    Arm13(arm_13::Instruction),
    /// Undefined ARM instruction
    Undefined(u32),
}

impl Arm {
    pub fn get_data(&self) -> InstructionData {
        match self {
            Arm::Arm03(op) => op.get_data(),
            Arm::Arm04(op) => op.get_data(),
            Arm::Arm05(op) => op.get_data(),
            Arm::Arm06(op) => op.get_data(),
            Arm::Arm07(op) => op.get_data(),
            Arm::Arm08(op) => op.get_data(),
            Arm::Arm09(op) => op.get_data(),
            Arm::Arm10(op) => op.get_data(),
            Arm::Arm11(op) => op.get_data(),
            Arm::Arm12(op) => op.get_data(),
            Arm::Arm13(op) => op.get_data(),
            Arm::Undefined(op) => InstructionData::undefined_arm(*op),
        }
    }
}

impl Arm7tdmi {
    #[rustfmt::skip]
    pub fn decode_arm(&self, word: u32) -> Arm {
        let bit_array = word.to_bit_array(4);

        match bit_array {
            [1, 1, 1, 1, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => Arm::Arm13(word.into()),
            [1, 0, 1, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => Arm::Arm04(word.into()),
            [1, 0, 0, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => Arm::Arm11(word.into()),
            [0, 1, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => Arm::Arm09(word.into()),
            [0, 0, 0, 0, 1, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, 1, 0, 0, 1] => Arm::Arm08(word.into()),
            [0, 0, 0, _, _, _, _, 0, _, _, _, _, _, _, _, _, _, _, _, _, 1, 0, 1, 1] => Arm::Arm10(word.into()),
            [0, 0, 0, _, _, _, _, 1, _, _, _, _, _, _, _, _, _, _, _, _, 1, _, _, 1] => Arm::Arm10(word.into()),
            [0, 0, 0, 1, 0, _, 0, 0, _, _, _, _, _, _, _, _, 0, 0, 0, 0, 1, 0, 0, 1] => Arm::Arm12(word.into()),
            [0, 0, 0, 0, 0, 0, _, _, _, _, _, _, _, _, _, _, _, _, _, _, 1, 0, 0, 1] => Arm::Arm07(word.into()),
            [0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1] => Arm::Arm03(word.into()),
            [0, 0, _, 1, 0, _, _, 0, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => Arm::Arm06(word.into()),
            [0, 0, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => Arm::Arm05(word.into()),
            _ => Arm::Undefined(word),
        }
    }

    pub fn exec_arm(&mut self, instruction: Arm) -> Cycle {
        match instruction {
            Arm::Arm03(op) => op.dispatch_checked(self),
            Arm::Arm04(op) => op.dispatch_checked(self),
            Arm::Arm05(op) => op.dispatch_checked(self),
            Arm::Arm06(op) => op.dispatch_checked(self),
            Arm::Arm07(op) => op.dispatch_checked(self),
            Arm::Arm08(op) => op.dispatch_checked(self),
            Arm::Arm09(op) => op.dispatch_checked(self),
            Arm::Arm10(op) => op.dispatch_checked(self),
            Arm::Arm11(op) => op.dispatch_checked(self),
            Arm::Arm12(op) => op.dispatch_checked(self),
            Arm::Arm13(op) => op.dispatch_checked(self),
            Arm::Undefined(_) => self.handle_exception(Exception::Undefined),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_REG: usize = 12;
    const TEST_START: u32 = 0x0800_00F4;
    const TEST_END: u32 = 0x0800_1D4C;

    const TEST_FILE: &[u8] = include_bytes!("../../../../../submodules/gba-tests/arm/arm.gba");

    #[test]
    fn test_arm_suite() {
        GbaTestBuilder::new()
            .bytes(TEST_FILE)
            .pc(TEST_START)
            .assert_fn(|cpu| {
                let test = cpu.registers.get(TEST_REG, cpu.cpsr.op_mode());
                assert_eq!(test, 0, "test t{test} failed");
            })
            .run_while(|cpu| cpu.pc() <= TEST_END);
    }
}

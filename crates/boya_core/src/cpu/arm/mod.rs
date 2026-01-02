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

pub enum ArmInstr {
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

impl Debug for ArmInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArmInstr::Arm03(op) => write!(f, "{op:?} ; arm 03"),
            ArmInstr::Arm04(op) => write!(f, "{op:?} ; arm 04"),
            ArmInstr::Arm05(op) => write!(f, "{op:?} ; arm 05"),
            ArmInstr::Arm06(op) => write!(f, "{op:?} ; arm 06"),
            ArmInstr::Arm07(op) => write!(f, "{op:?} ; arm 07"),
            ArmInstr::Arm08(op) => write!(f, "{op:?} ; arm 08"),
            ArmInstr::Arm09(op) => write!(f, "{op:?} ; arm 09"),
            ArmInstr::Arm10(op) => write!(f, "{op:?} ; arm 10"),
            ArmInstr::Arm11(op) => write!(f, "{op:?} ; arm 11"),
            ArmInstr::Arm12(op) => write!(f, "{op:?} ; arm 12"),
            ArmInstr::Arm13(op) => write!(f, "{op:?} ; arm 13"),
            ArmInstr::Undefined(op) => write!(f, "{op:x} ; arm undefined"),
        }
    }
}

impl Arm7tdmi {
    #[rustfmt::skip]
    pub fn decode_arm(&self, word: u32) -> ArmInstr {
        let bit_array = word.to_bit_array(4);

        match bit_array {
            [1, 1, 1, 1, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => ArmInstr::Arm13(word.into()),
            [1, 0, 1, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => ArmInstr::Arm04(word.into()),
            [1, 0, 0, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => ArmInstr::Arm11(word.into()),
            [0, 1, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => ArmInstr::Arm09(word.into()),
            [0, 0, 0, 0, 1, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, 1, 0, 0, 1] => ArmInstr::Arm08(word.into()),
            [0, 0, 0, _, _, _, _, 0, _, _, _, _, _, _, _, _, _, _, _, _, 1, 0, 1, 1] => ArmInstr::Arm10(word.into()),
            [0, 0, 0, _, _, _, _, 1, _, _, _, _, _, _, _, _, _, _, _, _, 1, _, _, 1] => ArmInstr::Arm10(word.into()),
            [0, 0, 0, 1, 0, _, 0, 0, _, _, _, _, _, _, _, _, 0, 0, 0, 0, 1, 0, 0, 1] => ArmInstr::Arm12(word.into()),
            [0, 0, 0, 0, 0, 0, _, _, _, _, _, _, _, _, _, _, _, _, _, _, 1, 0, 0, 1] => ArmInstr::Arm07(word.into()),
            [0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1] => ArmInstr::Arm03(word.into()),
            [0, 0, _, 1, 0, _, _, 0, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => ArmInstr::Arm06(word.into()),
            [0, 0, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => ArmInstr::Arm05(word.into()),
            _ => ArmInstr::Undefined(word),
        }
    }

    pub fn exec_arm(&mut self, instruction: ArmInstr) -> Cycle {
        match instruction {
            ArmInstr::Arm03(op) => op.dispatch_checked(self),
            ArmInstr::Arm04(op) => op.dispatch_checked(self),
            ArmInstr::Arm05(op) => op.dispatch_checked(self),
            ArmInstr::Arm06(op) => op.dispatch_checked(self),
            ArmInstr::Arm07(op) => op.dispatch_checked(self),
            ArmInstr::Arm08(op) => op.dispatch_checked(self),
            ArmInstr::Arm09(op) => op.dispatch_checked(self),
            ArmInstr::Arm10(op) => op.dispatch_checked(self),
            ArmInstr::Arm11(op) => op.dispatch_checked(self),
            ArmInstr::Arm12(op) => op.dispatch_checked(self),
            ArmInstr::Arm13(op) => op.dispatch_checked(self),
            ArmInstr::Undefined(_) => self.handle_exception(Exception::Undefined),
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

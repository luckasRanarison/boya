mod format_1;
mod format_2;
mod format_3;
mod format_4;
mod format_5;
mod format_6;
mod format_7;
mod format_8;
mod format_9;

mod prelude {
    pub use std::fmt::Debug;

    pub use crate::arm7tdmi::Arm7tdmi;
    pub use crate::arm7tdmi::common::{Operand, ToOperand};
    pub use crate::bus::Bus;
    pub use crate::utils::bitflags::Bitflag;
}

use format_1::Format1;
use format_2::Format2;
use format_3::Format3;
use format_4::Format4;
use format_5::Format5;
use format_6::Format6;
use format_7::Format7;
use format_8::Format8;
use format_9::Format9;

use prelude::*;

pub enum InstructionFormat {
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
}

impl Debug for InstructionFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionFormat::Format1(op) => write!(f, "{op:?} ; format 1"),
            InstructionFormat::Format2(op) => write!(f, "{op:?} ; format 2"),
            InstructionFormat::Format3(op) => write!(f, "{op:?} ; format 3"),
            InstructionFormat::Format4(op) => write!(f, "{op:?} ; format 4"),
            InstructionFormat::Format5(op) => write!(f, "{op:?} ; format 5"),
            InstructionFormat::Format6(op) => write!(f, "{op:?} ; format 6"),
            InstructionFormat::Format7(op) => write!(f, "{op:?} ; format 7"),
            InstructionFormat::Format8(op) => write!(f, "{op:?} ; format 8"),
            InstructionFormat::Format9(op) => write!(f, "{op:?} ; format 9"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn decode_thumb(&self, word: u32) -> InstructionFormat {
        let word_aligned = self.pc() & 0b1 == 0;
        let (lsb, msb) = if word_aligned { (0, 15) } else { (16, 31) };
        let instr = word.get_bits(lsb, msb) as u16;

        // the order is important!
        if instr.get_bits(11, 15) == 0b00011 {
            InstructionFormat::Format2(Format2::from(instr))
        } else if instr.get_bits(13, 15) == 0b000 {
            InstructionFormat::Format1(Format1::from(instr))
        } else if instr.get_bits(13, 15) == 0b001 {
            InstructionFormat::Format3(Format3::from(instr))
        } else if instr.get_bits(10, 15) == 0b010000 {
            InstructionFormat::Format4(Format4::from(instr))
        } else if instr.get_bits(10, 15) == 0b010001 {
            InstructionFormat::Format5(Format5::from(instr))
        } else if instr.get_bits(11, 15) == 0b01001 {
            InstructionFormat::Format6(Format6::from(instr))
        } else if instr.get_bits(12, 15) == 0b0101 && instr.has(9) {
            InstructionFormat::Format8(Format8::from(instr))
        } else if instr.get_bits(12, 15) == 0b0101 {
            InstructionFormat::Format7(Format7::from(instr))
        } else if instr.get_bits(13, 15) == 0b011 {
            InstructionFormat::Format9(Format9::from(instr))
        } else {
            todo!()
        }
    }

    pub fn exec_thumb(&mut self, instruction: InstructionFormat) {
        match instruction {
            InstructionFormat::Format1(op) => self.exec_thumb_format1(op),
            InstructionFormat::Format2(op) => self.exec_thumb_format2(op),
            InstructionFormat::Format3(op) => self.exec_thumb_format3(op),
            InstructionFormat::Format4(op) => self.exec_thumb_format4(op),
            InstructionFormat::Format5(op) => self.exec_thumb_format5(op),
            InstructionFormat::Format6(op) => self.exec_thumb_format6(op),
            InstructionFormat::Format7(op) => self.exec_thumb_format7(op),
            InstructionFormat::Format8(op) => self.exec_thumb_format8(op),
            InstructionFormat::Format9(op) => self.exec_thumb_format9(op),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{arm7tdmi::psr::Psr, bus::Bus, test::AsmTestBuilder};

    #[test]
    fn test_move() {
        let asm = r"
            mov r1, 5
            mvn r2, r1
            mov r3, 0
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(1, 5)
            .assert_reg(2, !5)
            .assert_flag(Psr::Z, true)
            .assert_flag(Psr::N, false)
            .run(3);
    }

    #[test]
    fn test_logic_shift() {
        let asm = r"
            mov r1, 2
            lsl r2, r1, 2
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(2, 8)
            .assert_flag(Psr::C, true)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, false)
            .run(2);
    }

    #[test]
    fn test_arithmetic_shift() {
        let asm = r"
            mov r0, 0
            mvn r1, r0
            asr r2, r1, 1
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(2, !0)
            .assert_flag(Psr::C, true)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, true)
            .run(3);
    }

    #[test]
    fn test_add_sub_basic() {
        let asm = r"
            mov r0, 2
            mov r1, 3
            add r2, r1, r0
            add r3, r2, 3 
            sub r4, r3, r0
            cmp r4, 6
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(2, 5)
            .assert_reg(3, 8)
            .assert_reg(4, 6)
            .assert_flag(Psr::C, true) // no borrow
            .assert_flag(Psr::Z, true)
            .assert_flag(Psr::N, false)
            .assert_flag(Psr::V, false)
            .run(6);
    }

    #[test]
    fn test_add_carry() {
        let asm = r"
            mov r0, 5
            sub r1, r0, 2
            adc r0, r1
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(0, 9)
            .assert_flag(Psr::C, false)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, false)
            .assert_flag(Psr::V, false)
            .run(4);
    }

    #[test]
    fn test_sub_carry() {
        let asm = r"
            mov r0, 5
            mov r1, 1
            sbc r0, r1
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(0, 3)
            .assert_flag(Psr::C, false)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, false)
            .assert_flag(Psr::V, false)
            .run(4);
    }

    #[test]
    fn test_negation() {
        let asm = r"
            mov r0, 2
            neg r1, r0
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(1, -2i32 as u32)
            .assert_flag(Psr::C, false)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, true)
            .assert_flag(Psr::V, false)
            .run(2);
    }

    #[test]
    fn test_logical_op() {
        let asm = r"
            mov r0, 2
            mov r1, 1
            mov r2, 2
            mov r3, 3
            orr r1, r0
            bic r3, r2
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(1, 3)
            .assert_reg(3, 1)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, false)
            .run(6);
    }

    #[test]
    fn test_mul_basic() {
        let asm = r"
            mov r0, 2
            mov r1, 3
            sub r2, r1, r0 ; sets carry
            mul r0, r1
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(0, 6)
            .assert_flag(Psr::C, false)
            .run(4);
    }

    #[test]
    fn test_hi_reg_ops() {
        let asm = r"
            mov r0, 5
            mov pc, r0
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(15, 6) // half-word alignement
            .run(2);
    }

    #[test]
    fn test_ldr_pc_offset() {
        AsmTestBuilder::new()
            .thumb()
            .setup(|cpu| cpu.bus.write_u32(20, 5))
            .asm("ldr r1, [PC, #16]")
            .assert_reg(1, 5)
            .run(1);
    }

    #[test]
    fn test_ldr_str_reg_offset() {
        let asm = r"
            mov r0, 3
            mov r1, 5
            mov r2, 6
            str r0, [r1, r2]
            ldr r3, [r1, r2]
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_mem(11, 3)
            .assert_reg(3, 3)
            .run(5);
    }

    #[test]
    fn test_lds() {
        let asm = r"
            mov   r0, 10
            mov   r1, 11
            ldrsb r2, [r0, r0]
            ldrsh r3, [r0, r1]
        ";

        AsmTestBuilder::new()
            .thumb()
            .setup(|cpu| {
                cpu.bus.write_u8(20, -1_i8 as u8);
                cpu.bus.write_u16(21, -5_i16 as u16);
            })
            .asm(asm)
            .assert_reg(2, -1_i32 as u32)
            .assert_reg(3, -5_i32 as u32)
            .run(4);
    }

    #[test]
    fn test_ldr_immediate() {
        let asm = r"
            mov r0, 7
            ldr r1, [r0, 116]
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .setup(|cpu| cpu.bus.write_u32(123, 5))
            .assert_reg(1, 5)
            .run(2);
    }
}

use crate::cpu::isa::prelude::*;

/// Unconditional branch
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  1 |  1 |  0 |  0 |                     Offset11                         |
/// +-------------------------------------------------------------------------------+
#[derive(Debug)]
pub struct Instruction {
    of: i16,
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        Self {
            of: ((value.get_bits(0, 10) << 5) as i16) >> 4, // sign-exteneded + steps 2
        }
    }
}

impl Executable for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        cpu.b(self.of.into())
    }

    fn get_data(&self) -> InstructionData {
        InstructionData {
            keyword: "B".into(),
            args: vec![InstructionParam::BranchOffset(self.of.into())],
            kind: InstructionKind::thumb(18),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unconditional_branch() {
        let asm = r"
            main:
                b   target ; 0
                mov r0, #1 ; 2

            back:
                mov r2, #4 ; 4

            target:
                mov r1, #2 ; 6
                b   back   ; 8
        ";

        GbaTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(0, 0)
            .assert_reg(1, 2)
            .assert_reg(2, 4)
            .assert_reg(15, TMB_MAIN_START + 10) // + pre-fetch 6
            .run(4)
    }
}

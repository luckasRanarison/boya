use crate::cpu::isa::prelude::*;

/// Long branch with link
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  1 |  1 |  1 |  H |                      Offset                          |
/// +-------------------------------------------------------------------------------+
#[derive(Debug)]
pub struct Instruction {
    h: bool,
    nn: u16,
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let h = value.has(11);
        let nn = value.get_bits(0, 10);

        Self { h, nn }
    }
}

impl Executable for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        match self.h {
            false => cpu.branch_long_first_op(self.nn),
            true => cpu.branch_long_second_op(self.nn),
        }
    }

    fn get_data(&self) -> InstructionData {
        InstructionData {
            keyword: format!("{:#06X}", ((self.h as u16) << 11) | self.nn),
            args: vec![],
            kind: InstructionKind::thumb(19),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_looooong_branch() {
        let asm = r"
            main:
                bl  target ; 0-2
                mov r0, #1 ; 4

            last:
                mov r2, #4 ; 6

            target:
                mov r1, #2 ; 8
                bl last    ; 10-12
        ";

        GbaTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(0, 0)
            .assert_reg(1, 2)
            .assert_reg(2, 4)
            .assert_reg(15, TMB_MAIN_START + 12) // + pre-fetch 6
            .run(6)
    }
}

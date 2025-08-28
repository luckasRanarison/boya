use crate::arm7tdmi::isa::prelude::*;

/// Long branch with link
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  1 |  1 |  1 |  H |                      Offset                          |
/// +-------------------------------------------------------------------------------+
pub struct Instruction {
    h: bool,
    nn: u16,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.h { "; BL label" } else { "BL label" })
    }
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let h = value.has(11);
        let nn = value.get_bits(0, 10);

        Self { h, nn }
    }
}

impl<B: Bus> Executable<B> for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        match self.h {
            false => cpu.branch_long_first_op(self.nn),
            true => cpu.branch_long_second_op(self.nn),
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

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(0, 0)
            .assert_reg(1, 2)
            .assert_reg(2, 4)
            .assert_reg(15, 12) // + pre-fetch 6
            .run(6)
    }
}

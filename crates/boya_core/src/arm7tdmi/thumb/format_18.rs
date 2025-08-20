use super::prelude::*;

/// Unconditional branch
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  1 |  1 |  0 |  0 |                     Offset11                         |
/// +-------------------------------------------------------------------------------+
pub struct Format18 {
    of: i16,
}

impl Debug for Format18 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "B {:?}", self.of)
    }
}

impl From<u16> for Format18 {
    fn from(value: u16) -> Self {
        Self {
            of: ((value.get_bits(0, 10) as i8) as i16) << 1,
        }
    }
}

impl<B: Bus> Executable<B> for Format18 {
    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        cpu.b(self.of)
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

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(0, 0)
            .assert_reg(1, 2)
            .assert_reg(2, 4)
            .assert_reg(15, 10) // + pre-fetch 6
            .run(4)
    }
}

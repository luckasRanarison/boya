use super::prelude::*;

/// Add offset to stack pointer
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  0 |  1 |  1 |  0 |  0 |  0 |  0 | Op |            SWord7                |
/// +-------------------------------------------------------------------------------+
pub struct Format13 {
    nn: Operand, // 0-1020, step 4
}

impl Debug for Format13 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ADD SP, {:?}", self.nn)
    }
}

impl From<u16> for Format13 {
    fn from(value: u16) -> Self {
        let nn = value.get_bits(0, 6) << 2;

        let nn = match value.get(7) {
            0 => nn.imm(),
            _ => nn.imm().not(),
        };

        Self { nn }
    }
}

impl<B: Bus> Executable<B> for Format13 {
    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        let sp = NamedRegister::SP as u8;
        let nn = self.nn.value.imm();

        match self.nn.negate {
            true => cpu.sub(sp, nn, sp, false),
            false => cpu.add(sp, nn, sp, false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_offset_sp() {
        let asm = r"
            add SP, 8
            add SP, -4
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(13, 208 - 4)
            .run(2);
    }
}

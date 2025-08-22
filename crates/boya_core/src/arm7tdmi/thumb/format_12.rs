use crate::arm7tdmi::isa::prelude::*;

/// Get relative address
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  0 |  1 |  0 | Op |      Rd      |                Offset8                |
/// +-------------------------------------------------------------------------------+
pub struct Format12 {
    rs: Operand,
    nn: u16, // 0-1020, steps 4
    rd: u8,
}

impl Debug for Format12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ADD {:?}, {:?}, {:?}",
            self.rd.reg(),
            self.rs,
            self.nn.imm()
        )
    }
}

impl From<u16> for Format12 {
    fn from(value: u16) -> Self {
        let rs = match value.get(11) {
            0 => Operand::pc(),
            _ => Operand::sp(),
        };

        let rd = value.get_bits_u8(8, 10);
        let nn = value.get_bits(0, 7) << 2;

        Self { rs, nn, rd }
    }
}

impl<B: Bus> Executable<B> for Format12 {
    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        cpu.add(self.rd, self.rs.value as u8, self.nn.imm(), false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_relative_addr() {
        let asm = r"
            add r2, PC, 12
            add r3, SP, 8
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(2, 16)
            .assert_reg(3, 208)
            .run(2);
    }
}

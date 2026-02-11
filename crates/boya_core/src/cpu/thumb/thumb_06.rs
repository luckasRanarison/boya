pub use crate::cpu::isa::prelude::*;

/// Load PC-relative
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  1 |  0 |  0 |  1 |      Rd      |              Offset8                  |
/// +-------------------------------------------------------------------------------+
#[derive(Debug)]
pub struct Instruction {
    pub rd: u8,
    pub nn: u16, // 0-1020, steps 4
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let rd = value.get_bits_u8(8, 10);
        let nn = value.get_bits(0, 7) << 2; // word aligned offset

        Self { rd, nn }
    }
}

impl Executable for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        let value = self.nn.into();
        let offset = RegisterOffset::new(value, AddrMode::IB, false);
        let pc = NamedRegister::PC as u8;

        cpu.ldr(self.rd, pc, offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ldr_pc_offset() {
        let asm = r"
            ldr r1, [PC, #4]
            dw  0x0000_0000
            dw  0x0000_0005
        ";

        GbaTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(1, 5)
            .run(1);
    }
}

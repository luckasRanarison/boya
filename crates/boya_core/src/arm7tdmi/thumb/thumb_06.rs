pub use crate::arm7tdmi::isa::prelude::*;

/// Load PC-relative
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  1 |  0 |  0 |  1 |      Rd      |              Offset8                  |
/// +-------------------------------------------------------------------------------+
pub struct Instruction {
    rd: u8,
    nn: u16, // 0-1020, steps 4
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR, {:?}, [PC, {:?}]", self.rd.reg(), self.nn.imm())
    }
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let rd = value.get_bits_u8(8, 10);
        let nn = value.get_bits(0, 7) << 2; // word aligned offset

        Self { rd, nn }
    }
}

impl<B: Bus> Executable<B> for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
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
        AsmTestBuilder::new()
            .thumb()
            .setup(|cpu| cpu.bus.write_word(TMB_MAIN_START + 20, 5))
            .asm("ldr r1, [PC, #16]")
            .assert_reg(1, 5)
            .run(1);
    }
}

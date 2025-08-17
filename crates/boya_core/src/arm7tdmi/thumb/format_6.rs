pub use super::prelude::*;

/// Load PC-relative
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  1 |  0 |  0 |  1 |      Rd      |              Offset8                  |
/// +-------------------------------------------------------------------------------+
pub struct Format6 {
    rd: u8,
    nn: u16,
}

impl Debug for Format6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDR, {:?}, [PC, {:?}]", self.rd.reg(), self.nn.imm())
    }
}

impl From<u16> for Format6 {
    fn from(value: u16) -> Self {
        let rd = value.get_bits_u8(8, 10);
        let nn = value.get_bits(0, 7) << 2; // word aligned offset (0-1020 in steps of 4)

        Self { rd, nn }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format6(&mut self, instr: Format6) {
        self.ldr(instr.rd, self.pc() + instr.nn as u32);
    }
}

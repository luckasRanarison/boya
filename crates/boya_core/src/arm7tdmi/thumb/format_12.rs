use super::prelude::*;

/// Get relative address
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  0 |  1 |  0 | Op |      Rd      |                Offset8                |
/// +-------------------------------------------------------------------------------+
pub struct Format12 {
    rs: Operand,
    nn: u16,
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

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format12(&mut self, instr: Format12) {
        self.add(instr.rs.value as u8, instr.nn.imm(), instr.rd, false);
    }
}

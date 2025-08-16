use super::prelude::*;

/// Load/store SP-relative
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  0 |  0 |  1 | Op |      Rd      |                Offset8                |
/// +-------------------------------------------------------------------------------+
pub struct Format11 {
    opcode: Opcode11,
    nn: u16,
    rd: u8,
}

impl Debug for Format11 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} R{}, [SP, #{}]", self.opcode, self.rd, self.nn)
    }
}

impl From<u16> for Format11 {
    fn from(value: u16) -> Self {
        let opcode = Opcode11::from(value.get(11));
        let rd = value.get_bits_u8(8, 10);
        let nn = value.get_bits(0, 7) << 2;

        Self { opcode, nn, rd }
    }
}

#[derive(Debug)]
pub enum Opcode11 {
    STR,
    LDR,
}

impl From<u16> for Opcode11 {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::STR,
            1 => Self::LDR,
            _ => unreachable!("invalid format 11 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format11(&mut self, op: Format11) {
        let addr = self.sp() + op.nn as u32;

        match op.opcode {
            Opcode11::STR => self.str(op.rd, addr),
            Opcode11::LDR => self.ldr(op.rd, addr),
        }
    }
}

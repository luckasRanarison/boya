use super::prelude::*;

/// Load/store halfword
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  0 |  0 |  0 | Op |           Offset5      |      Rb      |      Rd      |
/// +-------------------------------------------------------------------------------+
pub struct Format10 {
    opcode: Opcode10,
    nn: u16,
    rb: u8,
    rd: u8,
}

impl Debug for Format10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} R{}, [R{}, #{}]",
            self.opcode, self.rd, self.rb, self.nn
        )
    }
}

impl From<u16> for Format10 {
    fn from(value: u16) -> Self {
        let opcode = Opcode10::from(value.get(11));
        let nn = value.get_bits(6, 10) << 1;
        let rb = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        Self { opcode, nn, rb, rd }
    }
}

#[derive(Debug)]
pub enum Opcode10 {
    STRH,
    LDRH,
}

impl From<u16> for Opcode10 {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::STRH,
            1 => Self::LDRH,
            _ => unreachable!("invalid format 10 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format10(&mut self, op: Format10) {
        let addr = self.get_reg(op.rb) + op.nn as u32;

        match op.opcode {
            Opcode10::STRH => self.strh(op.rd, addr),
            Opcode10::LDRH => self.ldrh(op.rd, addr),
        }
    }
}

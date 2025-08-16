use super::prelude::*;

/// Load/store with immediate offset
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  1 |  1 |    Op   |           Offset5      |      Rb      |      Rd      |
/// +-------------------------------------------------------------------------------+
pub struct Format9 {
    opcode: Opcode9,
    nn: u16,
    rb: u8,
    rd: u8,
}

impl Debug for Format9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} R{}, [R{}, #{}]",
            self.opcode, self.rd, self.rb, self.nn
        )
    }
}

impl From<u16> for Format9 {
    fn from(value: u16) -> Self {
        let opcode = Opcode9::from(value.get_bits(11, 12));
        let nn = value.get_bits(6, 10) << 2;
        let rb = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        Self { opcode, nn, rb, rd }
    }
}

#[derive(Debug)]
pub enum Opcode9 {
    STR,
    LDR,
    STRB,
    LDRB,
}

impl From<u16> for Opcode9 {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::STR,
            1 => Self::LDR,
            2 => Self::STRB,
            3 => Self::LDRB,
            _ => unreachable!("invalid format 9 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format9(&mut self, op: Format9) {
        let addr = self.get_reg(op.rb) + op.nn as u32;

        match op.opcode {
            Opcode9::STR => self.str(op.rd, addr),
            Opcode9::LDR => self.ldr(op.rd, addr),
            Opcode9::STRB => self.strb(op.rd, addr),
            Opcode9::LDRB => self.ldrb(op.rd, addr),
        }
    }
}

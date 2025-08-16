use super::prelude::*;

/// Load/store sign-extended byte/halfword
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  1 |  0 |  1 |    Op   |  1 |      Ro      |      Rb      |      Rd      |
/// +-------------------------------------------------------------------------------+
pub struct Format8 {
    opcode: Opcode8,
    ro: u8,
    rb: u8,
    rd: u8,
}

impl Debug for Format8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} R{}, [R{}, R{}]",
            self.opcode, self.rd, self.rb, self.ro
        )
    }
}

impl From<u16> for Format8 {
    fn from(value: u16) -> Self {
        let opcode = Opcode8::from(value.get_bits(10, 11));
        let ro = value.get_bits_u8(6, 8);
        let rb = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        Self { opcode, ro, rb, rd }
    }
}

#[derive(Debug)]
pub enum Opcode8 {
    STRH,
    LDSB,
    LDSH,
}

impl From<u16> for Opcode8 {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::STRH,
            1 => Self::LDSB,
            2 | 3 => Self::LDSH,
            _ => unreachable!("invalid format 8 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format8(&mut self, op: Format8) {
        let addr = self.get_reg(op.rb) + self.get_reg(op.ro);

        match op.opcode {
            Opcode8::STRH => self.strh(op.rd, addr),
            Opcode8::LDSB => self.ldsb(op.rd, addr),
            Opcode8::LDSH => self.ldsh(op.rd, addr),
        }
    }
}

use super::prelude::*;

/// Load/Store with register offset
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  1 |  0 |  1 |    Op   |  0 |      Ro      |      Rb      |      Rd      |
/// +-------------------------------------------------------------------------------+
pub struct Format7 {
    opcode: Opcode7,
    ro: u8,
    rb: u8,
    rd: u8,
}

impl Debug for Format7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} R{}, [R{}, R{}]",
            self.opcode, self.rd, self.rb, self.ro
        )
    }
}

impl From<u16> for Format7 {
    fn from(value: u16) -> Self {
        let opcode = Opcode7::from(value);
        let ro = value.get_bits_u8(6, 8);
        let rb = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        Self { opcode, ro, rb, rd }
    }
}

#[derive(Debug)]
pub enum Opcode7 {
    STR,
    STRB,
    LDR,
    LDRB,
}

impl From<u16> for Opcode7 {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::STR,
            1 => Self::STRB,
            2 => Self::LDR,
            3 => Self::LDRB,
            _ => unreachable!("invalide format 7 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format7(&mut self, op: Format7) {
        let address = self.get_reg(op.rb) + self.get_reg(op.ro);

        match op.opcode {
            Opcode7::STR => self.str(op.rd, address, DataType::Word),
            Opcode7::STRB => self.str(op.rd, address, DataType::Byte),
            Opcode7::LDR => self.ldr(op.rd, address, DataType::Word),
            Opcode7::LDRB => self.ldr(op.rd, address, DataType::Byte),
        }
    }
}

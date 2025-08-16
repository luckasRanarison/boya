use super::prelude::*;

// Move/Compare/Add/Substract immediate
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  0 |  1 |    Op   |    Rd   |                   Offset8                  |
/// +-------------------------------------------------------------------------------+
pub struct Format3 {
    pub opcode: Opcode3,
    pub rd: u8,
    pub nn: u8,
}

impl Debug for Format3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} R{}, #{}", self.opcode, self.rd, self.nn)
    }
}

impl From<u16> for Format3 {
    fn from(value: u16) -> Self {
        let opcode = Opcode3::from(value.get_bits(11, 12));
        let rd = value.get_bits_u8(8, 10);
        let nn = value.get_bits_u8(0, 7);

        Self { opcode, rd, nn }
    }
}

#[derive(Debug)]
pub enum Opcode3 {
    MOV,
    CMP,
    ADD,
    SUB,
}

impl From<u16> for Opcode3 {
    fn from(value: u16) -> Self {
        match value {
            0b00 => Self::MOV,
            0b01 => Self::CMP,
            0b10 => Self::ADD,
            0b11 => Self::SUB,
            _ => unreachable!("invalid format 3 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format3(&mut self, op: Format3) {
        let nn = op.nn.immediate();

        match op.opcode {
            Opcode3::MOV => self.mov(op.rd, nn, true),
            Opcode3::CMP => self.cmp(op.rd, nn),
            Opcode3::ADD => self.add(op.rd, nn, op.rd, true),
            Opcode3::SUB => self.sub(op.rd, nn, op.rd),
        }
    }
}

pub use super::prelude::*;

/// Hi register operations/branch exchange
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  1 |  0 |  0 |  0 |  1 |    Op   | Hd | Hs |     Rs/Hs    |     Rd/Hd    |
/// +-------------------------------------------------------------------------------+
pub struct Format5 {
    pub opcode: Opcode5,
    pub rs: u8,
    pub rd: u8,
}

impl Debug for Format5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.opcode {
            Opcode5::BX => write!(f, "BX R{}", self.rs),
            opcode => write!(f, "{opcode:?} R{}, R{}", self.rd, self.rs),
        }
    }
}

impl From<u16> for Format5 {
    fn from(value: u16) -> Self {
        let opcode = Opcode5::from(value.get_bits(8, 9));
        let msbd = value.get_u8(7);
        let msbs = value.get_u8(6);
        let rs = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        Self {
            opcode,
            rs: rs | (msbs << 3),
            rd: rd | (msbd << 3),
        }
    }
}

#[derive(Debug)]
pub enum Opcode5 {
    ADD,
    CMP,
    MOV,
    BX,
}

impl From<u16> for Opcode5 {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::ADD,
            1 => Self::CMP,
            2 => Self::MOV,
            3 => Self::BX,
            _ => unreachable!("invalid format 5 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format5(&mut self, op: Format5) {
        match op.opcode {
            Opcode5::ADD => self.add(op.rd, op.rs.register(), op.rd, false),
            Opcode5::CMP => self.cmp(op.rd, op.rs.register()),
            Opcode5::MOV => self.mov(op.rd, op.rs.register(), false),
            Opcode5::BX => self.bx(op.rs),
        }
    }
}

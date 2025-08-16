use super::prelude::*;

/// Get relative address
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  0 |  1 |  0 | Op |      Rd      |                Offset8                |
/// +-------------------------------------------------------------------------------+
pub struct Format12 {
    rs: Register12,
    nn: u16,
    rd: u8,
}

impl Debug for Format12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ADD R{}, {:?}, #{}", self.rd, self.rs, self.nn)
    }
}

impl From<u16> for Format12 {
    fn from(value: u16) -> Self {
        let rs = Register12::from(value.get(11));
        let rd = value.get_bits_u8(8, 10);
        let nn = value.get_bits(0, 7) << 2;

        Self { rs, nn, rd }
    }
}

#[derive(Debug)]
pub enum Register12 {
    PC,
    SP,
}

impl From<u16> for Register12 {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::PC,
            1 => Self::SP,
            _ => unreachable!("invalid register for format 12: {value}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format12(&mut self, op: Format12) {
        match op.rs {
            Register12::PC => self.add(Self::PC as u8, op.nn.immediate(), op.rd, false),
            Register12::SP => self.add(Self::SP as u8, op.nn.immediate(), op.rd, false),
        }
    }
}

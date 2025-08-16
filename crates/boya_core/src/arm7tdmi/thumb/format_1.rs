use super::prelude::*;

/// Move shifted register
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  0 |  0 |    Op   |         Offset5        |      Rs      |      Rd      |
/// +-------------------------------------------------------------------------------+
pub struct Format1 {
    pub opcode: Opcode1,
    pub offset: u8,
    pub rs: u8,
    pub rd: u8,
}

impl Debug for Format1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} R{}, R{}, #{}",
            self.opcode, self.rd, self.rs, self.offset
        )
    }
}

impl From<u16> for Format1 {
    fn from(value: u16) -> Self {
        let opcode = value.get_bits(11, 12).into();
        let offset = value.get_bits_u8(6, 10);
        let rs = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        Self {
            opcode,
            offset,
            rs,
            rd,
        }
    }
}

#[derive(Debug)]
pub enum Opcode1 {
    LSL,
    LSR,
    ASR,
}

impl From<u16> for Opcode1 {
    fn from(value: u16) -> Self {
        match value {
            0b00 => Self::LSL,
            0b01 => Self::LSR,
            0b10 => Self::ASR,
            value => unreachable!("invalid format 1 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format1(&mut self, op: Format1) {
        let nn = op.offset.immediate();

        match op.opcode {
            Opcode1::LSL => self.lsl(op.rs, nn, op.rd),
            Opcode1::LSR => self.lsr(op.rs, nn, op.rd),
            Opcode1::ASR => self.asr(op.rs, nn, op.rd),
        }
    }
}

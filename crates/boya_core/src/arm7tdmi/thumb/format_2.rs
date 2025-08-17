use super::prelude::*;

/// Add/Substract
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  0 |  0 |  1 |  1 |  I | Op |  Rn/Offset3  |      Rs      |      Rd      |
/// +-------------------------------------------------------------------------------+
pub struct Format2 {
    pub op: Opcode2,
    pub nn: Operand,
    pub rs: u8,
    pub rd: u8,
}

impl Debug for Format2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?}, {:?}, {:?}",
            self.op,
            self.rd.reg(),
            self.rs.reg(),
            self.nn,
        )
    }
}

impl From<u16> for Format2 {
    fn from(value: u16) -> Self {
        let op = Opcode2::from(value.get_bits(9, 10));
        let operand = value.get_bits(6, 8);
        let rs = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        let nn = match value.has(10) {
            true => operand.imm(),
            false => operand.reg(),
        };

        Self { op, nn, rs, rd }
    }
}

#[derive(Debug)]
pub enum Opcode2 {
    ADD,
    SUB,
}

impl From<u16> for Opcode2 {
    fn from(value: u16) -> Self {
        match value {
            0b00 | 0b10 => Self::ADD,
            0b01 | 0b11 => Self::SUB,
            value => unreachable!("invalid format 2 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format2(&mut self, instr: Format2) {
        match instr.op {
            Opcode2::ADD => self.add(instr.rs, instr.nn, instr.rd, true),
            Opcode2::SUB => self.sub(instr.rs, instr.nn, instr.rd),
        }
    }
}

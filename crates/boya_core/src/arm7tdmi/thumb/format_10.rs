use super::prelude::*;

/// Load/store halfword
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  0 |  0 |  0 | Op |           Offset5      |      Rb      |      Rd      |
/// +-------------------------------------------------------------------------------+
pub struct Format10 {
    op: Opcode,
    nn: u16,
    rb: u8,
    rd: u8,
}

impl Debug for Format10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?}, [{:?}, {:?}]",
            self.op,
            self.rd.reg(),
            self.rb.reg(),
            self.nn.imm()
        )
    }
}

impl From<u16> for Format10 {
    fn from(value: u16) -> Self {
        let op = Opcode::from(value.get(11));
        let nn = value.get_bits(6, 10) << 1;
        let rb = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        Self { op, nn, rb, rd }
    }
}

#[derive(Debug)]
enum Opcode {
    STRH,
    LDRH,
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::STRH,
            1 => Self::LDRH,
            _ => unreachable!("invalid format 10 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format10(&mut self, instr: Format10) {
        let addr = self.get_reg(instr.rb) + instr.nn as u32;

        match instr.op {
            Opcode::STRH => self.strh(instr.rd, addr),
            Opcode::LDRH => self.ldrh(instr.rd, addr),
        }
    }
}

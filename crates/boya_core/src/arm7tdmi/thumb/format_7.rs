use super::prelude::*;

/// Load/Store with register offset
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  1 |  0 |  1 |    Op   |  0 |      Ro      |      Rb      |      Rd      |
/// +-------------------------------------------------------------------------------+
pub struct Format7 {
    op: Opcode7,
    ro: u8,
    rb: u8,
    rd: u8,
}

impl Debug for Format7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?}, [{:?}, {:?}]",
            self.op,
            self.rd.reg(),
            self.rb.reg(),
            self.ro.reg()
        )
    }
}

impl From<u16> for Format7 {
    fn from(value: u16) -> Self {
        let op = Opcode7::from(value.get_bits(10, 11));
        let ro = value.get_bits_u8(6, 8);
        let rb = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        Self { op, ro, rb, rd }
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
            _ => unreachable!("invalid format 7 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format7(&mut self, instr: Format7) {
        let addr = self.get_reg(instr.rb) + self.get_reg(instr.ro);

        match instr.op {
            Opcode7::STR => self.str(instr.rd, addr),
            Opcode7::STRB => self.strb(instr.rd, addr),
            Opcode7::LDR => self.ldr(instr.rd, addr),
            Opcode7::LDRB => self.ldrb(instr.rd, addr),
        }
    }
}

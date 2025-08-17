use super::prelude::*;

/// ALU operations
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  1 |  0 |  0 |  0 |  0 |         Op        |      Rs      |      Rd      |
/// +-------------------------------------------------------------------------------+
pub struct Format4 {
    pub op: Opcode4,
    pub rs: u8,
    pub rd: u8,
}

impl Debug for Format4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}, {:?}", self.op, self.rd.reg(), self.rs.reg())
    }
}

impl From<u16> for Format4 {
    fn from(value: u16) -> Self {
        let op = Opcode4::from(value.get_bits(6, 9));
        let rs = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        Self { op, rs, rd }
    }
}

#[derive(Debug)]
pub enum Opcode4 {
    AND,
    EOR,
    LSL,
    LSR,
    ASR,
    ADC,
    SBC,
    ROR,
    TST,
    NEG,
    CMP,
    CMN,
    ORR,
    MUL,
    BIC,
    MVN,
}

impl From<u16> for Opcode4 {
    fn from(value: u16) -> Self {
        match value {
            0x0 => Self::AND,
            0x1 => Self::EOR,
            0x2 => Self::LSL,
            0x3 => Self::LSR,
            0x4 => Self::ASR,
            0x5 => Self::ADC,
            0x6 => Self::SBC,
            0x7 => Self::ROR,
            0x8 => Self::TST,
            0x9 => Self::NEG,
            0xA => Self::CMP,
            0xB => Self::CMN,
            0xC => Self::ORR,
            0xD => Self::MUL,
            0xE => Self::BIC,
            0xF => Self::MVN,
            _ => unreachable!("invalid format 4 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format4(&mut self, instr: Format4) {
        match instr.op {
            Opcode4::AND => self.and(instr.rd, instr.rs),
            Opcode4::EOR => self.eor(instr.rd, instr.rs),
            Opcode4::LSL => self.lsl(instr.rd, instr.rs.reg(), instr.rd),
            Opcode4::LSR => self.lsr(instr.rd, instr.rs.reg(), instr.rd),
            Opcode4::ASR => self.asr(instr.rd, instr.rs.reg(), instr.rd),
            Opcode4::ADC => self.adc(instr.rd, instr.rs.reg(), instr.rd),
            Opcode4::SBC => self.sbc(instr.rd, instr.rs.reg(), instr.rd),
            Opcode4::ROR => self.ror(instr.rd, instr.rs.reg(), instr.rd),
            Opcode4::TST => self.tst(instr.rd, instr.rs),
            Opcode4::NEG => self.neg(instr.rd, instr.rs),
            Opcode4::CMP => self.cmp(instr.rd, instr.rs.reg()),
            Opcode4::CMN => self.cmn(instr.rd, instr.rs.reg()),
            Opcode4::ORR => self.orr(instr.rd, instr.rs),
            Opcode4::MUL => self.mul(instr.rd, instr.rs.reg(), instr.rd),
            Opcode4::BIC => self.bic(instr.rd, instr.rs),
            Opcode4::MVN => self.mvn(instr.rd, instr.rs),
        }
    }
}

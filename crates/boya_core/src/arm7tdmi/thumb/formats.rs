use std::fmt::Debug;

use crate::{
    arm7tdmi::common::{Operand, ToOperand},
    utils::bitflags::Bitflag,
};

pub enum InstructionFormat {
    /// Move shifted register
    Format1(Format1),
    /// Add/Substract
    Format2(Format2),
    /// Move/Compare/Add/Substract immediate
    Format3(Format3),
    /// ALU operations
    Format4(Format4),
}

impl Debug for InstructionFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionFormat::Format1(op) => write!(f, "{op:?} ; format 1"),
            InstructionFormat::Format2(op) => write!(f, "{op:?} ; format 2"),
            InstructionFormat::Format3(op) => write!(f, "{op:?} ; format 3"),
            InstructionFormat::Format4(op) => write!(f, "{op:?} ; format 4"),
        }
    }
}

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
        let offset = value.get_bits(6, 10) as u8;
        let rs = value.get_bits(3, 5) as u8;
        let rd = value.get_bits(0, 2) as u8;

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

pub struct Format2 {
    pub opcode: Opcode2,
    pub nn: Operand,
    pub rs: u8,
    pub rd: u8,
}

impl Debug for Format2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} R{}, R{}, {:?}",
            self.opcode, self.rd, self.rs, self.nn,
        )
    }
}

impl From<u16> for Format2 {
    fn from(value: u16) -> Self {
        let opcode = Opcode2::from(value.get_bits(9, 10));
        let operand = value.get_bits(6, 8);
        let rs = value.get_bits(3, 5) as u8;
        let rd = value.get_bits(0, 2) as u8;

        let nn = match value.contains(9) {
            true => operand.immediate(),
            false => operand.register(),
        };

        Self { opcode, nn, rs, rd }
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
        let rd = value.get_bits(8, 10) as u8;
        let nn = value.get_bits(0, 7) as u8;

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

pub struct Format4 {
    pub opcode: Opcode4,
    pub rs: u8,
    pub rd: u8,
}

impl Debug for Format4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} R{}, R{}", self.opcode, self.rs, self.rd)
    }
}

impl From<u16> for Format4 {
    fn from(value: u16) -> Self {
        let opcode = Opcode4::from(value.get_bits(6, 9));
        let rs = value.get_bits(3, 5) as u8;
        let rd = value.get_bits(0, 2) as u8;

        Self { opcode, rs, rd }
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

use std::fmt::Debug;

use crate::utils::bitflags::BitIter;

#[derive(Debug)]
pub enum DataType {
    Byte,
    HWord,
    Word,
}

#[derive(Debug, Clone, Copy)]
pub enum NamedRegister {
    LR = 14,
    SP = 13,
    PC = 15,
}

#[derive(Debug, Clone, Copy)]
pub enum MemoryAccess {
    Seq,
    NonSeq,
}

#[derive(Debug, Clone, Copy)]
pub enum RegisterFx {
    IncB,
    IncA,
    DecB,
    DecA,
}

#[derive(Debug)]
pub enum Carry {
    One,
    None,
    Flag,
}

#[derive(Debug)]
pub struct Cycle {
    pub i: u8,
    pub s: u8,
    pub n: u8,
}

impl Cycle {
    pub fn count(self) -> u8 {
        self.i + self.s + self.n
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OperatingMode {
    USR = 0b10000,
    FIQ = 0b10001,
    IRQ = 0b10010,
    SVC = 0b10011,
    ABT = 0b10111,
    UND = 0b11011,
    SYS = 0b11111,
}

#[derive(Debug, Clone, Copy)]
pub enum Exception {
    Reset,
    UndefinedInstruction,
    SoftwareInterrupt,
    PrefetchAbort,
    DataAbort,
    NormalInterrupt,
    FastInterrupt,
}

pub struct Operand {
    pub kind: OperandKind,
    pub value: u32,
    pub negate: bool,
}

impl Debug for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            OperandKind::SP => write!(f, "SP"),
            OperandKind::PC => write!(f, "PC"),
            OperandKind::Imm if self.negate => write!(f, "#-{}", self.value),
            OperandKind::Imm => write!(f, "#{}", self.value),
            OperandKind::Reg => write!(f, "R{}", self.value),
        }
    }
}

impl Operand {
    pub fn not(mut self) -> Self {
        self.negate = true;
        self
    }

    pub fn pc() -> Self {
        Operand {
            kind: OperandKind::PC,
            value: 15,
            negate: false,
        }
    }

    pub fn sp() -> Self {
        Operand {
            kind: OperandKind::SP,
            value: 13,
            negate: false,
        }
    }

    pub fn is_pc(&self) -> bool {
        match self.kind {
            OperandKind::PC | OperandKind::Reg if self.value == 15 => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperandKind {
    SP,
    PC,
    Imm,
    Reg,
}

pub trait ToOperand {
    fn reg(self) -> Operand;
    fn imm(self) -> Operand;
}

impl<T> ToOperand for T
where
    T: Into<u32>,
{
    fn reg(self) -> Operand {
        Operand {
            kind: OperandKind::Reg,
            value: self.into(),
            negate: false,
        }
    }

    fn imm(self) -> Operand {
        Operand {
            kind: OperandKind::Imm,
            value: self.into(),
            negate: false,
        }
    }
}

pub fn format_rlist<I: BitIter>(registers: I, named: Option<NamedRegister>) -> String {
    let inner = registers
        .iter_lsb()
        .filter(|(_, bit)| *bit == 1)
        .map(|(i, _)| format!("R{i}"))
        .chain([named.map(|s| format!("{s:?}"))].into_iter().flatten())
        .collect::<Vec<_>>()
        .join(",");

    format!("{{{inner}}}")
}

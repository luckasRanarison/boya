use std::fmt::Debug;

#[derive(Debug)]
pub enum DataType {
    Byte,
    HalfWord,
    Word,
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
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
pub enum Carry {
    One,
    None,
    Flag,
}

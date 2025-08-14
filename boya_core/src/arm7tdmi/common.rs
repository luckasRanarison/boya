use std::fmt::Debug;

pub struct Operand {
    pub kind: OperandKind,
    pub value: u32,
    pub negate: bool,
    pub mask: bool,
}

impl Debug for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.kind {
            OperandKind::Immediate => "#",
            OperandKind::Register => "R",
        };

        write!(f, "{prefix}{}", self.value)
    }
}

impl Operand {
    pub fn not(mut self) -> Self {
        self.negate = true;
        self
    }

    pub fn mask(mut self) -> Self {
        self.mask = true;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OperandKind {
    Immediate,
    Register,
}

#[derive(Debug, Clone, Copy)]
pub enum AddSubOp {
    Add,
    Sub,
}

pub trait ToOperand {
    fn register(self) -> Operand;
    fn immediate(self) -> Operand;
}

impl<T> ToOperand for T
where
    T: Into<u32>,
{
    fn register(self) -> Operand {
        Operand {
            kind: OperandKind::Register,
            value: self.into(),
            negate: false,
            mask: false,
        }
    }

    fn immediate(self) -> Operand {
        Operand {
            kind: OperandKind::Immediate,
            value: self.into(),
            negate: false,
            mask: false,
        }
    }
}

use std::{
    fmt::Debug,
    ops::{Add, AddAssign},
};

use crate::{bus::types::WaitState, utils::bitflags::BitIter};

#[derive(Debug, Clone, Copy)]
pub enum NamedRegister {
    LR = 14,
    SP = 13,
    PC = 15,
}

#[derive(Debug, Clone, Copy)]
pub enum AddrMode {
    IB,
    IA,
    DB,
    DA,
}

impl AddrMode {
    pub fn new(p: u8, u: u8) -> Self {
        match (p, u) {
            (0, 0) => Self::DA,
            (0, 1) => Self::IA,
            (1, 0) => Self::DB,
            (_, _) => Self::IB,
        }
    }
}

#[derive(Debug)]
pub struct RegisterOffset {
    pub amod: AddrMode,
    pub wb: bool,
    pub value: u32,
}

impl RegisterOffset {
    pub fn new(value: u32, amod: AddrMode, wb: bool) -> Self {
        Self { amod, wb, value }
    }
}

#[derive(Debug)]
pub enum Carry {
    One,
    None,
    Flag,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Cycle(u8);

impl Cycle {
    #[inline(always)]
    pub fn new(i: u8, s: u8, n: u8, ws: WaitState) -> Self {
        Self(i + ws.s * s + s + ws.n * n + n)
    }

    #[inline(always)]
    pub fn internal(n: u8) -> Self {
        Self(n)
    }

    #[inline(always)]
    pub fn repeat(self, n: u8) -> Self {
        Self(self.0 * n)
    }

    #[inline(always)]
    pub fn count(self) -> u8 {
        self.0
    }
}

impl Add<Self> for Cycle {
    type Output = Self;

    fn add(self, rhs: Cycle) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<Self> for Cycle {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    Undefined,
    SoftwareInterrupt,
    PrefetchAbort,
    DataAbort,
    NormalInterrupt,
    FastInterrupt,
}

impl Exception {
    pub fn vector(self) -> u32 {
        match self {
            Self::Reset => 0x00,
            Self::Undefined => 0x04,
            Self::SoftwareInterrupt => 0x08,
            Self::PrefetchAbort => 0x0C,
            Self::DataAbort => 0x10,
            Self::NormalInterrupt => 0x18,
            Self::FastInterrupt => 0x1C,
        }
    }

    pub fn operating_mode(self) -> OperatingMode {
        match self {
            Self::Reset => OperatingMode::SVC,
            Self::Undefined => OperatingMode::UND,
            Self::SoftwareInterrupt => OperatingMode::SVC,
            Self::PrefetchAbort => OperatingMode::ABT,
            Self::DataAbort => OperatingMode::ABT,
            Self::NormalInterrupt => OperatingMode::IRQ,
            Self::FastInterrupt => OperatingMode::IRQ,
        }
    }

    pub fn disable_irq(self) -> bool {
        matches!(
            self,
            Self::FastInterrupt | Self::NormalInterrupt | Self::Reset
        )
    }

    pub fn disable_fiq(self) -> bool {
        matches!(self, Self::Reset)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Condition {
    EQ,
    NE,
    CS,
    CC,
    MI,
    PL,
    VS,
    VC,
    HI,
    LS,
    GE,
    LT,
    GT,
    LE,
    AL,
}

impl From<u8> for Condition {
    fn from(value: u8) -> Self {
        match value {
            0x0 => Self::EQ,
            0x1 => Self::NE,
            0x2 => Self::CS,
            0x3 => Self::CC,
            0x4 => Self::MI,
            0x5 => Self::PL,
            0x6 => Self::VS,
            0x7 => Self::VC,
            0x8 => Self::HI,
            0x9 => Self::LS,
            0xA => Self::GE,
            0xB => Self::LT,
            0xC => Self::GT,
            0xD => Self::LE,
            0xE => Self::AL,
            _ => unreachable!("invalid condition: {value:04b}"),
        }
    }
}

pub struct Operand {
    pub kind: OperandKind,
    pub value: u32,
    pub negate: bool,
    pub shift: Option<Shift>,
}

impl Debug for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lhs = match self.kind {
            OperandKind::Imm if self.negate => format!("#-{}", self.value),
            OperandKind::Imm => format!("#{}", self.value),
            OperandKind::Reg => format!("R{}", self.value),
        };

        if let Some(shift) = &self.shift {
            write!(
                f,
                "{lhs}, {:?} {}{}",
                shift.kind,
                if shift.register { "R" } else { "#" },
                shift.value
            )
        } else {
            write!(f, "{lhs}")
        }
    }
}

impl Operand {
    pub fn not(mut self) -> Self {
        self.negate = true;
        self
    }

    pub fn shift(mut self, shift: Shift) -> Self {
        self.shift = Some(shift);
        self
    }

    pub fn is_pc(&self) -> bool {
        matches!(self.kind,  OperandKind::Reg if self.value == 15)
    }

    pub fn is_imm(&self) -> bool {
        matches!(self.kind, OperandKind::Imm)
    }

    pub fn is_reg(&self) -> bool {
        matches!(self.kind, OperandKind::Reg)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperandKind {
    Imm,
    Reg,
}

#[derive(Debug, Clone, Copy)]
pub enum ShiftKind {
    LSL,
    LSR,
    ASR,
    ROR,
}

impl From<u8> for ShiftKind {
    fn from(value: u8) -> Self {
        match value {
            0x0 => Self::LSL,
            0x1 => Self::LSR,
            0x2 => Self::ASR,
            0x3 => Self::ROR,
            _ => unreachable!("invalid shift type: {value:#b}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Shift {
    pub value: u8,
    pub register: bool,
    pub kind: ShiftKind,
}

impl Shift {
    pub fn imm(value: u8, kind: ShiftKind) -> Self {
        Self {
            value,
            register: false,
            kind,
        }
    }

    pub fn reg(value: u8, kind: ShiftKind) -> Self {
        Self {
            value,
            register: true,
            kind,
        }
    }
}

#[derive(Debug)]
pub enum CarryUpdate {
    Unchanged,
    Clear,
    Bit(u32),
}

#[derive(Debug, Clone, Copy)]
pub struct LongOperand {
    pub lo: u8,
    pub hi: Option<u8>,
}

impl From<u8> for LongOperand {
    fn from(lo: u8) -> Self {
        Self { lo, hi: None }
    }
}

impl LongOperand {
    pub fn new(lo: u8, hi: u8) -> Self {
        Self { lo, hi: Some(hi) }
    }
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
            shift: None,
        }
    }

    fn imm(self) -> Operand {
        Operand {
            kind: OperandKind::Imm,
            value: self.into(),
            negate: false,
            shift: None,
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

pub fn format_addr_mode(amod: AddrMode, base: u8, offset: &Operand, wb: bool) -> String {
    let rn = base.reg();

    let addr = match amod {
        AddrMode::IB | AddrMode::DB if offset.is_imm() && offset.value == 0 => {
            format!("[{rn:?}]")
        }
        AddrMode::IB => format!("[{rn:?}, {:?}]", offset),
        AddrMode::DB if offset.is_imm() => format!("[{rn:?}, #-{:?}]", offset.value),
        AddrMode::DB => format!("[{rn:?}, -{:?}]", offset),
        AddrMode::IA => format!("[{rn:?}], {:?}", offset),
        AddrMode::DA if offset.is_imm() => format!("[{rn:?}], #-{:?}", offset.value),
        AddrMode::DA => format!("[{rn:?}], -{:?}", offset),
    };

    match amod {
        AddrMode::IB | AddrMode::DB if wb => format!("{addr}!"),
        _ => addr,
    }
}

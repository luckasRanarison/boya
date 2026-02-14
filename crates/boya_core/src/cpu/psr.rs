use crate::{
    cpu::common::{Condition, OperatingMode},
    utils::bitflags::Bitflag,
};

#[derive(Debug, Clone, Copy)]
pub enum PsrKind {
    CPSR,
    SPSR,
}

/// +----------------------------------------------------------------------------+
/// | N(31) | Z(30) | C(29) |   V(28)  |  U(27-8) | I(7) | F(6) | T(5)  | M(4-0) |
/// |-------|-------|-------|----------|----------|------|------|-------|--------|
/// | sign  | zero  | carry | overflow | reserved | irq  | fiq  | thumb |  mode  |
/// +----------------------------------------------------------------------------+
#[derive(Default, Debug, Clone, Copy)]
pub struct Psr(u32);

impl From<u32> for Psr {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Psr {
    /// N - Sign flag (0: Not Signed, 1:Signed)
    pub const N: u32 = 31;
    /// Z - Zero flag (0: Not Zero, 1: Zero)
    pub const Z: u32 = 30;
    /// C - Carry flag (0: No Carry, 1: Carry)
    pub const C: u32 = 29;
    /// V - Overflow flag (0: No Overflow, 1: Overflow)
    pub const V: u32 = 28;
    /// I - IRQ disable (0: Enable, 1: Disable)
    pub const I: u32 = 7;
    /// F - FIQ disable (0: Enable, 1: Disable)
    pub const F: u32 = 6;
    /// T - State bit (0: ARM, 1: THUMB)
    pub const T: u32 = 5;
}

impl Psr {
    #[inline(always)]
    pub fn value(self) -> u32 {
        self.0
    }

    #[inline(always)]
    pub fn get(self, bit: u32) -> u32 {
        self.0.get(bit)
    }

    #[inline(always)]
    pub fn has(self, bit: u32) -> bool {
        self.0.has(bit)
    }

    pub fn matches(self, cond: Condition) -> bool {
        match cond {
            Condition::EQ => self.has(Self::Z),
            Condition::NE => !self.has(Self::Z),
            Condition::CS => self.has(Self::C),
            Condition::CC => !self.has(Self::C),
            Condition::MI => self.has(Self::N),
            Condition::PL => !self.has(Self::N),
            Condition::VS => self.has(Self::V),
            Condition::VC => !self.has(Self::V),
            Condition::HI => self.has(Self::C) && !self.has(Self::Z),
            Condition::LS => !self.has(Self::C) || self.has(Self::Z),
            Condition::GE => self.has(Self::N) == self.has(Self::V),
            Condition::LT => self.has(Self::N) != self.has(Self::V),
            Condition::GT => !self.has(Self::Z) && self.has(Self::N) == self.has(Self::V),
            Condition::LE => self.has(Self::Z) || self.has(Self::N) != self.has(Self::V),
            Condition::AL => true,
        }
    }

    #[inline(always)]
    pub fn thumb(self) -> bool {
        self.has(Self::T)
    }

    #[inline(always)]
    pub fn update(&mut self, bit: u32, value: bool) {
        self.0.update(bit, value);
    }

    #[inline(always)]
    pub fn update_zn(&mut self, value: u32) {
        self.0.update(Self::Z, value == 0);
        self.0.update(Self::N, value.has(31));
    }

    pub fn set_operating_mode(&mut self, mode: OperatingMode) {
        self.0.set_bits(0, 4, mode as u32);
    }

    /// # Panics
    ///
    /// Panics if an invalid operating mode is used.
    pub fn operating_mode(self) -> OperatingMode {
        match self.0.get_bits(0, 4) | 0b10000 {
            0b10000 => OperatingMode::USR,
            0b10001 => OperatingMode::FIQ,
            0b10010 => OperatingMode::IRQ,
            0b10011 => OperatingMode::SVC,
            0b10111 => OperatingMode::ABT,
            0b11011 => OperatingMode::UND,
            0b11111 => OperatingMode::SYS,
            value => panic!("invalid operating mode: {value:05b}"),
        }
    }

    #[cfg(test)]
    pub fn format_flag(bit: u32) -> &'static str {
        match bit {
            Self::N => "N",
            Self::Z => "Z",
            Self::C => "C",
            Self::V => "V",
            Self::I => "I",
            Self::F => "F",
            Self::T => "T",
            _ => unreachable!("invalid status bit: {bit}"),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct PsrField {
    pub mask: u32,
}

impl From<u8> for PsrField {
    fn from(value: u8) -> Self {
        let f = if value.has(3) { 0xFF000000 } else { 0 };
        let s = if value.has(2) { 0x00FF0000 } else { 0 };
        let x = if value.has(1) { 0x0000FF00 } else { 0 };
        let c = if value.has(0) { 0x000000FF } else { 0 };

        Self {
            mask: f | s | x | c,
        }
    }
}

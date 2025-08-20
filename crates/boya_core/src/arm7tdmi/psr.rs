use std::fmt::Debug;

use crate::{arm7tdmi::common::OperatingMode, utils::bitflags::Bitflag};

/// +----------------------------------------------------------------------------+
/// | N(31) | Z(30) | C(29) |   V(28)  |  U(27-8) | I(7) | F(6) | T(5)  | M(4-0) |
/// |-------|-------|-------|----------|----------|------|------|-------|--------|
/// | sign  | zero  | carry | overflow | reserved | irq  | fiq  | thumb |  mode  |
/// +----------------------------------------------------------------------------+
#[derive(Default, Clone, Copy)]
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

impl Debug for Psr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "N: {}, Z: {}, C: {}, V: {}, I: {}, F: {}, T: {}, M: {:?}",
            self.0.get(Self::N),
            self.0.get(Self::Z),
            self.0.get(Self::C),
            self.0.get(Self::V),
            self.0.get(Self::I),
            self.0.get(Self::F),
            self.0.get(Self::T),
            self.operating_mode()
        )
    }
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

    #[inline(always)]
    pub fn z(self) -> bool {
        self.has(Self::Z)
    }

    #[inline(always)]
    pub fn c(self) -> bool {
        self.has(Self::C)
    }

    #[inline(always)]
    pub fn s(self) -> bool {
        self.has(Self::N)
    }

    #[inline(always)]
    pub fn v(self) -> bool {
        self.has(Self::V)
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

    #[inline(always)]
    pub fn set_operating_mode(&mut self, mode: OperatingMode) {
        self.0.set_bits(0, 4, mode as u32);
    }

    #[inline(always)]
    pub fn set_arm_mode(&mut self) {
        self.0.clear(Self::T);
    }

    #[inline(always)]
    pub fn set_thumb_mode(&mut self) {
        self.0.set(Self::T);
    }

    pub fn operating_mode(self) -> OperatingMode {
        match self.0.get_bits(0, 4) {
            0b10000 => OperatingMode::USR,
            0b10001 => OperatingMode::FIQ,
            0b10010 => OperatingMode::IRQ,
            0b10011 => OperatingMode::SVC,
            0b10111 => OperatingMode::ABT,
            0b11011 => OperatingMode::UND,
            0b11111 => OperatingMode::SYS,
            value => unreachable!("invalid operating mode: {value:b}"),
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

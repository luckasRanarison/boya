use std::fmt::Debug;

use crate::utils::bitflags::Bitflag;

/// | Bit  | Name              | Value                         |
/// | :--- | :---------------- | :---------------------------- |
/// | 31   | N - Sign flag     | (0: Not Signed, 1:Signed)     |
/// | 30   | Z - Zero flag     | (0: Not Zero, 1: Zero)        |
/// | 29   | C - Carry flag    | (0: No Carry, 1: Carry)       |
/// | 28   | V - Overflow flag | (0: No Overflow, 1: Overflow) |
/// | 27-8 | Reserved          |                               |
/// | 7    | I - IRQ disable   | (0: Enable, 1: Disable)       |
/// | 6    | F - FIQ disable   | (0: Enable, 1: Disable)       |
/// | 5    | T - State bit     | (0: ARM, 1: THUMB)            |
/// | 4-0  | M4-M0 Mode bits   | Operating mode (see below)    |
#[derive(Default, Clone, Copy)]
pub struct Psr(u32);

impl Debug for Psr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "N: {}, Z: {}, C: {}, V: {}, I: {}, F: {}, T: {}, M: {:?}",
            self.0.get(flags::N),
            self.0.get(flags::Z),
            self.0.get(flags::C),
            self.0.get(flags::V),
            self.0.get(flags::I),
            self.0.get(flags::F),
            self.0.get(flags::T),
            self.operating_mode()
        )
    }
}

impl Psr {
    pub fn new() -> Self {
        let mut flags = 0b0_u32;

        flags.set(flags::I);
        flags.set(flags::F);
        flags.set_bits(0, 4, OperatingMode::Svc as u32);

        Self(flags)
    }

    pub fn value(self) -> u32 {
        self.0
    }

    pub fn contains(&self, flag: u32) -> bool {
        self.0.contains(flag)
    }

    pub fn update_zero(&mut self, value: u32) {
        self.0.update(flags::Z, value == 0);
    }

    pub fn update_sign(&mut self, value: u32) {
        self.0.update(flags::N, value.contains(31));
    }

    pub fn update_carry(&mut self, cond: bool) {
        self.0.update(flags::C, cond);
    }

    pub fn update_overflow(&mut self, cond: bool) {
        self.0.update(flags::V, cond);
    }

    pub fn update_thumb(&mut self, cond: bool) {
        self.0.update(flags::T, cond);
    }

    pub fn carry_bit(self) -> u32 {
        self.0.get(flags::C)
    }

    pub fn thumb_state(&self) -> bool {
        self.0.contains(flags::T)
    }

    pub fn operating_mode(self) -> OperatingMode {
        match self.0.get_bits(0, 4) {
            0b10000 => OperatingMode::Usr,
            0b10001 => OperatingMode::Fiq,
            0b10010 => OperatingMode::Irq,
            0b10011 => OperatingMode::Svc,
            0b10111 => OperatingMode::Abt,
            0b11011 => OperatingMode::Und,
            0b11111 => OperatingMode::Sys,
            value => unreachable!("invalid operating mode: {value:b}"),
        }
    }
}

pub mod flags {
    pub const N: u32 = 31;
    pub const Z: u32 = 30;
    pub const C: u32 = 29;
    pub const V: u32 = 28;
    pub const I: u32 = 7;
    pub const F: u32 = 6;
    pub const T: u32 = 5;
}

#[derive(Debug)]
pub enum OperatingMode {
    Usr = 0b10000,
    Fiq = 0b10001,
    Irq = 0b10010,
    Svc = 0b10011,
    Abt = 0b10111,
    Und = 0b11011,
    Sys = 0b11111,
}

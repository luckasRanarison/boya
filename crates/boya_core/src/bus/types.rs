use std::ops::{Add, AddAssign};

use crate::{bus::registers::IORegister, ppu::registers::PpuRegister};

#[derive(Debug, Clone, Copy)]
pub enum DataType {
    Byte = 1,
    HWord = 2,
    Word = 4,
}

#[derive(Debug)]
pub enum MemoryAccess {
    Seq,
    NonSeq,
}

#[derive(Debug)]
pub struct MemoryRegion {
    pub width: DataType,
    pub waitstate: WaitState,
}

#[derive(Debug, Default)]
pub struct WaitState {
    pub n: u8,
    pub s: u8,
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

#[derive(Debug)]
pub enum ReadableMemory<'a> {
    Slice(&'a [u8]),
    IoRegister(&'a IORegister),
    PpuRegister(&'a PpuRegister),
    OpenBus,
}

#[derive(Debug)]
pub enum WritableMemory<'a> {
    Slice(&'a mut [u8]),
    IoRegister(&'a IORegister),
    PpuRegister(&'a PpuRegister),
    OutOfBounds,
}

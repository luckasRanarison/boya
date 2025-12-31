use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy)]
pub enum DataType {
    Byte = 1,
    HWord = 2,
    Word = 4,
}

impl DataType {
    #[inline(always)]
    pub fn size(self) -> u8 {
        self as u8
    }
}

#[derive(Debug)]
pub enum MemoryAccess {
    Seq,
    NonSeq,
}

#[derive(Debug, Clone, Copy)]
pub enum MemoryRegion {
    BIOS,
    EWRAM,
    IWRAM,
    IO,
    PALETTE,
    VRAM,
    OAM,
    WAITSTATE0,
    WAITSTATE1,
    WAITSTATE2,
    SRAM,
    NonMapped,
}

impl MemoryRegion {
    pub fn from_address(address: u32) -> Self {
        match address {
            0x0000_0000..=0x0000_3FFF => MemoryRegion::BIOS,
            0x0200_0000..=0x0203_FFFF => MemoryRegion::EWRAM,
            0x0300_0000..=0x0300_7FFF => MemoryRegion::IWRAM,
            0x0400_0000..=0x0400_03FE => MemoryRegion::IO,
            0x0500_0000..=0x0500_03FF => MemoryRegion::PALETTE,
            0x0600_0000..=0x0617_FFFF => MemoryRegion::VRAM,
            0x0700_0000..=0x0700_03FF => MemoryRegion::OAM,
            0x0800_0000..=0x09FF_FFFF => MemoryRegion::WAITSTATE0,
            0x0A00_0000..=0x0BFF_FFFF => MemoryRegion::WAITSTATE1,
            0x0C00_0000..=0x0DFF_FFFF => MemoryRegion::WAITSTATE2,
            0x0E00_0000..=0x0E00_FFFF => MemoryRegion::BIOS,
            _ => MemoryRegion::NonMapped,
        }
    }

    pub fn get_offset(self) -> u32 {
        match self {
            MemoryRegion::BIOS => 0x0000_0000,
            MemoryRegion::EWRAM => 0x0200_0000,
            MemoryRegion::IWRAM => 0x0300_0000,
            MemoryRegion::IO => 0x0400_0000,
            MemoryRegion::PALETTE => 0x0500_0000,
            MemoryRegion::VRAM => 0x0600_0000,
            MemoryRegion::OAM => 0x0700_0000,
            MemoryRegion::WAITSTATE0 => 0x0800_0000,
            MemoryRegion::WAITSTATE1 => 0x0A00_0000,
            MemoryRegion::WAITSTATE2 => 0x0C00_0000,
            MemoryRegion::SRAM => 0x0E00_0000,
            MemoryRegion::NonMapped => 0x0FFF_FFFF,
        }
    }

    pub fn is_gamepak(self) -> bool {
        matches!(
            self,
            MemoryRegion::WAITSTATE0 | MemoryRegion::WAITSTATE1 | MemoryRegion::WAITSTATE2
        )
    }
}

#[derive(Debug)]
pub struct MemoryRegionData {
    pub width: DataType,
    pub waitstate: WaitState,
}

#[derive(Debug, Default)]
pub struct WaitState {
    pub n: u8,
    pub s: u8,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Cycle(u32);

impl Cycle {
    #[inline(always)]
    pub fn new(i: u32, s: u32, n: u32, ws: WaitState) -> Self {
        Self(i + ws.s as u32 * s + s + ws.n as u32 * n + n)
    }

    #[inline(always)]
    pub fn internal(n: u8) -> Self {
        Self(n as u32)
    }

    #[inline(always)]
    pub fn repeat(self, n: u32) -> Self {
        Self(self.0 * n)
    }

    #[inline(always)]
    pub fn count(self) -> u32 {
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

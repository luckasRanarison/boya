use crate::ppu::registers::PpuRegister;

pub mod registers;

pub const PALETTE_RAM_SIZE: usize = 0x400; // 1kb
pub const OAM_SIZE: usize = 0x400; // 1kb
pub const VRAM_SIZE: usize = 0x18_000; // 96kb

#[derive(Debug)]
pub struct Ppu {
    pub palette: [u8; PALETTE_RAM_SIZE],
    pub oam: [u8; OAM_SIZE],
    pub vram: Box<[u8; VRAM_SIZE]>,
    pub registers: PpuRegister,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            palette: [0; PALETTE_RAM_SIZE],
            oam: [0; OAM_SIZE],
            vram: Box::new([0; VRAM_SIZE]),
            registers: PpuRegister::default(),
        }
    }
}

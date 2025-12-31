use crate::registers::ppu::PpuRegister;

pub const PALETTE_RAM_SIZE: usize = 0x400; // 1kb
pub const OAM_SIZE: usize = 0x400; // 1kb
pub const VRAM_SIZE: usize = 0x18_000; // 96kb

#[derive(Debug)]
pub struct Ppu {
    pub palette: [u8; PALETTE_RAM_SIZE],
    pub oam: [u8; OAM_SIZE],
    pub vram: Box<[u8; VRAM_SIZE]>,
    pub registers: PpuRegister,
    pub dot: u16,
    pub scanline: u16,
    pub divider: u32,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            palette: [0; PALETTE_RAM_SIZE],
            oam: [0; OAM_SIZE],
            vram: Box::new([0; VRAM_SIZE]),
            registers: PpuRegister::default(),
            dot: 0,
            scanline: 0,
            divider: 0,
        }
    }
}

impl Ppu {
    pub fn tick(&mut self, cycles: u32) {
        self.divider += cycles;

        while self.divider > 4 {
            self.step();
            self.divider -= 4;
        }
    }

    pub fn step(&mut self) {
        //
    }
}

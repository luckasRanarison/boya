use crate::bus::Bus;

pub const PALETTE_RAM_SIZE: usize = 0x400; // 1kb
pub const OAM_SIZE: usize = 0x400; // 1kb
pub const VRAM_SIZE: usize = 0x18_000; // 96kb

#[derive(Debug)]
pub struct Ppu {
    pub palette_ram: [u8; PALETTE_RAM_SIZE],
    pub oam: [u8; OAM_SIZE],
    pub vram: Box<[u8; VRAM_SIZE]>,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            palette_ram: [0; PALETTE_RAM_SIZE],
            oam: [0; OAM_SIZE],
            vram: Box::new([0; VRAM_SIZE]),
        }
    }
}

impl Bus for Ppu {
    fn read_byte(&self, address: u32) -> u8 {
        match address {
            _ => 0, // open bus
        }
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address {
            _ => {}
        }
    }
}

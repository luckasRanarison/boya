use crate::registers::{
    io::interrupt::Interrupt,
    ppu::{PpuRegister, dispstat::Dispstat},
};

pub const PALETTE_RAM_SIZE: usize = 0x400; // 1kb
pub const OAM_SIZE: usize = 0x400; // 1kb
pub const VRAM_SIZE: usize = 0x18_000; // 96kb

#[derive(Debug)]
pub struct Ppu {
    pub palette: [u8; PALETTE_RAM_SIZE],
    pub oam: [u8; OAM_SIZE],
    pub vram: Box<[u8; VRAM_SIZE]>,
    pub registers: PpuRegister,

    dot: u16,
    scanline: u16,
    divider: u32,
    pending_irq: Option<Interrupt>,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            palette: [0; PALETTE_RAM_SIZE],
            oam: [0; OAM_SIZE],
            vram: Box::new([0; VRAM_SIZE]),
            registers: PpuRegister::default(),
            dot: 1,
            scanline: 1,
            divider: 0,
            pending_irq: None,
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

    pub fn poll_irq(&mut self) -> Option<Interrupt> {
        self.pending_irq.take()
    }

    pub fn step(&mut self) {
        let dispstat = &mut self.registers.dispstat;

        match self.dot {
            1 => {
                dispstat.clear(Dispstat::HBLANK);
            }
            240 => {
                dispstat.set(Dispstat::HBLANK);

                if dispstat.has(Dispstat::HBLANK_IRQ) {
                    self.pending_irq = Some(Interrupt::HBlank);
                }
            }
            308 => {
                self.scanline += 1;
                self.dot = 1;
            }
            _ => {}
        }

        match self.scanline {
            160 if self.dot == 1 => {
                dispstat.set(Dispstat::VBLANK);
            }
            229 => {
                dispstat.clear(Dispstat::VBLANK);
                self.scanline = 1;

                if dispstat.has(Dispstat::VBLANK_IRQ) {
                    self.pending_irq = Some(Interrupt::VBlank);
                }
            }
            _ => {}
        }
    }
}

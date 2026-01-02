pub mod registers;

use crate::{
    bus::types::Interrupt,
    ppu::registers::{PpuRegister, dispstat::Dispstat},
};

pub const PALETTE_RAM_SIZE: usize = 0x400; // 1kb
pub const OAM_SIZE: usize = 0x400; // 1kb
pub const VRAM_SIZE: usize = 0x18_000; // 96kb

pub const LCD_WIDTH: usize = 240;
pub const LCD_HEIGHT: usize = 160;
pub const BUFFER_LEN: usize = LCD_WIDTH * LCD_HEIGHT;

#[derive(Debug)]
pub struct Ppu {
    pub palette: [u8; PALETTE_RAM_SIZE],
    pub oam: [u8; OAM_SIZE],
    pub vram: Box<[u8; VRAM_SIZE]>,
    pub registers: PpuRegister,

    dot: u16,
    scanline: u8,
    divider: u32,
    pending_irq: Option<Interrupt>,
    buffer: Box<[u8; BUFFER_LEN]>,
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
            pending_irq: None,
            buffer: Box::new([0; BUFFER_LEN]),
        }
    }
}

impl Ppu {
    pub fn tick(&mut self, cycles: u32) {
        self.divider += cycles;

        while self.divider >= 4 {
            self.step();
            self.divider -= 4;
        }
    }

    pub fn poll_interrupt(&mut self) -> Option<Interrupt> {
        self.pending_irq.take()
    }

    pub fn get_frame_buffer(&self) -> &[u8; BUFFER_LEN] {
        &self.buffer
    }

    pub fn is_rendering(&self) -> bool {
        !self.registers.dispstat.has(Dispstat::VBLANK)
    }

    pub fn step(&mut self) {
        self.handle_dot();
        self.registers.vcount = self.scanline.into();
        self.handle_scanline();
        self.dot += 1;
    }

    fn handle_dot(&mut self) {
        let dispstat = &mut self.registers.dispstat;

        match self.dot {
            0 => {
                dispstat.clear(Dispstat::HBLANK);
            }
            239 => {
                dispstat.set(Dispstat::HBLANK);

                if dispstat.has(Dispstat::HBLANK_IRQ) {
                    self.pending_irq = Some(Interrupt::HBlank);
                }
            }
            307 => {
                self.scanline += 1;
                self.dot = 0;
            }
            _ => {}
        }
    }

    fn handle_scanline(&mut self) {
        let dispstat = &mut self.registers.dispstat;

        match self.scanline {
            159 if self.dot == 0 => {
                dispstat.set(Dispstat::VBLANK);

                if dispstat.has(Dispstat::VBLANK_IRQ) {
                    self.pending_irq = Some(Interrupt::VBlank);
                }
            }
            228 => {
                self.scanline = 0;
                dispstat.clear(Dispstat::VBLANK);
            }
            _ => {}
        }

        if self.scanline == dispstat.vcount {
            dispstat.set(Dispstat::VCOUNT);

            if dispstat.has(Dispstat::VCOUNT_IRQ) {
                self.pending_irq = Some(Interrupt::VCount);
            }
        } else {
            dispstat.clear(Dispstat::VCOUNT);
        }
    }
}

#[cfg(test)]
mod tets {
    use crate::test::AsmTestBuilder;

    #[test]
    fn test_ppu_timing() {
        // HDRaw: 240 * 4 = 960
        // HBlank: 240 * 4 = 272
        // Single scanline: 960 + 272 = 1232
        // Visible scanlines: 1232 * 160 = 197120

        let asm = r"
            loop:
                B   loop ; 2S + 1N (20)
        ";

        AsmTestBuilder::new()
            .asm(asm)
            .assert_fn(|cpu| {
                assert_eq!(240, cpu.bus.ppu.dot);
            })
            .run(960 / 20);

        AsmTestBuilder::new()
            .asm(asm)
            .assert_fn(|cpu| {
                assert_eq!(160, cpu.bus.ppu.scanline);
            })
            .run(197120 / 20);
    }
}

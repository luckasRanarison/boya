pub mod background;
pub mod color;
pub mod debug;
pub mod registers;

use crate::{
    bus::types::Interrupt,
    ppu::{
        color::Color15,
        registers::{PpuRegister, dispcnt::Background, dispstat::Dispstat},
    },
    utils::bitflags::Bitflag,
};

pub const PALETTE_RAM_SIZE: usize = 0x400; // 1kb
pub const OAM_SIZE: usize = 0x400; // 1kb
pub const VRAM_SIZE: usize = 0x18_000; // 96kb
pub const PALETTE_SIZE: usize = 16 * 2;

pub const LCD_WIDTH: usize = 240;
pub const LCD_HEIGHT: usize = 160;
pub const FRAME_BUFFER_LEN: usize = LCD_WIDTH * LCD_HEIGHT * 4;
pub const TRANS_BUFFER_LEN: usize = 160 * 128 * 2;

#[derive(Debug)]
pub struct Ppu {
    pub palette: [u8; PALETTE_RAM_SIZE],
    pub oam: [u8; OAM_SIZE],
    pub vram: Box<[u8; VRAM_SIZE]>,
    pub registers: PpuRegister,
    pub dot: u16,
    pub scanline: u8,
    pub divider: u32,

    pending_irq: Option<Interrupt>,
    pipeline: RenderPipeline,
    frame_buffer: Box<[u8; FRAME_BUFFER_LEN]>,
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
            pipeline: RenderPipeline::default(),
            frame_buffer: Box::new([0xFF; FRAME_BUFFER_LEN]),
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

    pub fn get_frame_buffer(&self) -> &[u8; FRAME_BUFFER_LEN] {
        &self.frame_buffer
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
        if self.scanline < 160 && self.dot < 240 {
            self.write_pixel();
        }

        match self.dot {
            0 => {
                self.registers.dispstat.clear(Dispstat::HBLANK);
            }
            239 => {
                self.registers.dispstat.set(Dispstat::HBLANK);

                if self.registers.dispstat.has(Dispstat::HBLANK_IRQ) {
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
        match self.scanline {
            0 if self.dot == 0 => {
                self.sort_bg();
                self.apply_bg_transform();
            }
            159 if self.dot == 0 => {
                self.registers.dispstat.set(Dispstat::VBLANK);

                if self.registers.dispstat.has(Dispstat::VBLANK_IRQ) {
                    self.pending_irq = Some(Interrupt::VBlank);
                }
            }
            228 => {
                self.scanline = 0;
                self.registers.dispstat.clear(Dispstat::VBLANK);
            }
            _ => {}
        }

        if self.scanline == self.registers.dispstat.vcount {
            self.registers.dispstat.set(Dispstat::VCOUNT);

            if self.registers.dispstat.has(Dispstat::VCOUNT_IRQ) {
                self.pending_irq = Some(Interrupt::VCount);
            }
        } else {
            self.registers.dispstat.clear(Dispstat::VCOUNT);
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Pixel(u16);

impl Pixel {
    pub fn is_transparent(self) -> bool {
        self.0.has(15)
    }

    pub fn get_color(self) -> Color15 {
        Color15::from(self.0 & 0x7FFF)
    }
}

#[derive(Debug)]
pub struct RenderPipeline {
    bg_prio: [Background; 4],
    bg2_buffer: Box<[Pixel; TRANS_BUFFER_LEN]>,
    bg3_buffer: Box<[Pixel; TRANS_BUFFER_LEN]>,
    bg2_buffer_enabled: bool,
    bg3_buffer_enabled: bool,
}

impl Default for RenderPipeline {
    fn default() -> Self {
        Self {
            bg_prio: [
                Background::Bg0,
                Background::Bg1,
                Background::Bg2,
                Background::Bg3,
            ],
            bg2_buffer: Box::new([Pixel(0); TRANS_BUFFER_LEN]),
            bg3_buffer: Box::new([Pixel(0); TRANS_BUFFER_LEN]),
            bg2_buffer_enabled: false,
            bg3_buffer_enabled: false,
        }
    }
}

#[cfg(test)]
mod tets {
    use crate::test::GbaTestBuilder;

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

        // PPU is not clocked during the fake BIOS setup
        GbaTestBuilder::new()
            .asm(asm)
            .assert_fn(|cpu| {
                assert_eq!(240, cpu.bus.ppu.dot);
            })
            .run(960 / 20);

        GbaTestBuilder::new()
            .asm(asm)
            .assert_fn(|cpu| {
                assert_eq!(160, cpu.bus.ppu.scanline);
            })
            .run(197120 / 20);
    }
}

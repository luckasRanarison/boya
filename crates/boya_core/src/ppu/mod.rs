pub mod background;
pub mod character;
pub mod color;
pub mod object;
pub mod registers;

use crate::{
    bus::types::Interrupt,
    ppu::{
        color::{Color15, Color24},
        object::ObjPool,
        registers::{PpuRegister, dispcnt::Background, dispstat::Dispstat},
    },
    utils::Reset,
};

pub const PALETTE_RAM_SIZE: usize = 0x400; // 1kb
pub const OAM_SIZE: usize = 0x400; // 1kb
pub const VRAM_SIZE: usize = 0x18_000; // 96kb

pub const PALETTE_SIZE: usize = 16 * 2;
pub const OBJ_COUNT: u8 = 128;

pub const LCD_WIDTH: usize = 240;
pub const LCD_HEIGHT: usize = 160;
pub const FRAME_BUFFER_LEN: usize = LCD_WIDTH * LCD_HEIGHT * 4;

#[derive(Debug)]
pub struct Ppu {
    pub palette: [u8; PALETTE_RAM_SIZE],
    pub oam: [u8; OAM_SIZE],
    pub vram: Box<[u8; VRAM_SIZE]>,
    pub registers: PpuRegister,
    pub dot: u16,
    pub scanline: u8,
    pub divider: u32,
    pub mask_vblank: bool,
    pub mask_hblank: bool,

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
            mask_vblank: false,
            mask_hblank: false,
            pending_irq: None,
            pipeline: RenderPipeline::default(),
            frame_buffer: Box::new([0x00; FRAME_BUFFER_LEN]),
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
        self.registers.vcount = self.scanline.into();
        self.handle_scanline();
        self.handle_dot();
    }

    pub fn read_vram(&self, address: u32) -> u8 {
        self.vram[self.vram_offset(address)]
    }

    pub fn write_vram(&mut self, address: u32, value: u8) {
        self.vram[self.vram_offset(address)] = value;
    }

    pub fn write_pixel(&mut self) {
        let x = self.dot;
        let y = self.scanline as u16;
        let idx = (y as usize * LCD_WIDTH + x as usize) * 4;
        let color15 = self.get_pixel(x, y);
        let color24 = Color24::from(color15);

        self.frame_buffer[idx] = color24.r;
        self.frame_buffer[idx + 1] = color24.g;
        self.frame_buffer[idx + 2] = color24.b;
    }

    pub fn get_pixel(&mut self, x: u16, y: u16) -> Color15 {
        for bg in self.pipeline.bg_prio {
            if let Some(pixel) = self.get_obj_pixel(x, y, bg as u8) {
                return pixel;
            }

            if let Some(pixel) = self.get_bg_pixel(x, y, bg) {
                return pixel;
            }
        }

        Color15::default()
    }

    fn vram_offset(&self, address: u32) -> usize {
        let base = address as usize & 0x1FFFF;

        if base >= VRAM_SIZE {
            base - 0x8000
        } else {
            base
        }
    }

    fn handle_dot(&mut self) {
        match self.dot {
            0 if self.scanline < 160 => {
                self.registers.dispstat.clear(Dispstat::HBLANK);
                self.load_obj_pool();
            }
            239..=306 if self.scanline < 160 => {
                self.registers.dispstat.set(Dispstat::HBLANK);

                if !self.mask_hblank && self.registers.dispstat.has(Dispstat::HBLANK_IRQ) {
                    self.pending_irq = Some(Interrupt::HBlank);
                }
            }
            307 => {
                self.scanline += 1;
                self.dot = 0;
                self.mask_hblank = false;
                return;
            }
            _ => {}
        }

        if self.scanline < 160 && self.dot < 240 {
            self.write_pixel();
        }

        self.dot += 1;
    }

    fn handle_scanline(&mut self) {
        match self.scanline {
            0 if self.dot == 0 => {
                self.sort_bg();
            }
            160..=227 => {
                self.registers.dispstat.set(Dispstat::VBLANK);

                if !self.mask_vblank && self.registers.dispstat.has(Dispstat::VBLANK_IRQ) {
                    self.pending_irq = Some(Interrupt::VBlank);
                }
            }
            228 => {
                self.scanline = 0;
                self.mask_vblank = false;
                self.registers.dispstat.clear(Dispstat::VBLANK);
            }
            _ => {}
        }

        if self.registers.vcount == self.registers.dispstat.vcount.into() {
            self.registers.dispstat.set(Dispstat::VCOUNT);

            if self.dot == 0 && self.registers.dispstat.has(Dispstat::VCOUNT_IRQ) {
                self.pending_irq = Some(Interrupt::VCount);
            }
        } else {
            self.registers.dispstat.clear(Dispstat::VCOUNT);
        }
    }
}

impl Reset for Ppu {
    fn reset(&mut self) {
        self.palette.fill(0);
        self.oam.fill(0);
        self.vram.fill(0);
        self.registers = PpuRegister::default();
        self.dot = 0;
        self.scanline = 0;
        self.divider = 0;
        self.pending_irq = None;
        self.pipeline = RenderPipeline::default();
        self.frame_buffer.fill(0xFF);
    }
}

#[derive(Debug)]
pub struct RenderPipeline {
    bg_prio: [Background; 4],
    obj_pool: ObjPool,
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
            obj_pool: ObjPool::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransformParam {
    pub pa: u16,
    pub pb: u16,
    pub pc: u16,
    pub pd: u16,
    pub x: u32,
    pub y: u32,
}

impl Default for TransformParam {
    fn default() -> Self {
        Self {
            pa: 256,
            pb: 0,
            pc: 0,
            pd: 256,
            x: 0,
            y: 0,
        }
    }
}

impl TransformParam {
    pub fn map(&self, x: i32, y: i32) -> (u16, u16) {
        let pa = self.pa as i16 as i32;
        let pb = self.pb as i16 as i32;
        let pc = self.pc as i16 as i32;
        let pd = self.pd as i16 as i32;

        let tx = (self.x as i32).wrapping_add(x * pa).wrapping_add(y * pb) >> 8;
        let ty = (self.y as i32).wrapping_add(x * pc).wrapping_add(y * pd) >> 8;

        (tx as u16, ty as u16)
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

use crate::{
    bus::Bus,
    ppu::{
        LCD_WIDTH, PALETTE_SIZE, Ppu,
        color::{Color15, Color24},
        registers::{
            bgcnt::{Bgcnt, ColorMode},
            dispcnt::{Background, BgMode, FrameBuffer, TransBackground},
        },
    },
    utils::bitflags::Bitflag,
};

pub const TILE_BUFFER_SIZE: usize = 8 * 8 * 4;
pub const TILE4BPP_SIZE: usize = 32;
pub const TILE8BPP_SIZE: usize = 64;

#[derive(Debug, Clone, Copy)]
pub enum ColorSrc {
    Palette,
    RGB,
}

#[derive(Debug, Clone, Copy)]
pub enum BackgroundKind {
    Text,
    Affine,
}

#[derive(Debug, Default, Clone, Copy)]

pub struct BgScreen {
    character: u16,
    hflip: bool,
    vflip: bool,
    palette: u8,
}

impl From<u16> for BgScreen {
    fn from(value: u16) -> Self {
        Self {
            character: value.get_bits(0, 9),
            hflip: value.has(10),
            vflip: value.has(11),
            palette: value.get_bits_u8(12, 15),
        }
    }
}

impl Ppu {
    pub fn write_pixel(&mut self) {
        for bg in self.pipeline.bg_prio {
            let x = self.dot;
            let y = self.scanline as u16;

            if let Some(pixel15) = self.get_bg_pixel(x, y, bg) {
                let Color24 { r, g, b } = pixel15.into();
                let idx = (self.scanline as usize * LCD_WIDTH + self.dot as usize) * 4;

                self.frame_buffer[idx] = r;
                self.frame_buffer[idx + 1] = g;
                self.frame_buffer[idx + 2] = b;

                break;
            }
        }
    }

    pub fn get_bg_pixel(&mut self, x: u16, y: u16, bg: Background) -> Option<Color15> {
        if !self.registers.dispcnt.is_bg_enabled(bg) {
            return None;
        }

        let bg_mode = self.registers.dispcnt.bg_mode();

        match (bg_mode, bg) {
            (BgMode::Mode0, _) => self.get_bg_tile_pixel(x, y, bg, BackgroundKind::Text),
            (BgMode::Mode1, _) => todo!(),
            (BgMode::Mode2, _) => todo!(),
            (BgMode::Mode3, Background::Bg2) => self.get_bg_bmp_pixel(x, y, ColorSrc::RGB, 1),
            (BgMode::Mode4, Background::Bg2) => self.get_bg_bmp_pixel(x, y, ColorSrc::Palette, 2),
            (BgMode::Mode5, Background::Bg2) => self.get_bg_bmp_pixel(x, y, ColorSrc::RGB, 2),
            _ => None,
        }
    }

    pub fn get_bg_tile_pixel(
        &self,
        x: u16,
        y: u16,
        bg: Background,
        bg_kind: BackgroundKind,
    ) -> Option<Color15> {
        let bg_idx = bg.to_index();
        let bgcnt = self.registers.bgcnt[bg_idx];
        let bgofs = self.registers.bgofs[bg_idx];

        let tx = x + bgofs.x;
        let ty = y + bgofs.y;
        let sx = (tx / 8) as u32;
        let sy = (ty / 8) as u32;
        let cx = (tx % 8) as u32;
        let cy = (ty % 8) as u32;

        let (width, _height) = bgcnt.screen_mode().text_size();
        let base_screen_offset = bgcnt.screen_block_offset();
        let screen_block_offset = base_screen_offset + (sx + sy * (width / 8) as u32) * 2;
        let raw_bg_screen = self.vram.read_hword(screen_block_offset);
        let bg_screen = BgScreen::from(raw_bg_screen);
        let char_id = bg_screen.character as u32;
        let base_char_offset = bgcnt.char_block_offset();

        let (base_palette, rel_color_id) = match bgcnt.color_mode() {
            ColorMode::Palette16 => {
                let base_char_addr = base_char_offset + char_id * TILE4BPP_SIZE as u32;
                let base_palette = bg_screen.palette as u32 * 16;
                let pixel_addr = base_char_addr + ((cy * 8 + cx) / 2);
                let pixels = self.vram.read_byte(pixel_addr);
                let (b_start, b_end) = if cx & 1 == 0 { (0, 3) } else { (4, 7) };
                let color_id = pixels.get_bits_u8(b_start, b_end);
                (base_palette, color_id as u32)
            }
            ColorMode::Palette256 => todo!(),
        };

        if rel_color_id == 0 {
            return None;
        }

        let color_addr = base_palette + rel_color_id;
        let color = self.read_bg_palette(color_addr);

        Some(color)
    }

    pub fn get_bg_bmp_pixel(
        &self,
        x: u16,
        y: u16,
        color_mode: ColorSrc,
        buffer_count: u8,
    ) -> Option<Color15> {
        let (width, height, pixel_size) = match (color_mode, buffer_count) {
            (ColorSrc::RGB, 2) => (160, 128, 2),
            (ColorSrc::RGB, _) => (240, 160, 2),
            (ColorSrc::Palette, _) => (240, 160, 1),
        };

        let t = &self.registers.bg2trans;
        let bgcnt = self.registers.bgcnt[2];
        let frame_buffer = self.registers.dispcnt.frame_buffer();
        let buffer_size = width * height * pixel_size;
        let buffer_start = frame_buffer as usize * pixel_size;
        let buffer_slice = &self.vram[buffer_start..buffer_start + buffer_size];
        let tx = t.pa as i32 * x as i32 + t.pb as i32 * y as i32 + t.x as i32;
        let ty = t.pc as i32 * x as i32 + t.pd as i32 * y as i32 + t.y as i32;
        let sx = (tx >> 8) as usize;
        let sy = (ty >> 8) as usize;
        let idx = (sy * width + sx) * pixel_size;

        if (sx >= width || sy >= height) && !bgcnt.overflow_wrap() {
            return None;
        }

        let entry_lo = buffer_slice[idx % buffer_size];
        let entry_hi = buffer_slice[(idx + 1) % buffer_size];
        let entry = u16::from_le_bytes([entry_lo, entry_hi]);

        match color_mode {
            ColorSrc::RGB => Some(entry.into()),
            ColorSrc::Palette => Some(self.read_bg_palette(entry)),
        }
    }

    pub fn sort_bg(&mut self) {
        self.pipeline.bg_prio.sort_by(|a, b| {
            let a_prio = self.registers.bgcnt[a.to_index()].bg_priority();
            let b_prio = self.registers.bgcnt[b.to_index()].bg_priority();
            b_prio.cmp(&a_prio)
        });
    }

    fn read_bg_palette<I: Into<u32>>(&self, index: I) -> Color15 {
        self.palette.read_hword(index.into() * 2).into()
    }
}

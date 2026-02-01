use crate::{
    bus::Bus,
    ppu::{
        LCD_WIDTH, PALETTE_SIZE, Ppu, TransformParam,
        character::{CharacterData, CharacterKind},
        color::{Color15, Color24},
        registers::{
            bgcnt::{Bgcnt, ColorMode},
            dispcnt::{Background, BgMode, FrameBuffer, TransBackground, VramMapping},
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
pub enum BgKind {
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
        let x = self.dot;
        let y = self.scanline as u16;
        let idx = (y as usize * LCD_WIDTH + x as usize) * 4;
        let Color24 { r, g, b } = self.get_pixel(x, y).into();

        self.frame_buffer[idx] = r;
        self.frame_buffer[idx + 1] = g;
        self.frame_buffer[idx + 2] = b;
    }

    pub fn get_pixel(&mut self, x: u16, y: u16) -> Color15 {
        for bg in self.pipeline.bg_prio {
            if let Some(pixel15) = self.get_bg_pixel(x, y, bg) {
                return pixel15;
            }
        }

        Color15::default()
    }

    pub fn get_bg_pixel(&mut self, x: u16, y: u16, bg: Background) -> Option<Color15> {
        if !self.registers.dispcnt.is_bg_enabled(bg) {
            return None;
        }

        let bg_mode = self.registers.dispcnt.bg_mode();

        match (bg_mode, bg) {
            (BgMode::Mode0, _) => self.get_bg_tile_pixel(x, y, bg, BgKind::Text),
            (BgMode::Mode1, Background::Bg0) => self.get_bg_tile_pixel(x, y, bg, BgKind::Text),
            (BgMode::Mode1, Background::Bg1) => self.get_bg_tile_pixel(x, y, bg, BgKind::Text),
            (BgMode::Mode1, Background::Bg2) => None, // TODO
            (BgMode::Mode2, Background::Bg2) => None, // TODO
            (BgMode::Mode2, Background::Bg3) => None, // TODO
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
        bg_kind: BgKind,
    ) -> Option<Color15> {
        let bg_idx = bg.to_index();
        let bgcnt = self.registers.bgcnt[bg_idx];
        let bgofs = self.registers.bgofs[bg_idx];

        let tx = x + bgofs.x;
        let ty = y + bgofs.y;
        let sx = (tx / 8) as u32;
        let sy = (ty / 8) as u32;
        let cx = (tx % 8);
        let cy = (ty % 8);

        let (width, _height) = bgcnt.screen_mode().text_size();
        let base_screen_offset = bgcnt.screen_block_offset();
        let base_char_offset = bgcnt.char_block_offset();
        let screen_block_offset = base_screen_offset + (sx + sy * (width / 8) as u32) * 2;
        let raw_bg_screen = self.vram.read_hword(screen_block_offset);
        let bg_screen = BgScreen::from(raw_bg_screen);

        let char_data = CharacterData {
            name: bg_screen.character,
            base_offset: base_char_offset,
            hflip: bg_screen.hflip,
            vflip: bg_screen.vflip,
            transform: None,
            color: bgcnt.color_mode(),
            palette: bg_screen.palette,
            kind: CharacterKind::Background,
            height: 8,
            width: 8,
        };

        self.get_char_pixel(cx, cy, char_data)
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
            ColorSrc::Palette => Some(self.read_bg_palette(entry as u32)),
        }
    }

    pub fn sort_bg(&mut self) {
        self.pipeline.bg_prio.sort_by(|a, b| {
            let a_prio = self.registers.bgcnt[a.to_index()].bg_priority();
            let b_prio = self.registers.bgcnt[b.to_index()].bg_priority();
            b_prio.cmp(&a_prio)
        });
    }

    pub fn read_bg_palette(&self, index: u32) -> Color15 {
        self.palette.read_hword(index * 2).into()
    }
}

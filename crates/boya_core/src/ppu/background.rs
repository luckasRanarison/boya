use crate::{
    bus::Bus,
    ppu::{
        LCD_WIDTH, Ppu,
        color::{Color15, Color24},
        registers::{
            bgcnt::{Bgcnt, ColorMode},
            dispcnt::{Background, BgMode, FrameBuffer, TransBackground},
        },
    },
    utils::bitflags::Bitflag,
};

#[derive(Debug, Clone, Copy)]
pub enum BitmapColor {
    Palette,
    RGB,
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

        match self.registers.dispcnt.bg_mode() {
            BgMode::Mode0 => Color15::default().into(), //
            BgMode::Mode1 => Color15::default().into(), //
            BgMode::Mode2 => Color15::default().into(), // TODO: implementation
            BgMode::Mode3 => self.get_bg_bitmap_pixel(x, y, bg, BitmapColor::RGB, 1),
            BgMode::Mode4 => self.get_bg_bitmap_pixel(x, y, bg, BitmapColor::Palette, 2),
            BgMode::Mode5 => self.get_bg_bitmap_pixel(x, y, bg, BitmapColor::RGB, 2),
        }
    }

    pub fn get_bg_bitmap_pixel(
        &self,
        x: u16,
        y: u16,
        bg: Background,
        color_mode: BitmapColor,
        buffer_count: u8,
    ) -> Option<Color15> {
        if !matches!(bg, Background::Bg2) {
            return None;
        }

        let (width, height, pixel_size) = match (color_mode, buffer_count) {
            (BitmapColor::RGB, 2) => (160, 128, 2),
            (BitmapColor::RGB, _) => (160, 128, 2),
            (BitmapColor::Palette, _) => (240, 160, 1),
        };

        let t = &self.registers.bg2trans;
        let frame_buffer = self.registers.dispcnt.frame_buffer();
        let buffer_size = width * height * pixel_size;
        let buffer_start = frame_buffer as usize * pixel_size;
        let buffer_slice = &self.vram[buffer_start..buffer_start + buffer_size];
        let tx = t.pa as u32 * x as u32 + t.pb as u32 * x as u32 + t.x;
        let ty = t.pc as u32 * y as u32 + t.pd as u32 * y as u32 + t.y;
        let sx = (tx >> 8) as usize;
        let sy = (tx >> 8) as usize;
        let idx = (sy * width + sx) * pixel_size;
        let entry_lo = buffer_slice[idx];
        let entry_hi = buffer_slice[idx + 1];
        let entry = u16::from_le_bytes([entry_lo, entry_hi]);

        match color_mode {
            BitmapColor::RGB => Some(entry.into()),
            BitmapColor::Palette => Some(self.palette.read_hword(entry as u32 * 2).into()),
        }
    }

    pub fn sort_bg(&mut self) {
        self.pipeline.bg_prio.sort_by(|a, b| {
            let a_prio = self.registers.bgcnt[a.to_index()].bg_priority();
            let b_prio = self.registers.bgcnt[b.to_index()].bg_priority();
            b_prio.cmp(&a_prio)
        });
    }
}

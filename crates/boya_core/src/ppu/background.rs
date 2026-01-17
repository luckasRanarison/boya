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

#[derive(Debug)]
pub enum BitmapColor {
    Palette,
    RGB,
}

impl Ppu {
    pub fn write_pixel(&mut self) {
        for bg in self.pipeline.bg_prio {
            if let Some(pixel15) = self.get_bg_pixel(bg) {
                let Color24 { r, g, b } = pixel15.into();
                let idx = (self.scanline as usize * LCD_WIDTH + self.dot as usize) * 4;

                self.frame_buffer[idx] = r;
                self.frame_buffer[idx + 1] = g;
                self.frame_buffer[idx + 2] = b;
            }
        }
    }

    pub fn get_bg_pixel(&mut self, bg: Background) -> Option<Color15> {
        if !self.registers.dispcnt.is_bg_enabled(bg) {
            return None;
        }

        match self.registers.dispcnt.bg_mode() {
            BgMode::Mode0 => Color15::default().into(), //
            BgMode::Mode1 => Color15::default().into(), //
            BgMode::Mode2 => Color15::default().into(), // TODO: implementation
            BgMode::Mode3 => self.get_bg_pixel_bitmap(bg, BitmapColor::RGB, 1),
            BgMode::Mode4 => self.get_bg_pixel_bitmap(bg, BitmapColor::Palette, 2),
            BgMode::Mode5 => self.get_bg_pixel_bitmap(bg, BitmapColor::RGB, 2),
        }
    }

    pub fn get_bg_pixel_bitmap(
        &self,
        bg: Background,
        color_mode: BitmapColor,
        buffer_count: u8,
    ) -> Option<Color15> {
        if !matches!(bg, Background::Bg2) {
            return None;
        }

        let frame_buffer = self.registers.dispcnt.frame_buffer();

        Color15::default().into() // TODO: implementation
    }

    pub fn apply_bg_transform(&mut self) {
        self.pipeline.bg2_buffer_enabled = self.check_bg_transform(TransBackground::Bg2);
        self.pipeline.bg3_buffer_enabled = self.check_bg_transform(TransBackground::Bg3);

        if self.pipeline.bg2_buffer_enabled {
            self.tranform_bg(TransBackground::Bg2);
        }

        if self.pipeline.bg3_buffer_enabled {
            self.tranform_bg(TransBackground::Bg3);
        }
    }

    pub fn sort_bg(&mut self) {
        self.pipeline.bg_prio.sort_by(|a, b| {
            let a_prio = self.registers.bgcnt[a.to_index()].bg_priority();
            let b_prio = self.registers.bgcnt[b.to_index()].bg_priority();
            b_prio.cmp(&a_prio)
        });
    }

    fn tranform_bg(&mut self, trans_bg: TransBackground) {
        // TODO: implementation
        // let bg = Background::from(trans_bg);
    }

    fn check_bg_transform(&self, bg: TransBackground) -> bool {
        let bgtrans = match bg {
            TransBackground::Bg2 => &self.registers.bg2trans,
            TransBackground::Bg3 => &self.registers.bg3trans,
        };

        if bgtrans.zero_transform() {
            return false;
        }

        match self.registers.dispcnt.bg_mode() {
            BgMode::Mode0 => false,
            _ => self.registers.dispcnt.is_bg_enabled(bg.into()),
        }
    }
}

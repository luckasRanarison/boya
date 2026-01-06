use crate::{
    ppu::{
        LCD_HEIGHT, LCD_WIDTH, Ppu,
        registers::dispcnt::{Background, BgMode},
    },
    utils::bitflags::Bitflag,
};

impl Ppu {
    pub fn update_buffer(&mut self) {
        let mode = self.registers.dispcnt.bg_mode();

        match mode {
            BgMode::Mode0 => {}
            BgMode::Mode1 => {}
            BgMode::Mode2 => {}
            BgMode::Mode3 => self.draw_bg_mode3(),
            BgMode::Mode4 => {}
            BgMode::Mode5 => {}
        }
    }

    fn draw_bg_mode3(&mut self) {
        if !self.registers.dispcnt.is_bg_enabled(Background::Bg2) {
            return;
        }

        for i in 0..LCD_HEIGHT * LCD_WIDTH {
            let lo = self.vram[i * 2];
            let hi = self.vram[i * 2 + 1];
            let r5 = lo.get_bits(0, 4);
            let g5 = lo.get_bits(5, 7) | (hi.get_bits(0, 1) << 3);
            let b5 = hi.get_bits(2, 6);
            self.buffer[i * 4] = (r5 << 3) | (r5 >> 2);
            self.buffer[i * 4 + 1] = (g5 << 3) | (g5 >> 2);
            self.buffer[i * 4 + 2] = (b5 << 3) | (b5 >> 2);
        }
    }
}

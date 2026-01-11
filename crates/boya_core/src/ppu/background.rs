use crate::{
    bus::Bus,
    ppu::{
        LCD_WIDTH, Ppu,
        registers::{
            bgcnt::{Bgcnt, ColorMode},
            dispcnt::{Background, BgMode},
        },
    },
    utils::bitflags::Bitflag,
};

pub const TILE_SIZE: u32 = 8;
pub const PALETTE_SIZE: u32 = 16 * 2;
pub const COLOR16_TILE_SIZE: u32 = (8 * 8) / 2;
pub const COLOR256_TILE_SIZE: u32 = 8 * 8;

#[derive(Debug, Default, Clone, Copy)]
pub struct TextBgScreen {
    character: u16,
    hflip: bool,
    vflip: bool,
    palette: u8,
}

impl From<u16> for TextBgScreen {
    fn from(value: u16) -> Self {
        Self {
            character: value.get_bits(0, 9),
            hflip: value.has(10),
            vflip: value.has(11),
            palette: value.get_bits_u8(12, 15),
        }
    }
}

#[derive(Debug, Default)]
pub struct BgCharacterBuffer {
    pub bg_screen: TextBgScreen,
    pub offset_x: u8,
    pub offset_y: u8,
}

#[derive(Debug)]
pub enum BgPipeline {
    Screen(BgCharacterBuffer),
}

#[derive(Debug)]
pub struct Pixel {
    color: u16,
}

impl Ppu {
    pub fn load_bg_screen(&mut self) {
        match self.registers.dispcnt.bg_mode() {
            BgMode::Mode0 => {
                self.load_text_bg_screen(Background::Bg0);
                self.load_text_bg_screen(Background::Bg1);
                self.load_text_bg_screen(Background::Bg2);
                self.load_text_bg_screen(Background::Bg3);
            }
            BgMode::Mode1 => {} // TODO
            BgMode::Mode2 => {} // TODO
            BgMode::Mode3 | BgMode::Mode4 | BgMode::Mode5 => {}
        }
    }

    pub fn write_bg_dot(&mut self) {
        for bg_prio in self.pipeline.bg_priority {
            if let Some(pixel) = self.get_bg_pixel(bg_prio) {
                let r5 = pixel.color.get_bits_u8(0, 4);
                let g5 = pixel.color.get_bits_u8(5, 9);
                let b5 = pixel.color.get_bits_u8(10, 14);

                let r8 = (r5 << 3) | (r5 >> 2);
                let g8 = (g5 << 3) | (g5 >> 2);
                let b8 = (b5 << 3) | (b5 >> 2);

                let idx = self.scanline as usize * LCD_WIDTH * 4 + self.dot as usize * 4;

                if idx < self.buffer.len() {
                    self.buffer[idx] = r8;
                    self.buffer[idx + 1] = g8;
                    self.buffer[idx + 2] = b8;
                }
            }
        }

        self.flush_bg_pipeline();
    }

    pub fn get_bg_pixel(&self, bg: Background) -> Option<Pixel> {
        let bg_idx = bg.as_index();
        let pipeline = self.pipeline.bg[bg_idx].as_ref()?;
        let bgcnt = self.registers.bgcnt[bg_idx];

        let color = match pipeline {
            BgPipeline::Screen(screen) => {
                let char_offset = bgcnt.char_block_offset();
                let char_id = screen.bg_screen.character as u32;

                match bgcnt.color_mode() {
                    ColorMode::Palette16 => {
                        let base_char_addr = char_offset + char_id * COLOR16_TILE_SIZE;
                        let base_pal_addr = PALETTE_SIZE * screen.bg_screen.palette as u32;

                        let pixel_addr = base_char_addr
                            + screen.offset_x as u32 / 2
                            + screen.offset_y as u32 * 4;

                        let pixels = self.vram.read_byte(pixel_addr);

                        let color_id = match screen.offset_x % 2 {
                            0 => pixels.get_bits(0, 3),
                            _ => pixels.get_bits(4, 7),
                        };

                        let color_addr = base_pal_addr + color_id as u32 * 2;

                        self.palette.read_hword(color_addr)
                    }
                    ColorMode::Palette256 => todo!("palette 256"),
                }
            }
        };

        Some(Pixel { color })
    }

    pub fn load_text_bg_screen(&mut self, bg: Background) {
        let bg_idx = bg.as_index();

        if !self.registers.dispcnt.is_bg_enabled(bg) || self.pipeline.bg[bg_idx].is_some() {
            return;
        }

        let bgcnt = self.registers.bgcnt[bg_idx];
        let bgofs = self.registers.bgofs[bg_idx];
        let rel_offset_x = if self.dot == 0 { bgofs.x % 8 } else { 0 };
        let rel_offset_y = if self.scanline == 0 { bgofs.y % 8 } else { 0 };
        let (width, _height) = bgcnt.screen_mode().text_size();

        let screen_block_offset = bgcnt.screen_block_offset()
            + ((self.dot + bgofs.x) / 8) as u32 * 2
            + ((self.scanline as u16 + bgofs.y) / 8) as u32 * width as u32 * 2
            + ((self.scanline as u32 * (width - self.dot) as u32) / 8) * 2;

        let raw_bg_screen = self.vram.read_hword(screen_block_offset);

        let buffer = BgCharacterBuffer {
            bg_screen: TextBgScreen::from(raw_bg_screen),
            offset_x: ((self.dot % 8) + rel_offset_x) as u8,
            offset_y: ((self.scanline as u16 % 8) + rel_offset_y) as u8,
        };

        self.pipeline.bg[bg_idx] = Some(BgPipeline::Screen(buffer));
    }

    pub fn sort_bg(&mut self) {
        self.pipeline.bg_priority.sort_by(|a, b| {
            let a_prio = self.registers.bgcnt[a.as_index()].bg_priority();
            let b_prio = self.registers.bgcnt[b.as_index()].bg_priority();
            b_prio.cmp(&a_prio)
        });
    }

    fn flush_bg_pipeline(&mut self) {
        for bg_pipeline in self.pipeline.bg.iter_mut() {
            if let Some(pipeline) = bg_pipeline {
                match pipeline {
                    BgPipeline::Screen(screen) => {
                        screen.offset_x += 1;

                        if screen.offset_x > 8 {
                            bg_pipeline.take();
                        }
                    }
                }
            }
        }
    }
}

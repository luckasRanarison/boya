use crate::{
    bus::Bus,
    ppu::{
        Ppu, TransformParam,
        character::{CharacterData, CharacterKind},
        pixel::{Color15, PixelContext, PixelResult},
        registers::{
            bgcnt::ColorMode,
            dispcnt::{Background, BgMode},
        },
    },
    utils::bitflags::Bitflag,
};

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

impl BgScreen {
    fn text(value: u16) -> Self {
        Self {
            character: value.get_bits(0, 9),
            hflip: value.has(10),
            vflip: value.has(11),
            palette: value.get_bits_u8(12, 15),
        }
    }

    fn affine(value: u8) -> Self {
        Self {
            character: value.into(),
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct PartialCharData {
    pub x: u16,
    pub y: u16,
    pub transform: Option<TransformParam>,
    pub color_mode: ColorMode,
    pub bg_screen: BgScreen,
}

impl Ppu {
    pub fn read_bg_palette(&self, id: u8) -> Color15 {
        self.palette.read_hword(id as u32 * 2).into()
    }

    pub fn get_bg_priority(&self, bg: Background) -> u8 {
        self.registers.bgcnt[bg.to_index()].bg_priority()
    }

    pub fn sort_bg(&mut self) {
        self.pipeline.sorted_bg.sort_by(|a, b| {
            let a_idx = a.to_index();
            let b_idx = b.to_index();

            let a_prio = self.registers.bgcnt[a_idx].bg_priority();
            let b_prio = self.registers.bgcnt[b_idx].bg_priority();

            if a_prio == b_prio {
                a_idx.cmp(&b_idx)
            } else {
                a_prio.cmp(&b_prio)
            }
        });
    }

    pub fn get_bg_pixel_result(
        &self,
        x: u16,
        y: u16,
        bg: Background,
        ctx: &PixelContext,
    ) -> Option<PixelResult> {
        if !self.window_bg_enable(ctx.window, bg) {
            return None;
        }

        let pixel = self.get_bg_pixel(x, y, bg)?;

        if !self.window_fx_enable(ctx.window) {
            Some(PixelResult::Top(pixel))
        } else if self.registers.bldcnt.is_bg_second_target(bg) && ctx.acc.top.is_some() {
            Some(PixelResult::Bottom(pixel))
        } else if self.registers.bldcnt.is_bg_first_target(bg) && ctx.acc.top.is_none() {
            Some(PixelResult::BlendTop(pixel))
        } else {
            Some(PixelResult::Top(pixel))
        }
    }

    pub fn get_bg_pixel(&self, x: u16, y: u16, bg: Background) -> Option<Color15> {
        let bg_mode = self.registers.dispcnt.bg_mode();

        match (bg_mode, bg) {
            (BgMode::Mode0, _) => self.get_bg_tile_pixel(x, y, bg, BgKind::Text),
            (BgMode::Mode1, Background::Bg0) => self.get_bg_tile_pixel(x, y, bg, BgKind::Text),
            (BgMode::Mode1, Background::Bg1) => self.get_bg_tile_pixel(x, y, bg, BgKind::Text),
            (BgMode::Mode1, Background::Bg2) => self.get_bg_tile_pixel(x, y, bg, BgKind::Affine),
            (BgMode::Mode2, Background::Bg2) => self.get_bg_tile_pixel(x, y, bg, BgKind::Affine),
            (BgMode::Mode2, Background::Bg3) => self.get_bg_tile_pixel(x, y, bg, BgKind::Affine),
            (BgMode::Mode3, Background::Bg2) => self.get_bg_bmp_pixel(x, y, ColorSrc::RGB, 1),
            (BgMode::Mode4, Background::Bg2) => self.get_bg_bmp_pixel(x, y, ColorSrc::Palette, 2),
            (BgMode::Mode5, Background::Bg2) => self.get_bg_bmp_pixel(x, y, ColorSrc::RGB, 2),
            _ => None,
        }
    }

    fn get_bg_tile_pixel(
        &self,
        x: u16,
        y: u16,
        bg: Background,
        bg_kind: BgKind,
    ) -> Option<Color15> {
        let bg_idx = bg.to_index();
        let bgcnt = self.registers.bgcnt[bg_idx];

        let base_screen_offset = bgcnt.screen_block_offset();
        let base_char_offset = bgcnt.char_block_offset();
        let screen_mode = bgcnt.screen_mode();

        let partial_data = match bg_kind {
            BgKind::Text => {
                let bgofs = self.registers.bgofs[bg_idx];
                let (width, height) = screen_mode.text_size();

                let ox = (x + bgofs.x) % width;
                let oy = (y + bgofs.y) % height;
                let screen_x = (ox / 8) as u32;
                let screen_y = (oy / 8) as u32;
                let tile_x = screen_x % 32;
                let tile_y = screen_y % 32;

                let block_id = match (width, height) {
                    (512, 256) => screen_x / 32,
                    (256, 512) => screen_y / 32,
                    (512, 512) => (screen_x / 32) + (screen_y / 32) * 2,
                    _ => 0,
                };

                let bg_screen_size = 2;
                let block_size = 32 * 32 * bg_screen_size;

                let block_address = base_screen_offset + block_id * block_size;
                let local_tile_id = tile_y * 32 + tile_x;
                let screen_block_offset = block_address + local_tile_id * bg_screen_size;
                let raw_bg_screen = self.vram.read_hword(screen_block_offset);

                PartialCharData {
                    x: ox % 8,
                    y: oy % 8,
                    transform: None,
                    color_mode: bgcnt.color_mode(),
                    bg_screen: BgScreen::text(raw_bg_screen),
                }
            }
            BgKind::Affine => {
                let (width, height) = screen_mode.affine_size();

                let bgtrans = match bg {
                    Background::Bg2 => &self.registers.bg2trans,
                    Background::Bg3 => &self.registers.bg3trans,
                    _ => unreachable!(),
                };

                let (ox, oy) = bgtrans.params.map(x.into(), y.into());

                let tile_x = (ox % width) / 8;
                let tile_y = (oy % height) / 8;

                let tile_map_index = tile_y * (width / 8) + tile_x;
                let screen_block_offset = base_screen_offset + tile_map_index as u32;
                let raw_bg_screen = self.vram.read_byte(screen_block_offset);

                PartialCharData {
                    x: ox % 8,
                    y: oy % 8,
                    transform: None,
                    color_mode: ColorMode::Palette256,
                    bg_screen: BgScreen::affine(raw_bg_screen),
                }
            }
        };

        let char_data = CharacterData {
            name: partial_data.bg_screen.character,
            base_offset: base_char_offset,
            hflip: partial_data.bg_screen.hflip,
            vflip: partial_data.bg_screen.vflip,
            color_mode: partial_data.color_mode,
            palette: partial_data.bg_screen.palette,
            kind: CharacterKind::Background,
            height: 8,
            width: 8,
            transform: partial_data.transform,
        };

        self.get_char_pixel(partial_data.x, partial_data.y, &char_data)
    }

    fn get_bg_bmp_pixel(
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

        let bgcnt = self.registers.bgcnt[2];
        let frame_buffer = self.registers.dispcnt.frame_buffer();
        let buffer_size = width * height * pixel_size;
        let buffer_start = frame_buffer as usize * pixel_size;
        let buffer_slice = &self.vram[buffer_start..buffer_start + buffer_size];
        let (tx, ty) = self.registers.bg2trans.params.map(x.into(), y.into());
        let idx = (ty as usize * width + tx as usize) * pixel_size;

        if (tx as usize >= width || ty as usize >= height) && !bgcnt.overflow_wrap() {
            return None;
        }

        let entry_lo = buffer_slice[idx % buffer_size];
        let entry_hi = buffer_slice[(idx + 1) % buffer_size];
        let entry = u16::from_le_bytes([entry_lo, entry_hi]);

        match color_mode {
            ColorSrc::RGB => Some(entry.into()),
            ColorSrc::Palette => Some(self.read_bg_palette(entry_lo)),
        }
    }
}

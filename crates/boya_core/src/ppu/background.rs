use crate::{
    bus::Bus,
    ppu::{
        PixelResult, Ppu, RenderingState,
        character::{CharacterData, CharacterKind},
        color::Color15,
        registers::{
            bldcnt::ColorFx,
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
    pub fn read_bg_palette(&self, id: u8) -> Color15 {
        self.palette.read_hword(id as u32 * 2).into()
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
                b_prio.cmp(&a_prio)
            }
        });
    }

    pub fn process_bg_pixel(
        &self,
        x: u16,
        y: u16,
        bg: Background,
        state: &mut RenderingState,
    ) -> Option<PixelResult> {
        if !state.flags.bg {
            return None;
        }

        let pixel = self.get_bg_pixel(x, y, bg)?;

        if state.flags.effects
            && self.registers.bldcnt.is_bg_second_target(bg)
            && state.target1.is_some()
        {
            state.target2 = Some(pixel);
            Some(PixelResult::Complete)
        } else if state.flags.effects && self.registers.bldcnt.is_bg_first_target(bg) {
            state.target1 = Some(pixel);

            match self.registers.bldcnt.color_effect() {
                ColorFx::AlphaBld => Some(PixelResult::Blend),
                _ => Some(PixelResult::Complete),
            }
        } else {
            state.target1 = Some(pixel);
            Some(PixelResult::Complete)
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
        let bgofs = self.registers.bgofs[bg_idx];

        let (width, height) = bgcnt.screen_mode().text_size();
        let base_screen_offset = bgcnt.screen_block_offset();
        let base_char_offset = bgcnt.char_block_offset();

        let ox = (x + bgofs.x) % width;
        let oy = (y + bgofs.y) % height;
        let char_x = ox % 8;
        let char_y = oy % 8;
        let screen_x = (ox / 8) as u32;
        let screen_y = (oy / 8) as u32;

        let block_x = screen_x / 32;
        let block_y = screen_y / 32;
        let tile_x = screen_x % 32;
        let tile_y = screen_y % 32;

        let block_id = match (bg_kind, width, height) {
            (BgKind::Text, 512, 256) => block_x,
            (BgKind::Text, 256, 512) => block_y,
            (BgKind::Text, 512, 512) => block_x + block_y * 2,
            _ => 0,
        };

        let block_address = base_screen_offset + block_id * 2048;
        let local_tile_id = tile_y * 32 + tile_x;
        let screen_block_offset = block_address + local_tile_id * 2;
        let raw_bg_screen = self.vram.read_hword(screen_block_offset);
        let bg_screen = BgScreen::from(raw_bg_screen);

        let transform = match (bg_kind, bg) {
            (BgKind::Affine, Background::Bg2) => Some(self.registers.bg2trans.params.clone()),
            (BgKind::Affine, Background::Bg3) => Some(self.registers.bg3trans.params.clone()),
            _ => None,
        };

        let char_data = CharacterData {
            name: bg_screen.character,
            base_offset: base_char_offset,
            hflip: bg_screen.hflip,
            vflip: bg_screen.vflip,
            color: bgcnt.color_mode(),
            palette: bg_screen.palette,
            kind: CharacterKind::Background,
            height: 8,
            width: 8,
            transform,
        };

        self.get_char_pixel(char_x, char_y, &char_data)
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

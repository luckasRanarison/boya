use crate::{
    bus::Bus,
    ppu::{
        Ppu, TransformParam,
        color::Color15,
        registers::{bgcnt::ColorMode, dispcnt::VramMapping},
    },
    utils::bitflags::Bitflag,
};

pub const TILE4BPP_SIZE: usize = 32;
pub const TILE8BPP_SIZE: usize = 64;

#[derive(Debug, Clone, Copy)]
pub enum CharacterKind {
    Background,
    Object(VramMapping),
}

#[derive(Debug)]
pub struct CharacterData {
    pub name: u16,
    pub base_offset: u32,
    pub width: u8,
    pub height: u8,
    pub hflip: bool,
    pub vflip: bool,
    pub transform: Option<TransformParam>,
    pub color: ColorMode,
    pub palette: u8,
    pub kind: CharacterKind,
}

impl Ppu {
    pub fn get_char_pixel(&self, x: u16, y: u16, char: CharacterData) -> Option<Color15> {
        let mut cx = x;
        let mut cy = y;

        if char.hflip {
            cx = char.width as u16 - cx;
        }

        if char.vflip {
            cy = char.height as u16 - cy;
        }

        if let Some(t) = &char.transform {
            let tx = t.pa as i32 * x as i32 + t.pb as i32 * cy as i32;
            let ty = t.pc as i32 * x as i32 + t.pd as i32 * cy as i32;
            cx = (tx >> 8) as u16;
            cy = (ty >> 8) as u16;
        }

        let pixel_addr = self.get_pixel_address(x, y, &char);
        let pixel_byte = self.vram.read_byte(pixel_addr);

        let mut base_palette = None;
        let mut rel_color_id = pixel_byte;

        if matches!(char.color, ColorMode::Palette16) {
            let (b_start, b_end) = if x & 1 == 0 { (0, 3) } else { (4, 7) };
            rel_color_id = pixel_byte.get_bits_u8(b_start, b_end);
            base_palette = Some(char.palette as u32 * 16);
        }

        if rel_color_id == 0 {
            return None;
        }

        let color_addr = base_palette.unwrap_or_default() + rel_color_id as u32;

        let color = match char.kind {
            CharacterKind::Background => self.read_bg_palette(color_addr),
            CharacterKind::Object(_) => self.read_obj_palette(color_addr),
        };

        Some(color)
    }

    fn get_pixel_address(&self, x: u16, y: u16, char: &CharacterData) -> u32 {
        let base_offset = match char.kind {
            CharacterKind::Background | CharacterKind::Object(VramMapping::Map1D) => {
                y as u32 * char.width as u32 + x as u32
            }
            CharacterKind::Object(VramMapping::Map2D) => y as u32 * 32 * 8 + x as u32,
        };

        let (tile_size, offset) = match char.color {
            ColorMode::Palette16 => (TILE4BPP_SIZE, base_offset / 2),
            ColorMode::Palette256 => (TILE8BPP_SIZE, base_offset),
        };

        char.base_offset + char.name as u32 * tile_size as u32 + offset
    }
}

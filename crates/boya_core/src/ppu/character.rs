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
    pub fn get_char_pixel(&self, x: u16, y: u16, char: &CharacterData) -> Option<Color15> {
        let width = char.width as u16;
        let height = char.height as u16;

        let (cx, cy) = if let Some(transform) = &char.transform {
            transform.map(x.into(), y.into())
        } else {
            let cx = if char.hflip { width - x - 1 } else { x };
            let cy = if char.vflip { height - y - 1 } else { y };
            (cx, cy)
        };

        if cx >= width || cy >= height {
            return None;
        }

        let pixel_addr = self.get_pixel_address(cx, cy, char);
        let pixel_byte = self.vram.read_byte(pixel_addr);

        let (base_palette, rel_color_id) = match char.color {
            ColorMode::Palette16 => {
                let (b_start, b_end) = if cx & 1 == 0 { (0, 3) } else { (4, 7) };
                let color_id = pixel_byte.get_bits(b_start, b_end);
                (char.palette, color_id)
            }
            ColorMode::Palette256 => (0, pixel_byte),
        };

        if rel_color_id == 0 {
            return None;
        }

        let color_id = base_palette * 16 + rel_color_id;

        let color = match char.kind {
            CharacterKind::Background => self.read_bg_palette(color_id),
            CharacterKind::Object(_) => self.read_obj_palette(color_id),
        };

        Some(color)
    }

    fn get_pixel_address(&self, x: u16, y: u16, char: &CharacterData) -> u32 {
        let tx = x / 8;
        let ty = y / 8;
        let px = x % 8;
        let py = y % 8;
        let base_pixel_offset = py * 8 + px;

        match char.kind {
            CharacterKind::Background | CharacterKind::Object(VramMapping::Map1D) => {
                let tiles_wide = char.width / 8;
                let tile_index = (ty * tiles_wide as u16) + tx;

                let (tile_size, pixel_offset) = match char.color {
                    ColorMode::Palette16 => (TILE4BPP_SIZE, base_pixel_offset / 2),
                    ColorMode::Palette256 => (TILE8BPP_SIZE, base_pixel_offset),
                };

                char.base_offset
                    + (char.name as u32 + tile_index as u32) * tile_size as u32
                    + pixel_offset as u32
            }
            CharacterKind::Object(VramMapping::Map2D) => {
                let (tx_offset, pixel_offset) = match char.color {
                    ColorMode::Palette16 => (tx, base_pixel_offset / 2),
                    ColorMode::Palette256 => (tx * 2, base_pixel_offset),
                };

                let tile_index = char.name as u32 + (ty as u32 * 32) + tx_offset as u32;

                char.base_offset + (tile_index * 32) + pixel_offset as u32
            }
        }
    }
}

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

#[derive(Debug)]
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

        if char.hflip {}

        if char.vflip {}
        // inp: 0 1 2 3 | 4 5 6 7
        // out: 7 6 5 4 | 3 2 1 0

        if let Some(t) = char.transform {
            let tx = t.pa as i32 * x as i32 + t.pb as i32 * y as i32;
            let ty = t.pc as i32 * x as i32 + t.pd as i32 * y as i32;
            cx = (tx >> 8) as u16;
            cy = (ty >> 8) as u16;
        }

        let (base_palette, rel_color_id) = match char.color {
            ColorMode::Palette16 => {
                let base_char_addr = char.base_offset + char.name as u32 * TILE4BPP_SIZE as u32;
                let base_palette = char.palette as u32 * 16;
                let pixel_addr = base_char_addr + ((y as u32 * 8 + x as u32) / 2);
                let pixels = self.vram.read_byte(pixel_addr);
                let (b_start, b_end) = if x & 1 == 0 { (0, 3) } else { (4, 7) };
                let color_id = pixels.get_bits_u8(b_start, b_end);
                (base_palette, color_id as u32)
            }
            ColorMode::Palette256 => todo!(),
        };

        if rel_color_id == 0 {
            return None;
        }

        let color_addr = base_palette + rel_color_id;

        let color = match char.kind {
            CharacterKind::Background => self.read_bg_palette(color_addr),
            CharacterKind::Object(_) => self.read_obj_palette(color_addr),
        };

        Some(color)
    }
}

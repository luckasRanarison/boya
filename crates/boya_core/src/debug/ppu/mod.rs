mod utils;

use crate::{
    debug::ppu::utils::FrameBuffer,
    ppu::{
        OBJ_COUNT, Ppu,
        character::{CharacterData, CharacterKind},
        color::Color15,
        object::Obj,
        registers::{bgcnt::ColorMode, dispcnt::Background},
    },
};

impl Ppu {
    pub fn color_palette(&self) -> Vec<Color15> {
        self.palette
            .chunks_exact(2)
            .map(|color| u16::from_le_bytes([color[0], color[1]]).into())
            .collect()
    }

    pub fn object_palette(&self, id: u8) -> Vec<Color15> {
        let mut buffer = Vec::new();
        let obj = self.get_object(id);

        match obj.color_mode() {
            ColorMode::Palette16 => {
                for i in 0..16 {
                    buffer.push(self.read_obj_palette(obj.palette() as u32 * 16 + i));
                }
            }
            ColorMode::Palette256 => {
                for i in 0..256 {
                    buffer.push(self.read_obj_palette(i));
                }
            }
        }

        buffer
    }

    pub fn objects(&self) -> Vec<Obj> {
        let mut objects = Vec::new();

        for i in 0..OBJ_COUNT {
            objects.push(self.get_object(i));
        }

        objects
    }

    pub fn render_tile(&self, id: u16, offset: u32, color: ColorMode, palette_id: u8) -> Vec<u8> {
        let mut buffer = FrameBuffer::default();

        let char_data = CharacterData {
            name: id,
            base_offset: offset,
            hflip: false,
            vflip: false,
            color,
            palette: palette_id,
            kind: CharacterKind::Background,
            height: 8,
            width: 8,
            transform: None,
        };

        for y in 0..8 {
            for x in 0..8 {
                let pixel = self.get_char_pixel(x, y, &char_data).unwrap_or_default();
                buffer.push(pixel);
            }
        }

        buffer.into()
    }

    pub fn render_bg(&self, bg: Background) -> Vec<u8> {
        let mut buffer = FrameBuffer::default();

        for y in 0..160 {
            for x in 0..240 {
                let pixel = self.get_bg_pixel(x, y, bg).unwrap_or_default();
                buffer.push(pixel);
            }
        }

        buffer.into()
    }

    pub fn render_obj(&self, id: u8) -> Vec<u8> {
        let mut buffer = FrameBuffer::default();
        let obj = self.get_object(id);
        let (width, height) = obj.dimmensions();

        for y in 0..width as u16 {
            for x in 0..height as u16 {
                let pixel = self.get_obj_pixel_inner(x, y, &obj).unwrap_or_default();
                buffer.push(pixel);
            }
        }

        buffer.into()
    }
}

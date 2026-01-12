use crate::ppu::{Ppu, color::Color15};

impl Ppu {
    pub fn color_palette(&self) -> Vec<Color15> {
        self.palette
            .chunks_exact(2)
            .map(|color| u16::from_le_bytes([color[0], color[1]]).into())
            .collect()
    }
}

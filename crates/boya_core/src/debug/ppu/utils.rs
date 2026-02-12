use crate::ppu::color::{Color15, Color24};

#[derive(Debug, Default)]
pub struct FrameBuffer {
    data: Vec<u8>,
}

impl FrameBuffer {
    pub fn push(&mut self, value: Option<Color15>) {
        let color = if let Some(pixel) = value {
            let Color24 { r, g, b } = pixel.into();
            [r, g, b, 0xFF]
        } else {
            [0, 0, 0, 0x00]
        };

        self.data.extend(color);
    }

    pub fn into_data(self) -> Vec<u8> {
        self.data
    }
}

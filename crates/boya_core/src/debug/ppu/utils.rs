use crate::ppu::color::{Color15, Color24};

#[derive(Debug, Default)]
pub struct FrameBuffer {
    items: Vec<Color15>,
}

impl FrameBuffer {
    pub fn push(&mut self, value: Color15) {
        self.items.push(value);
    }
}

impl From<FrameBuffer> for Vec<u8> {
    fn from(value: FrameBuffer) -> Self {
        let mut buffer = Vec::new();

        for pixel in value.items {
            let color24 = Color24::from(pixel);

            buffer.push(color24.r);
            buffer.push(color24.g);
            buffer.push(color24.b);
            buffer.push(0xFF);
        }

        buffer
    }
}

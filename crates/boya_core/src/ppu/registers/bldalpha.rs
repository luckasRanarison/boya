use crate::{ppu::color::Color15, utils::bitflags::Bitflag};

#[derive(Debug, Default)]
pub struct Bldalpha {
    pub value: u16,
}

impl Bldalpha {
    fn eva(&self) -> u16 {
        u16::min(16, self.value.get_bits(0, 4))
    }

    fn evb(&self) -> u16 {
        u16::min(16, self.value.get_bits(8, 12))
    }

    #[inline(always)]
    fn get_intensity(&self, ia: u8, ib: u8) -> u16 {
        u16::min(31, (ia as u16 * self.eva() + ib as u16 * self.evb()) >> 4)
    }

    pub fn blend(&self, color_a: Color15, color_b: Color15) -> Color15 {
        let r = self.get_intensity(color_a.r, color_b.r);
        let g = self.get_intensity(color_a.g, color_b.g);
        let b = self.get_intensity(color_a.b, color_b.b);

        Color15::new(r, g, b)
    }
}

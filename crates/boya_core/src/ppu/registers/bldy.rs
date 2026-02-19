use crate::{ppu::color::Color15, utils::bitflags::Bitflag};

#[derive(Debug, Default)]
pub struct Bldy {
    pub value: u16,
}

impl Bldy {
    fn evy(&self) -> u16 {
        u16::min(16, self.value.get_bits(0, 4))
    }

    #[inline(always)]
    fn get_bright_intensity(&self, ia: u8) -> u16 {
        u16::min(31, ia as u16 + ((31 - ia as u16) * self.evy()) >> 4)
    }

    #[inline(always)]
    fn get_dark_intensity(&self, ia: u8) -> u16 {
        u16::min(31, ia as u16 - ((ia as u16 * self.evy()) >> 4))
    }

    pub fn brighten(&self, color: Color15) -> Color15 {
        let r = self.get_bright_intensity(color.r);
        let g = self.get_bright_intensity(color.g);
        let b = self.get_bright_intensity(color.b);

        Color15::new(r, g, b)
    }

    pub fn darken(&self, color: Color15) -> Color15 {
        let r = self.get_dark_intensity(color.r);
        let g = self.get_dark_intensity(color.g);
        let b = self.get_dark_intensity(color.b);

        Color15::new(r, g, b)
    }
}

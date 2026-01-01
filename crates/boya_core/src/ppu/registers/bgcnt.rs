use crate::utils::bitflags::Bitflag;

#[derive(Debug, Default)]
pub struct BgCnt {
    pub value: u16,
}

impl BgCnt {
    pub fn bg_priority(&self) -> u8 {
        self.value.get_bits_u8(0, 1)
    }

    pub fn char_base_block(&self) -> u8 {
        self.value.get_bits_u8(2, 3)
    }

    pub fn mosaic_enabled(&self) -> bool {
        self.value.has(6)
    }

    pub fn color_mode(&self) -> ColorMode {
        match self.value.get(7) {
            0 => ColorMode::Palette16,
            _ => ColorMode::Palette256,
        }
    }

    pub fn screen_base_block(&self) -> u8 {
        self.value.get_bits_u8(8, 12)
    }

    pub fn screen_size(&self) -> ScreenSize {
        match self.value.get_bits(14, 15) {
            0 => ScreenSize::Mode0,
            1 => ScreenSize::Mode1,
            2 => ScreenSize::Mode2,
            _ => ScreenSize::Mode3,
        }
    }
}

#[derive(Debug)]
pub enum ColorMode {
    Palette16,
    Palette256,
}

#[derive(Debug)]
pub enum ScreenSize {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}

use crate::utils::bitflags::Bitflag;

#[derive(Debug, Default, Clone, Copy)]
pub struct Bgcnt {
    pub value: u16,
}

impl Bgcnt {
    pub fn bg_priority(&self) -> u8 {
        self.value.get_bits_u8(0, 1)
    }

    pub fn char_block_offset(&self) -> u32 {
        self.value.get_bits(2, 3) as u32 * 0x4000
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

    pub fn screen_block_offset(&self) -> u32 {
        self.value.get_bits(8, 12) as u32 * 0x400
    }

    pub fn screen_mode(&self) -> ScreenSize {
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

#[derive(Debug, Clone, Copy)]
pub enum ScreenSize {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}

impl ScreenSize {
    pub fn text_size(self) -> (u16, u16) {
        match self {
            ScreenSize::Mode0 => (256, 256),
            ScreenSize::Mode1 => (512, 256),
            ScreenSize::Mode2 => (256, 512),
            ScreenSize::Mode3 => (512, 512),
        }
    }

    pub fn alt_size(self) -> (u16, u16) {
        match self {
            ScreenSize::Mode0 => (128, 128),
            ScreenSize::Mode1 => (256, 256),
            ScreenSize::Mode2 => (512, 512),
            ScreenSize::Mode3 => (1024, 1024),
        }
    }
}

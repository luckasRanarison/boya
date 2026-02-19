use crate::{ppu::registers::dispcnt::Background, utils::bitflags::Bitflag};

#[derive(Debug, Default)]
pub struct Bldcnt {
    pub value: u16,
}

impl Bldcnt {
    pub fn is_bg_first_target(&self, bg: Background) -> bool {
        self.value.has(bg as u16)
    }

    pub fn is_obj_first_target(&self) -> bool {
        self.value.has(4)
    }

    pub fn is_bd_first_target(&self) -> bool {
        self.value.has(5)
    }

    pub fn color_effect(&self) -> ColorFx {
        match self.value.get_bits(6, 7) {
            1 => ColorFx::AlphaBld,
            2 => ColorFx::BrightnessInc,
            3 => ColorFx::BrightnessDec,
            _ => ColorFx::None,
        }
    }

    pub fn is_bg_second_target(&self, bg: Background) -> bool {
        self.value.has(bg as u16 + 8)
    }

    pub fn is_obj_second_target(&self) -> bool {
        self.value.has(12)
    }

    pub fn is_bd_second_traget(&self) -> bool {
        self.value.has(13)
    }
}

#[derive(Debug)]
pub enum ColorFx {
    None,
    AlphaBld,
    BrightnessInc,
    BrightnessDec,
}

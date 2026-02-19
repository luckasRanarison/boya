use crate::{ppu::registers::dispcnt::Background, utils::bitflags::Bitflag};

#[derive(Debug, Default)]
pub struct WinH {
    pub x1: u8,
    pub x2: u8,
}

#[derive(Debug, Default)]
pub struct WinV {
    pub y1: u8,
    pub y2: u8,
}

#[derive(Debug, Default)]
pub struct Winin {
    pub value: u16,
}

impl Winin {
    pub fn bg_enable(&self, win: usize, bg: Background) -> bool {
        self.value.has((win * 8) as u16 + (bg as u16))
    }

    pub fn obj_enable(&self, win: usize) -> bool {
        self.value.has((win * 8) as u16 + 4)
    }

    pub fn color_fx_enable(&self, win: usize) -> bool {
        self.value.has((win * 8) as u16 + 5)
    }
}

#[derive(Debug, Default)]
pub struct Winout {
    pub value: u16,
}

impl Winout {
    pub fn outside_bg_enable(&self, bg: Background) -> bool {
        self.value.has(bg as u16)
    }

    pub fn outside_obj_enable(&self) -> bool {
        self.value.has(4)
    }

    pub fn outside_color_fx(&self) -> bool {
        self.value.has(5)
    }

    pub fn obj_win_bg_enable(&self, bg: Background) -> bool {
        self.value.has(8 + bg as u16)
    }

    pub fn obj_win_obj_enable(&self) -> bool {
        self.value.has(12)
    }

    pub fn obj_win_color_fx(&self) -> bool {
        self.value.has(13)
    }
}

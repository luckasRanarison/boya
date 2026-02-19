use crate::{ppu::registers::window::Window, utils::bitflags::Bitflag};

#[derive(Debug, Default)]
pub struct Dispcnt {
    pub value: u16,
}

// TODO: CGB mode (bit 3)
impl Dispcnt {
    pub fn bg_mode(&self) -> BgMode {
        match self.value.get_bits(0, 2) {
            0 => BgMode::Mode0,
            1 => BgMode::Mode1,
            2 => BgMode::Mode2,
            3 => BgMode::Mode3,
            4 => BgMode::Mode4,
            _ => BgMode::Mode5,
        }
    }

    pub fn frame_buffer(&self) -> FrameBuffer {
        match self.value.get(4) {
            0 => FrameBuffer::Buffer0,
            _ => FrameBuffer::Buffer1,
        }
    }

    pub fn hblank_obj_proc(&self) -> bool {
        self.value.has(5)
    }

    pub fn obj_vram_mapping(&self) -> VramMapping {
        match self.value.get(6) {
            0 => VramMapping::Map2D,
            _ => VramMapping::Map1D,
        }
    }

    pub fn forced_blank(&self) -> bool {
        self.value.has(7)
    }

    pub fn bg_enable(&self, bg: Background) -> bool {
        match bg {
            Background::Bg0 => self.value.has(8),
            Background::Bg1 => self.value.has(9),
            Background::Bg2 => self.value.has(10),
            Background::Bg3 => self.value.has(11),
        }
    }

    pub fn obj_enable(&self) -> bool {
        self.value.has(12)
    }

    pub fn win_enable(&self, win: Window) -> bool {
        match win {
            Window::Win0 => self.value.has(13),
            Window::Win1 => self.value.has(14),
        }
    }
}

#[derive(Debug)]
pub enum FrameBuffer {
    Buffer0,
    Buffer1,
}

#[derive(Debug, Clone, Copy)]
pub enum BgMode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
    Mode4,
    Mode5,
}

impl BgMode {
    pub fn is_tile(self) -> bool {
        matches!(self, BgMode::Mode0 | BgMode::Mode1 | BgMode::Mode2)
    }

    pub fn is_bitmap(self) -> bool {
        matches!(self, BgMode::Mode3 | BgMode::Mode4 | BgMode::Mode5)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VramMapping {
    Map1D,
    Map2D,
}

#[derive(Debug, Clone, Copy)]
pub enum Background {
    Bg0,
    Bg1,
    Bg2,
    Bg3,
}

impl From<TransBackground> for Background {
    fn from(value: TransBackground) -> Self {
        match value {
            TransBackground::Bg2 => Background::Bg2,
            TransBackground::Bg3 => Background::Bg3,
        }
    }
}

#[derive(Debug)]
pub enum TransBackground {
    Bg2,
    Bg3,
}

impl Background {
    #[inline(always)]
    pub fn to_index(self) -> usize {
        self as usize
    }
}

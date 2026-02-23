use crate::{ppu::registers::window::Window, utils::bitflags::Bitflag};

#[derive(Debug, Default, Clone, Copy)]
pub struct Color15 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color15 {
    pub fn new(r: u16, g: u16, b: u16) -> Self {
        Self::from(r | (g << 5) | (b << 10))
    }
}

impl From<u16> for Color15 {
    fn from(color: u16) -> Self {
        let r = color.get_bits_u8(0, 4);
        let g = color.get_bits_u8(5, 9);
        let b = color.get_bits_u8(10, 14);

        Self { r, g, b }
    }
}

impl From<Color15> for u16 {
    fn from(value: Color15) -> u16 {
        ((value.r as u16) << 10) | ((value.g as u16) << 5) | value.b as u16
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Color24 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Color15> for Color24 {
    fn from(value: Color15) -> Self {
        let r = (value.r << 3) | (value.r >> 2);
        let g = (value.g << 3) | (value.g >> 2);
        let b = (value.b << 3) | (value.b >> 2);

        Self { r, g, b }
    }
}

impl From<Color24> for u32 {
    fn from(value: Color24) -> u32 {
        ((value.r as u32) << 16) | ((value.g as u32) << 8) | value.b as u32
    }
}

#[derive(Debug, Default)]
pub struct PixelContext {
    pub window: Option<Window>,
    pub acc: PixelAccumulator,
}

#[derive(Debug, Default)]
pub struct PixelAccumulator {
    pub top: Option<Color15>,
    pub bottom: Option<Color15>,
    pub blend: bool,
}

impl PixelAccumulator {
    pub fn is_done(&self) -> bool {
        self.top.is_some() && (self.bottom.is_some() || !self.blend)
    }
}

#[derive(Debug)]
pub enum PixelResult {
    Top(Color15),
    BlendTop(Color15),
    Bottom(Color15),
    Window,
}

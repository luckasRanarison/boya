use boya_core::ppu::registers::bgcnt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum ColorMode {
    Palette16,
    Palette256,
}

impl From<ColorMode> for bgcnt::ColorMode {
    fn from(value: ColorMode) -> Self {
        match value {
            ColorMode::Palette16 => bgcnt::ColorMode::Palette16,
            ColorMode::Palette256 => bgcnt::ColorMode::Palette256,
        }
    }
}

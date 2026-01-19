use boya_core::ppu::registers::bgcnt;
use serde::Serialize;
use tsify::Tsify;
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

#[derive(Tsify, Serialize)]
pub enum RegisterSize {
    HWord,
    Word,
}

#[derive(Tsify, Serialize)]
pub struct RegisterEntry {
    pub name: &'static str,
    pub address: u32,
    pub size: RegisterSize,
    pub flags: Vec<Flag>,
}

#[derive(Tsify, Serialize)]
pub struct Flag {
    pub name: &'static str,
    pub start: u8,
    pub end: Option<u8>,
}

impl Flag {
    pub fn new(name: &'static str, start: u8, end: Option<u8>) -> Self {
        Self { name, start, end }
    }
}

#[derive(Tsify, Serialize)]
pub struct IOMap(pub Vec<RegisterEntry>);

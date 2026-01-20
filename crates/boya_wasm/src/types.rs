use boya_core::{
    bus::{self, debug::io_map::IO_MAP},
    ppu,
};
use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum ColorMode {
    Palette16,
    Palette256,
}

impl From<ColorMode> for ppu::registers::bgcnt::ColorMode {
    fn from(value: ColorMode) -> Self {
        match value {
            ColorMode::Palette16 => ppu::registers::bgcnt::ColorMode::Palette16,
            ColorMode::Palette256 => ppu::registers::bgcnt::ColorMode::Palette256,
        }
    }
}

#[derive(Tsify, Serialize)]
pub enum RegisterSize {
    HWord,
    Word,
}

impl From<bus::debug::types::RegisterSize> for RegisterSize {
    fn from(value: bus::debug::types::RegisterSize) -> Self {
        match value {
            bus::debug::types::RegisterSize::HWord => RegisterSize::HWord,
            bus::debug::types::RegisterSize::Word => RegisterSize::Word,
        }
    }
}

#[derive(Tsify, Serialize)]
pub struct RegisterEntry {
    pub name: &'static str,
    pub address: u32,
    pub size: RegisterSize,
    pub flags: Vec<Flag>,
}

impl From<&bus::debug::types::RegisterEntry> for RegisterEntry {
    fn from(value: &bus::debug::types::RegisterEntry) -> Self {
        Self {
            name: value.name,
            address: value.address,
            size: value.size.into(),
            flags: value.flags.iter().rev().map(|f| f.into()).collect(),
        }
    }
}

#[derive(Tsify, Serialize)]
pub struct Flag {
    pub name: &'static str,
    pub start: u8,
    pub length: u8,
    pub mappings: Option<Vec<&'static str>>,
}

impl From<&bus::debug::types::Flag> for Flag {
    fn from(value: &bus::debug::types::Flag) -> Self {
        Self {
            name: value.name,
            start: value.start,
            length: value.length,
            mappings: value
                .mappings
                .map(|ms| ms.iter().map(|(_, v)| *v).collect()),
        }
    }
}

#[derive(Tsify, Serialize)]
pub struct IOMap(pub Vec<RegisterEntry>);

impl Default for IOMap {
    fn default() -> Self {
        Self(IO_MAP.iter().map(|r| r.into()).collect())
    }
}

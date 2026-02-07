use boya_core::{
    bus::{self, debug::io_map::IO_MAP},
    ppu,
};
use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum MemoryRegion {
    BIOS,
    EWRAM,
    IWRAM,
    IO,
    ROM,
    PALETTE,
    VRAM,
    OAM,
    SRAM,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
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

impl From<ppu::registers::bgcnt::ColorMode> for ColorMode {
    fn from(value: ppu::registers::bgcnt::ColorMode) -> Self {
        match value {
            ppu::registers::bgcnt::ColorMode::Palette16 => ColorMode::Palette16,
            ppu::registers::bgcnt::ColorMode::Palette256 => ColorMode::Palette256,
        }
    }
}

#[derive(Tsify, Serialize)]
pub enum RegisterSize {
    Byte,
    HWord,
    Word,
}

impl From<bus::debug::types::RegisterSize> for RegisterSize {
    fn from(value: bus::debug::types::RegisterSize) -> Self {
        match value {
            bus::debug::types::RegisterSize::Byte => RegisterSize::Byte,
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

#[wasm_bindgen]
pub struct Obj {
    pub x: u16,
    pub y: u16,
    pub width: u8,
    pub height: u8,
    pub priority: u8,
    pub palette: u8,
    pub character: u16,
    pub color_mode: ColorMode,
    pub transform: bool,
    pub hflip: bool,
    pub vflip: bool,
    pub mosaic: bool,
    pub double_size: bool,
}

impl From<ppu::object::Obj> for Obj {
    fn from(value: ppu::object::Obj) -> Self {
        let (width, height) = value.dimmensions();

        Self {
            x: value.x(),
            y: value.y(),
            priority: value.bg_priority(),
            palette: value.palette(),
            character: value.character(),
            color_mode: value.color_mode().into(),
            transform: value.transform(),
            hflip: value.hflip(),
            vflip: value.vflip(),
            mosaic: value.mosaic(),
            double_size: value.double_size(),
            width,
            height,
        }
    }
}

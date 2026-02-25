use boya_core::{
    debug::{self, bus::registers::IO_REGISTERS, cpu::types::InstructionData},
    ppu::{
        self,
        registers::{bgcnt, dispcnt},
    },
};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi)]
pub struct Instruction {
    pub address: u32,
    pub value: String,
}

impl Instruction {
    pub fn new(address: u32, data: InstructionData) -> Self {
        Self {
            address,
            value: data.format(10), // TODO: structured data?
        }
    }
}

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
pub enum Background {
    BG0,
    BG1,
    BG2,
    BG3,
}

impl From<Background> for dispcnt::Background {
    fn from(value: Background) -> Self {
        match value {
            Background::BG0 => dispcnt::Background::Bg0,
            Background::BG1 => dispcnt::Background::Bg1,
            Background::BG2 => dispcnt::Background::Bg2,
            Background::BG3 => dispcnt::Background::Bg3,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
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

impl From<bgcnt::ColorMode> for ColorMode {
    fn from(value: bgcnt::ColorMode) -> Self {
        match value {
            bgcnt::ColorMode::Palette16 => ColorMode::Palette16,
            bgcnt::ColorMode::Palette256 => ColorMode::Palette256,
        }
    }
}

#[derive(Tsify, Serialize)]
pub enum RegisterSize {
    Byte,
    HWord,
    Word,
}

impl From<debug::bus::registers::RegisterSize> for RegisterSize {
    fn from(value: debug::bus::registers::RegisterSize) -> Self {
        match value {
            debug::bus::registers::RegisterSize::Byte => RegisterSize::Byte,
            debug::bus::registers::RegisterSize::HWord => RegisterSize::HWord,
            debug::bus::registers::RegisterSize::Word => RegisterSize::Word,
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

impl From<&debug::bus::registers::RegisterEntry> for RegisterEntry {
    fn from(value: &debug::bus::registers::RegisterEntry) -> Self {
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

impl From<&debug::bus::registers::Flag> for Flag {
    fn from(value: &debug::bus::registers::Flag) -> Self {
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
#[tsify(into_wasm_abi)]
pub struct IOMap(pub Vec<RegisterEntry>);

impl Default for IOMap {
    fn default() -> Self {
        Self(IO_REGISTERS.iter().map(|r| r.into()).collect())
    }
}

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub enum ObjMode {
    Normal,
    SemiTransparent,
    Window,
}

impl From<ppu::object::ObjMode> for ObjMode {
    fn from(value: ppu::object::ObjMode) -> Self {
        match value {
            ppu::object::ObjMode::Normal => ObjMode::Normal,
            ppu::object::ObjMode::SemiTransparent => ObjMode::SemiTransparent,
            ppu::object::ObjMode::Window => ObjMode::Window,
        }
    }
}

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct Obj {
    pub x: u16,
    pub y: u8,
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
    pub mode: ObjMode,
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
            mode: value.mode().into(),
            width,
            height,
        }
    }
}

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct CartridgeHeader {
    pub entry_point: u32,
    pub title: String,
    pub game_code: String,
    pub maker_code: String,
    pub software_version: u8,
    pub checksum: u8,
}

impl From<boya_core::rom::CartridgeHeader> for CartridgeHeader {
    fn from(value: boya_core::rom::CartridgeHeader) -> Self {
        Self {
            entry_point: value.entry_point,
            title: String::from_utf8_lossy(&value.title).to_string(),
            game_code: String::from_utf8_lossy(&value.game_code).to_string(),
            maker_code: String::from_utf8_lossy(&value.maker_code).to_string(),
            software_version: value.software_version,
            checksum: value.checksum,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub enum Hook {
    Breakpoints(Vec<u32>),
    Irq(bool),
}

impl From<Hook> for debug::Hook {
    fn from(value: Hook) -> Self {
        match value {
            Hook::Breakpoints(bp) => debug::Hook::Breakpoints(bp),
            Hook::Irq(value) => debug::Hook::Irq(value),
        }
    }
}

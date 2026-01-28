pub mod types;

use boya_core::{Gba as GbaCore, bus::Bus, ppu::color::Color24, utils::Reset};
use wasm_bindgen::prelude::*;
use web_sys::js_sys::{Uint8Array, Uint32Array};

use crate::types::{ColorMode, IOMap};

#[wasm_bindgen]
#[derive(Default)]
pub struct Gba {
    core: GbaCore,
}

#[wasm_bindgen]
impl Gba {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[wasm_bindgen(js_name = "loadBios")]
    pub fn load_bios(&mut self, bios: &[u8]) -> Result<(), JsError> {
        self.core.load_bios(bios.try_into()?);
        Ok(())
    }

    #[wasm_bindgen(js_name = "loadRom")]
    pub fn load_rom(&mut self, rom: &[u8]) {
        self.core.load_rom(rom);
    }

    #[wasm_bindgen]
    pub fn boot(&mut self) {
        self.core.boot();
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.core.reset();
    }

    #[wasm_bindgen(js_name = "debugSyncedStep")]
    pub fn debug_synced_step(&mut self) -> u32 {
        self.core.debug_step().cycles().count()
    }

    #[wasm_bindgen(js_name = "stepFrameWithHooks")]
    pub fn step_frame_with_hooks(&mut self, breakpoints: &[u32], irq: bool) -> bool {
        self.core.step_frame_with_hook(breakpoints, irq)
    }

    #[wasm_bindgen(js_name = "stepScanline")]
    pub fn step_scanline(&mut self) {
        self.core.step_scanline();
    }

    #[wasm_bindgen]
    pub fn cycles(&mut self) -> u64 {
        self.core.cycles
    }

    #[wasm_bindgen(js_name = "execAddress")]
    pub fn exec_address(&self) -> u32 {
        self.core.cpu.exec_address()
    }

    #[wasm_bindgen(js_name = "lr")]
    pub fn lr(&self) -> u32 {
        self.core.cpu.lr()
    }

    #[wasm_bindgen(js_name = "sp")]
    pub fn sp(&self) -> u32 {
        self.core.cpu.lr()
    }

    #[wasm_bindgen]
    pub fn cpsr(&self) -> u32 {
        self.core.cpu.cpsr.value()
    }

    #[wasm_bindgen(js_name = "currentInstruction")]
    pub fn current_instruction(&self) -> Option<String> {
        let pipeline = &self.core.cpu.pipeline;
        let instruction = pipeline.current_instruction();
        instruction.map(|instr| instr.get_data().format(10))
    }

    #[wasm_bindgen(js_name = "nextInstructions")]
    pub fn next_instructions(&self, max_length: u16) -> Result<JsValue, JsValue> {
        let instructions = self
            .core
            .cpu
            .decode_until_branch(max_length)
            .into_iter()
            .map(|(addr, instr)| (addr, instr.format(10)))
            .collect::<Vec<_>>();

        Ok(serde_wasm_bindgen::to_value(&instructions)?)
    }

    #[wasm_bindgen(js_name = "instructionSize")]
    pub fn instruction_size(&self) -> u8 {
        self.core.cpu.instr_size()
    }

    #[wasm_bindgen(js_name = "startingSubroutine")]
    pub fn starting_subroutine(&self) -> bool {
        self.core.cpu.starting_subroutine()
    }

    #[wasm_bindgen(js_name = "stepFrame")]
    pub fn step_frame(&mut self) {
        self.core.step_frame();
    }

    #[wasm_bindgen(js_name = "writeFrameBuffer")]
    pub fn write_frame_buffer(&self, image_data: &mut [u8]) {
        image_data.copy_from_slice(self.core.frame_buffer());
    }

    #[wasm_bindgen(js_name = "setKeyinput")]
    pub fn set_keyinput(&mut self, value: u16) {
        self.core.set_keyinput(value);
    }

    #[wasm_bindgen]
    pub fn bios(&self) -> Uint8Array {
        unsafe { Uint8Array::view(self.core.bios()) }
    }

    #[wasm_bindgen]
    pub fn ewram(&self) -> Uint8Array {
        unsafe { Uint8Array::view(self.core.ewram()) }
    }

    #[wasm_bindgen]
    pub fn iwram(&self) -> Uint8Array {
        unsafe { Uint8Array::view(self.core.iwram()) }
    }

    #[wasm_bindgen]
    pub fn palette(&self) -> Uint8Array {
        unsafe { Uint8Array::view(self.core.palette()) }
    }

    #[wasm_bindgen]
    pub fn vram(&self) -> Uint8Array {
        unsafe { Uint8Array::view(self.core.vram()) }
    }

    #[wasm_bindgen]
    pub fn oam(&self) -> Uint8Array {
        unsafe { Uint8Array::view(self.core.oam()) }
    }

    #[wasm_bindgen]
    pub fn rom(&self) -> Uint8Array {
        unsafe { Uint8Array::view(self.core.rom()) }
    }

    #[wasm_bindgen]
    pub fn sram(&self) -> Uint8Array {
        unsafe { Uint8Array::view(self.core.sram()) }
    }

    #[wasm_bindgen(js_name = "getMainRegisters")]
    pub fn get_main_registers(&self) -> Uint32Array {
        unsafe { Uint32Array::view(&self.core.cpu.registers.main) }
    }

    #[wasm_bindgen(js_name = "getFiqRegisters")]
    pub fn get_fiq_registers(&self) -> Uint32Array {
        unsafe { Uint32Array::view(&self.core.cpu.registers.fiq) }
    }

    #[wasm_bindgen(js_name = "getSvcRegisters")]
    pub fn get_svc_registers(&self) -> Uint32Array {
        unsafe { Uint32Array::view(&self.core.cpu.registers.svc) }
    }

    #[wasm_bindgen(js_name = "getAbtRegisters")]
    pub fn get_abt_registers(&self) -> Uint32Array {
        unsafe { Uint32Array::view(&self.core.cpu.registers.abt) }
    }

    #[wasm_bindgen(js_name = "getIrqRegisters")]
    pub fn get_irq_registers(&self) -> Uint32Array {
        unsafe { Uint32Array::view(&self.core.cpu.registers.irq) }
    }

    #[wasm_bindgen(js_name = "getUndRegisters")]
    pub fn get_und_registers(&self) -> Uint32Array {
        unsafe { Uint32Array::view(&self.core.cpu.registers.und) }
    }

    #[wasm_bindgen(js_name = "getSpsrBank")]
    pub fn get_spsr_bank(&self) -> Vec<u32> {
        self.core.cpu.registers.psr.map(|psr| psr.value()).to_vec()
    }

    #[wasm_bindgen(js_name = "cpuOperatingMode")]
    pub fn cpu_operating_mode(&self) -> String {
        format!("{:?}", self.core.cpu.cpsr.op_mode())
    }

    #[wasm_bindgen(js_name = "colorPalette")]
    pub fn color_palette(&self) -> Vec<u32> {
        self.core
            .color_palette()
            .into_iter()
            .map(|c| Color24::from(c).into())
            .collect()
    }

    #[wasm_bindgen(js_name = "writeTileBuffer")]
    pub fn write_tile_buffer(
        &self,
        image_data: &mut [u8],
        tile: &[u8],
        color_mode: ColorMode,
        palette_id: usize,
    ) {
        let buffer = self.core.render_tile(tile, color_mode.into(), palette_id);
        image_data.copy_from_slice(buffer.as_slice());
    }

    #[wasm_bindgen(js_name = "generateIOMap")]
    pub fn generate_io_map(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&IOMap::default())?)
    }

    #[wasm_bindgen(js_name = "writeByte")]
    pub fn write_byte(&mut self, address: u32, value: u8) {
        self.core.cpu.bus.write_byte(address, value);
    }

    #[wasm_bindgen(js_name = "peekHWord")]
    pub fn peek_hword(&self, address: u32) -> u16 {
        self.core.cpu.bus.peek_hword(address)
    }

    #[wasm_bindgen(js_name = "peekWord")]
    pub fn peek_word(&self, address: u32) -> u32 {
        self.core.cpu.bus.peek_word(address)
    }
}

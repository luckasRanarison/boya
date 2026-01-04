use boya_core::Gba as GbaCore;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::{Uint8Array, Uint32Array};

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

    #[wasm_bindgen(js_name = "syncedStep")]
    pub fn synced_step(&mut self) {
        self.core.synced_step();
    }

    #[wasm_bindgen]
    pub fn cycles(&mut self) -> u64 {
        self.core.cycles
    }

    #[wasm_bindgen(js_name = "executionAddress")]
    pub fn execution_address(&self) -> u32 {
        self.core.cpu.pipeline.current_address()
    }

    #[wasm_bindgen(js_name = "currentInstruction")]
    pub fn current_instruction(&self) -> String {
        let pipeline = &self.core.cpu.pipeline;
        let instruction = pipeline.current_instruction().unwrap();
        instruction.get_data().format(10)
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

    #[wasm_bindgen]
    pub fn cpsr(&self) -> u32 {
        self.core.cpu.cpsr.value()
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

    #[wasm_bindgen(js_name = "getBankedPsr")]
    pub fn get_banked_psr(&self) -> Vec<u32> {
        self.core.cpu.registers.psr.map(|psr| psr.value()).to_vec()
    }
}

use crate::{
    bus::{BIOS_SIZE, types::Cycle},
    cpu::{
        Arm7tdmi,
        common::Exception,
        debug::types::{GbaStep, GbaStepKind},
    },
    ppu::{character::TILE_BUFFER_SIZE, color::Color15, object::Obj, registers::bgcnt::ColorMode},
    utils::Reset,
};

#[allow(clippy::all)]
pub mod apu; // TODO: APU implmentation
pub mod bus;
pub mod cpu;
#[allow(unused)]
pub mod ppu;
pub mod utils;

#[cfg(test)]
mod test;

#[derive(Default)]
pub struct Gba {
    pub cpu: Arm7tdmi,
    pub cycles: u64,
}

impl Gba {
    pub fn load_bios(&mut self, bios: [u8; BIOS_SIZE]) {
        self.cpu.bus.bios = bios;
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.cpu.bus.rom = rom.to_vec();
    }

    pub fn boot(&mut self) -> Cycle {
        self.cpu.handle_exception(Exception::Reset)
    }

    pub fn step(&mut self) {
        let cycles = self
            .cpu
            .try_irq()
            .map(|irq| irq.cycles)
            .or_else(|| self.cpu.bus.try_dma().map(|dma| dma.cycles))
            .unwrap_or_else(|| self.cpu.step());

        self.sync(cycles);
    }

    pub fn debug_step(&mut self) -> GbaStep {
        let value = self
            .cpu
            .try_irq()
            .map(GbaStepKind::Interrupt)
            .or_else(|| self.cpu.bus.try_dma().map(GbaStepKind::Dma))
            .unwrap_or_else(|| GbaStepKind::Instruction(self.cpu.debug_step()));

        let step = GbaStep { value };
        let cycles = step.cycles();

        self.sync(cycles);

        step
    }

    pub fn step_frame(&mut self) {
        self.step_visible_frame();
        self.step_vblank();
    }

    pub fn step_visible_frame(&mut self) {
        while self.is_rendering() {
            self.step();
        }
    }

    pub fn step_vblank(&mut self) {
        while !self.is_rendering() {
            self.step();
        }
    }

    #[inline(always)]
    pub fn is_rendering(&self) -> bool {
        self.cpu.bus.ppu.is_rendering()
    }

    pub fn frame_buffer(&self) -> &[u8] {
        self.cpu.bus.ppu.get_frame_buffer()
    }

    pub fn set_keyinput(&mut self, value: u16) {
        self.cpu.bus.io.keypad.keyinput = value;
    }

    pub fn bios(&self) -> &[u8] {
        &self.cpu.bus.bios
    }

    pub fn ewram(&self) -> &[u8] {
        self.cpu.bus.ewram.as_slice()
    }

    pub fn iwram(&self) -> &[u8] {
        self.cpu.bus.iwram.as_slice()
    }

    // TODO: I/O register array view
    pub fn io(&self) -> &[u8] {
        todo!()
    }

    pub fn palette(&self) -> &[u8] {
        &self.cpu.bus.ppu.palette
    }

    pub fn vram(&self) -> &[u8] {
        self.cpu.bus.ppu.vram.as_slice()
    }

    pub fn oam(&self) -> &[u8] {
        &self.cpu.bus.ppu.oam
    }

    pub fn rom(&self) -> &[u8] {
        &self.cpu.bus.rom
    }

    pub fn sram(&self) -> &[u8] {
        self.cpu.bus.sram.as_slice()
    }

    fn sync(&mut self, cycles: Cycle) {
        let count = cycles.count();

        self.cpu.bus.tick(count);
        self.cycles += count as u64;
    }
}

impl Gba {
    pub fn color_palette(&self) -> Vec<Color15> {
        self.cpu.bus.ppu.color_palette()
    }

    pub fn objects(&self) -> Vec<Obj> {
        self.cpu.bus.ppu.objects()
    }

    pub fn render_tile(
        &self,
        tile: &[u8],
        color_mode: ColorMode,
        palette_id: usize,
    ) -> Box<[u8; TILE_BUFFER_SIZE]> {
        self.cpu.bus.ppu.render_tile(tile, color_mode, palette_id)
    }

    pub fn step_scanline(&mut self) {
        let initial_scanline = self.cpu.bus.ppu.scanline;

        while self.cpu.bus.ppu.scanline == initial_scanline {
            self.step();
        }
    }

    pub fn step_frame_with_hook(&mut self, breakpoints: &[u32], irq: bool) -> bool {
        let inital_state = self.is_rendering();
        let mut state_switch = false;

        loop {
            if inital_state != self.is_rendering() {
                state_switch = true;
            }

            if !breakpoints.is_empty() {
                let curr_addr = self.cpu.exec_address();

                if breakpoints.contains(&curr_addr) {
                    return true;
                }
            }

            if state_switch && inital_state == self.is_rendering() {
                break false; // frame completed
            }

            let step = self.debug_step();

            if irq && matches!(step.value, GbaStepKind::Interrupt(_)) {
                return true;
            }
        }
    }
}

impl Reset for Gba {
    fn reset(&mut self) {
        self.cpu.reset();
        self.cycles = 0;
    }
}

#[cfg(test)]
mod tests {
    // use crate::{Gba, bus::BIOS_SIZE};
    //
    // const GBA_BIOS: &[u8; BIOS_SIZE] = include_bytes!("../../../bin/gba_bios.bin");
    //
    // #[test]
    // fn test_bios_load() {
    //     let mut gba = Gba::default();
    //
    //     gba.load_bios(*GBA_BIOS);
    //     gba.load_rom(&[0; 8]);
    //     gba.boot();
    //
    //     assert_eq!(gba.cpu.pipeline.current_address(), 0x0800_0000);
    // }
}

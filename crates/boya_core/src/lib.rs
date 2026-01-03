use crate::{
    bus::{BIOS_SIZE, types::Cycle},
    cpu::{Arm7tdmi, common::Exception},
    debug::types::{GbaStep, GbaStepKind},
};

pub mod debug;

#[allow(clippy::all)]
pub mod apu; // TODO: APU implmentation
pub mod bus;
pub mod cpu;
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

    pub fn reset(&mut self) {
        *self = Self::default()
    }

    pub fn boot(&mut self) -> Cycle {
        self.cpu.handle_exception(Exception::Reset)
    }

    pub fn step(&mut self) -> Cycle {
        self.cpu
            .try_irq()
            .map(|irq| irq.cycles)
            .or_else(|| self.cpu.bus.try_dma().map(|dma| dma.cycles))
            .unwrap_or_else(|| self.cpu.step())
    }

    pub fn sync(&mut self, cycles: Cycle) {
        let count = cycles.count();

        self.cpu.bus.tick(count);
        self.cycles += count as u64;
    }

    pub fn synced_step(&mut self) {
        let cycles = self.step();

        self.sync(cycles);
    }

    pub fn debug_step(&mut self) -> GbaStep {
        let value = self
            .cpu
            .try_irq()
            .map(|irq| GbaStepKind::Interrupt(irq))
            .or_else(|| self.cpu.bus.try_dma().map(|dma| GbaStepKind::Dma(dma)))
            .unwrap_or_else(|| GbaStepKind::Instruction(self.cpu.debug_step()));

        GbaStep { value }
    }

    pub fn debug_synced_step(&mut self) -> Cycle {
        let step = self.debug_step();
        let cycles = step.value.cycles();

        self.sync(cycles);

        cycles
    }
}

impl Gba {
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
}

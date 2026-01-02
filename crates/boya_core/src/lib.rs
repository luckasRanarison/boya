use crate::{
    bus::{BIOS_SIZE, types::Cycle},
    cpu::Arm7tdmi,
};

#[allow(clippy::all)]
mod apu; // TODO: APU implmentation
mod bus;
mod cpu;
mod ppu;
mod utils;

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

    pub fn step(&mut self) -> Cycle {
        let cycles = self
            .cpu
            .try_irq()
            .or_else(|| self.cpu.bus.try_dma())
            .unwrap_or_else(|| self.cpu.step());

        let count = cycles.count();

        self.cpu.bus.tick(count);
        self.cycles += count as u64;

        cycles
    }
}

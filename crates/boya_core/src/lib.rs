use crate::{
    bus::{BIOS_SIZE, types::Cycle},
    cpu::{Arm7tdmi, common::Exception},
    utils::Reset,
};

#[allow(clippy::all)]
pub mod apu; // TODO: APU implmentation
pub mod bus;
pub mod cpu;
pub mod ppu;
pub mod utils;

#[cfg(feature = "debug")]
pub mod debug;

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

    pub fn step_frame(&mut self) {
        self.step_visible_frame();
        self.step_vblank();
    }

    pub fn step_visible_frame(&mut self) {
        while self.rendering() {
            self.step();
        }
    }

    pub fn step_vblank(&mut self) {
        while !self.rendering() {
            self.step();
        }
    }

    #[inline(always)]
    pub fn rendering(&self) -> bool {
        self.cpu.bus.ppu.rendering()
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

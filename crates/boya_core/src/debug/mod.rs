use crate::{
    Gba,
    debug::cpu::types::Step,
    ppu::{
        object::Obj,
        pixel::Color15,
        registers::{bgcnt::ColorMode, dispcnt::Background},
    },
};

pub mod bus;
pub mod cpu;
pub mod ppu;

#[derive(Debug)]
pub enum Hook {
    Breakpoints(Vec<u32>),
    Irq(bool),
}

impl Gba {
    pub fn debug_step(&mut self) -> Step {
        let step = self
            .cpu
            .try_irq()
            .map(Step::Interrupt)
            .or_else(|| self.cpu.bus.try_dma().map(Step::Dma))
            .unwrap_or_else(|| Step::Instruction(self.cpu.debug_step()));

        let cycles = step.cycles();

        self.sync(cycles);

        step
    }

    pub fn step_scanline(&mut self) {
        let initial_scanline = self.cpu.bus.ppu.scanline;

        while self.cpu.bus.ppu.scanline == initial_scanline {
            self.step();
        }
    }

    pub fn step_frame_with_hooks(&mut self, hooks: Vec<Hook>) -> bool {
        let inital_state = self.rendering();
        let mut state_switch = false;

        loop {
            if inital_state != self.rendering() {
                state_switch = true;
            }

            for hook in &hooks {
                if self.resolve_break_hook(hook) {
                    return true;
                }
            }

            if inital_state != self.rendering() {
                state_switch = true;
            }

            if state_switch && inital_state == self.rendering() {
                break false; // frame completed
            }

            self.step();
        }
    }

    pub fn color_palette(&self) -> Vec<Color15> {
        self.cpu.bus.ppu.color_palette()
    }

    pub fn object_palette(&self, id: u8) -> Vec<Color15> {
        self.cpu.bus.ppu.object_palette(id)
    }

    pub fn objects(&self) -> Vec<Obj> {
        self.cpu.bus.ppu.objects()
    }

    pub fn render_tile(&self, id: u16, offset: u32, color: ColorMode, palette_id: u8) -> Vec<u8> {
        self.cpu.bus.ppu.render_tile(id, offset, color, palette_id)
    }

    pub fn render_bg(&self, bg: Background) -> Vec<u8> {
        self.cpu.bus.ppu.render_bg(bg)
    }

    pub fn render_obj(&self, id: u8) -> Vec<u8> {
        self.cpu.bus.ppu.render_obj(id)
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

    fn resolve_break_hook(&self, hook: &Hook) -> bool {
        match hook {
            Hook::Breakpoints(breakpoints) => breakpoints.contains(&self.cpu.exec_address()),
            Hook::Irq(irq) => *irq && self.cpu.exec_address() == 0x18,
        }
    }
}

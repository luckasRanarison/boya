use crate::{
    Gba,
    debug::cpu::types::Step,
    ppu::{
        color::Color15,
        object::Obj,
        registers::{bgcnt::ColorMode, dispcnt::Background},
    },
};

pub mod bus;
pub mod cpu;
pub mod ppu;

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

            if irq && matches!(step, Step::Interrupt(_)) {
                return true;
            }
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
}

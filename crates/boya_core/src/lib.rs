use crate::{bus::types::Cycle, cpu::Arm7tdmi};

pub mod bus;
pub mod cpu;
pub mod ppu;
pub mod registers;
pub mod utils;

#[cfg(test)]
mod test;

pub struct Gba {
    pub cpu: Arm7tdmi,
}

impl Gba {
    pub fn step(&mut self) -> Cycle {
        let cycles = self
            .cpu
            .try_irq()
            .or_else(|| self.cpu.bus.try_dma())
            .unwrap_or_else(|| self.cpu.step());

        self.cpu.bus.tick(cycles.count());

        cycles
    }
}

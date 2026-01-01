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
    pub cycles: u64,
}

impl Gba {
    pub fn new(cpu: Arm7tdmi) -> Self {
        Self { cpu, cycles: 0 }
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

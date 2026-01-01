use crate::{
    arm7tdmi::{Arm7tdmi, common::Exception, psr::Psr},
    bus::types::Cycle,
};

#[allow(clippy::upper_case_acronyms)]
pub mod arm7tdmi;
pub mod bus;
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
        if !self.cpu.cpsr.has(Psr::I) && self.cpu.bus.poll_interrupt() {
            return self.cpu.handle_exception(Exception::NormalInterrupt);
        }

        if let Some(cycles) = self.cpu.bus.try_dma() {
            return cycles;
        }

        self.cpu.step()
    }
}

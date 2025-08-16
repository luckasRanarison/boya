use std::fmt::Debug;

use crate::bus::Bus;

use super::{Arm7tdmi, thumb};

#[derive(Debug, Default)]
pub struct Pipeline {
    pub next_instr: u32,
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn reload_pipeline(&mut self) {
        self.pipeline.next_instr = self.fetch();
        self.increment_pc();
    }

    pub fn fetch_pipeline(&mut self) -> u32 {
        self.pipeline.next_instr
    }

    pub fn pre_fetch(&mut self) {
        self.pipeline.next_instr = self.fetch();
        self.increment_pc();
    }
}

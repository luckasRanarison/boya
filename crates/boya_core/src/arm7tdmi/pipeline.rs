use std::fmt::Debug;

use crate::bus::Bus;

use super::Arm7tdmi;

#[derive(Debug, Default)]
pub struct Pipeline {
    next_instr: u32,
    last_pc: u32,
}

impl Pipeline {
    pub fn next(&self) -> u32 {
        self.next_instr
    }

    pub fn last_pc(&self) -> u32 {
        self.last_pc
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn reload_pipeline(&mut self) {
        self.align_pc();
        self.pre_fetch();
    }

    pub fn pre_fetch(&mut self) {
        self.pipeline.next_instr = self.fetch();
        self.increment_pc();
        self.pipeline.last_pc = self.pc();
    }
}

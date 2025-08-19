use std::fmt::Debug;

use crate::bus::Bus;

use super::{Arm7tdmi, Instruction};

#[derive(Debug, Default)]
pub struct Pipeline {
    current: Option<Instruction>,
    next: Option<u32>,
    last_pc: u32,
}

impl Pipeline {
    pub fn take(&mut self) -> Option<Instruction> {
        self.current.take()
    }

    pub fn last_pc(&self) -> u32 {
        self.last_pc
    }

    pub fn flush(&mut self) {
        self.current.take();
        self.next.take();
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn load_pipeline(&mut self) {
        let current = self.pipeline.next.unwrap_or_else(|| self.fetch());

        self.pipeline.current = Some(self.decode(current));
        self.pipeline.next = Some(self.fetch());
        self.pipeline.last_pc = self.pc();
    }
}

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
    pub fn reload_pipeline(&mut self) {
        self.pipeline.flush();
        self.pre_fetch();
    }

    pub fn pre_fetch(&mut self) {
        let offset = self.instruction_size().into();
        let current = self.pipeline.next.unwrap_or_else(|| self.fetch());

        self.pipeline.current = Some(self.decode(current));
        self.shift_pc(offset);
        self.pipeline.next = Some(self.fetch());
        self.shift_pc(offset);
        self.pipeline.last_pc = self.pc();
    }
}

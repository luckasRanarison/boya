use std::fmt::Debug;

use crate::bus::Bus;

use super::{Arm7tdmi, Instruction};

#[derive(Debug, Default)]
pub struct Pipeline {
    pub curr_pc: u32,
    pub last_pc: u32,
    pub curr_word: Option<u32>,
    pub next_word: Option<u32>,
    pub curr_instr: Option<Instruction>,
}

impl Pipeline {
    pub fn take(&mut self) -> Instruction {
        self.curr_instr.take().unwrap() // pipeline should always be pre-loaded
    }

    pub fn last_pc(&self) -> u32 {
        self.last_pc
    }

    pub fn flush(&mut self) {
        self.curr_instr.take();
        self.curr_word.take();
        self.next_word.take();
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn next_instr_addr(&self) -> Option<u32> {
        let last_pc = self.pipeline.last_pc();
        let instr_size = self.instr_size().into();

        last_pc.checked_sub(instr_size)
    }

    pub fn load_pipeline(&mut self) {
        let curr_pc = self.pc();
        let word = self.pipeline.next_word.unwrap_or_else(|| self.fetch());

        self.pipeline.curr_pc = curr_pc;
        self.pipeline.curr_word = Some(word);
        self.pipeline.curr_instr = Some(self.decode(word));
        self.pipeline.next_word = Some(self.fetch());
        self.pipeline.last_pc = self.pc();
    }
}

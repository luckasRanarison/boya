use std::fmt::Debug;

use super::{Arm7tdmi, Instruction};

#[derive(Debug, Default)]
pub struct Pipeline {
    curr_pc: u32,
    next_pc: u32,
    curr_word: Option<u32>,
    next_word: Option<u32>,
    curr_instr: Option<Instruction>,
}

impl Pipeline {
    /// # Panics
    ///
    /// If pipeline has not been loaded.
    pub fn take(&mut self) -> Instruction {
        self.curr_instr
            .take()
            .expect("pipeline has not been loaded")
    }

    pub fn next_address(&self) -> u32 {
        self.next_pc
    }

    pub fn current_address(&self) -> u32 {
        self.curr_pc
    }

    pub fn current_instruction(&self) -> Option<&Instruction> {
        self.curr_instr.as_ref()
    }

    pub fn flush(&mut self) {
        self.curr_instr.take();
        self.curr_word.take();
        self.next_word.take();
    }
}

impl Arm7tdmi {
    pub fn load_pipeline(&mut self) {
        let curr_pc = self.pc();
        let word = self.pipeline.next_word.unwrap_or_else(|| self.fetch());

        self.pipeline.curr_pc = curr_pc;
        self.pipeline.curr_word = Some(word);
        self.pipeline.curr_instr = Some(self.decode(word));
        self.pipeline.next_word = Some(self.fetch());
        self.pipeline.next_pc = self.pc();
    }
}

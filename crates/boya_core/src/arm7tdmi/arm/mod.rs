use crate::{arm7tdmi::Arm7tdmi, bus::Bus};

#[derive(Debug)]
pub enum ArmInstr {}

impl<B: Bus> Arm7tdmi<B> {
    pub fn decode_arm(&self, word: u32) -> ArmInstr {
        todo!()
    }
    pub fn exec_arm(&mut self, instruction: ArmInstr) {
        todo!()
    }
}

use crate::{
    arm7tdmi::{Arm7tdmi, common::Cycle},
    bus::Bus,
};

#[derive(Debug)]
pub enum ArmInstr {}

impl<B: Bus> Arm7tdmi<B> {
    pub fn decode_arm(&self, _word: u32) -> ArmInstr {
        todo!()
    }

    pub fn exec_arm(&mut self, _instruction: ArmInstr) -> Cycle {
        todo!()
    }
}

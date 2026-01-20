use crate::{
    bus::Bus,
    cpu::{Arm7tdmi, debug::types::InstructionData},
};

impl Arm7tdmi {
    pub fn decode_until_branch(&self) -> Vec<(u32, InstructionData)> {
        let mut instructions = vec![];
        let mut address = self.pipeline.current_address();

        loop {
            let raw = self.bus.read_word(address);
            let instruction = self.decode(raw);
            address += self.instr_size() as u32;
            instructions.push((address, instruction.get_data()));

            if instruction.is_branch() {
                break;
            }
        }

        instructions
    }
}

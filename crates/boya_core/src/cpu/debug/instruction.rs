use crate::{
    bus::Bus,
    cpu::{Arm7tdmi, debug::types::InstructionData},
};

impl Arm7tdmi {
    pub fn decode_until_branch(&self, max_depth: u16) -> Vec<(u32, InstructionData)> {
        let mut instructions = vec![];
        let mut address = self.pipeline.current_address();

        for _ in 0..max_depth {
            let raw = self.bus.read_word(address);
            let instruction = self.decode(raw);

            instructions.push((address, instruction.get_data()));
            address += self.instr_size() as u32;

            if instruction.is_branch() {
                break;
            }
        }

        instructions
    }
}

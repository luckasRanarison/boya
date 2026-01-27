use crate::{
    bus::Bus,
    cpu::{Arm7tdmi, debug::types::InstructionData},
};

impl Arm7tdmi {
    pub fn decode_until_branch(&self, max_depth: u16) -> Vec<(u32, InstructionData)> {
        let mut instructions = vec![];
        let mut address = self.exec_address();
        let instr_size = self.instr_size() as u32;

        for i in 0..max_depth {
            let raw = self.bus.read_word(address);
            let instruction = self.decode(raw);

            instructions.push((address, instruction.get_data()));
            address += instr_size;

            if instruction.is_branch() && i >= 1 {
                break;
            }
        }

        instructions
    }

    pub fn starting_subroutine(&self) -> bool {
        self.pipeline
            .current_instruction()
            .map(|instr| instr.is_branch_link())
            .unwrap_or_default()
    }
}

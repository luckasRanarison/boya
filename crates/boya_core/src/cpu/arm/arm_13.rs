use crate::cpu::isa::prelude::*;

/// Software interrupt
/// +-----------------------------------------------------------------+
/// |...3 ..................2 ..................1 ..................0.|
/// |-----------------------------------------------------------------|
/// |_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_|
/// |-----------------------------------------------------------------|
/// |  Cond  |1 1 1 1|                       nn                       |
/// +-----------------------------------------------------------------+
pub struct Instruction {
    nn: u32,
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        let nn = value.get_bits(0, 23);

        Self { nn }
    }
}

impl Executable for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        cpu.swi()
    }

    fn get_data(&self) -> InstructionData {
        InstructionData {
            keyword: "SWI".into(),
            args: vec![self.nn.imm().into()],
            kind: InstructionKind::arm(13, None, None, false),
        }
    }
}

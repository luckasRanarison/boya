use crate::{
    cpu::{
        arm::{Arm, arm_04},
        isa::Instruction,
        thumb::{Thumb, thumb_05},
    },
    debug::cpu::types::InstructionData,
};

pub trait Inspectable {
    fn inspect(&self) -> InstructionData;
}

impl Inspectable for Instruction {
    fn inspect(&self) -> InstructionData {
        match self {
            Instruction::Arm(arm) => arm.inspect(),
            Instruction::Thumb(thumb) => thumb.inspect(),
        }
    }
}

impl Instruction {
    pub fn is_branch(&self) -> bool {
        match self {
            Instruction::Arm(Arm::Arm03(_))
            | Instruction::Arm(Arm::Arm04(_))
            | Instruction::Thumb(Thumb::Format16(_))
            | Instruction::Thumb(Thumb::Format18(_))
            | Instruction::Thumb(Thumb::Format19(_)) => true,
            Instruction::Thumb(Thumb::Format05(instr)) => matches!(instr.op, thumb_05::Opcode::BX),
            _ => false,
        }
    }

    pub fn is_branch_link(&self) -> bool {
        match self {
            Instruction::Thumb(Thumb::Format19(instr)) => instr.h,
            Instruction::Arm(Arm::Arm04(instr)) => matches!(instr.op, arm_04::Opcode::BL),
            _ => false,
        }
    }
}

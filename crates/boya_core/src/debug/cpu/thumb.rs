use crate::{
    cpu::{
        common::{AddrMode, NamedRegister, ToOperand},
        thumb::*,
    },
    debug::cpu::{inspect::Inspectable, types::*},
};

impl Inspectable for Thumb {
    fn inspect(&self) -> InstructionData {
        match self {
            Thumb::Format01(op) => op.inspect(),
            Thumb::Format02(op) => op.inspect(),
            Thumb::Format03(op) => op.inspect(),
            Thumb::Format04(op) => op.inspect(),
            Thumb::Format05(op) => op.inspect(),
            Thumb::Format06(op) => op.inspect(),
            Thumb::Format07(op) => op.inspect(),
            Thumb::Format08(op) => op.inspect(),
            Thumb::Format09(op) => op.inspect(),
            Thumb::Format10(op) => op.inspect(),
            Thumb::Format11(op) => op.inspect(),
            Thumb::Format12(op) => op.inspect(),
            Thumb::Format13(op) => op.inspect(),
            Thumb::Format14(op) => op.inspect(),
            Thumb::Format15(op) => op.inspect(),
            Thumb::Format16(op) => op.inspect(),
            Thumb::Format17(op) => op.inspect(),
            Thumb::Format18(op) => op.inspect(),
            Thumb::Format19(op) => op.inspect(),
            Thumb::Undefined(op) => InstructionData::undefined_thumb(*op),
        }
    }
}

impl Inspectable for thumb_01::Instruction {
    fn inspect(&self) -> InstructionData {
        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![
                self.rd.reg().into(),
                self.rs.reg().into(),
                self.of.imm().into(),
            ],
            kind: InstructionKind::thumb(1),
        }
    }
}

impl Inspectable for thumb_02::Instruction {
    fn inspect(&self) -> InstructionData {
        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![
                self.rd.reg().into(),
                self.rs.reg().into(),
                self.nn.clone().into(),
            ],
            kind: InstructionKind::thumb(2),
        }
    }
}

impl Inspectable for thumb_03::Instruction {
    fn inspect(&self) -> InstructionData {
        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![self.rd.reg().into(), self.nn.imm().into()],
            kind: InstructionKind::thumb(3),
        }
    }
}

impl Inspectable for thumb_04::Instruction {
    fn inspect(&self) -> InstructionData {
        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![self.rd.reg().into(), self.rs.reg().into()],
            kind: InstructionKind::thumb(4),
        }
    }
}

impl Inspectable for thumb_05::Instruction {
    fn inspect(&self) -> InstructionData {
        let args = match &self.op {
            thumb_05::Opcode::BX => vec![self.rs.reg().into()],
            _ => vec![self.rd.reg().into(), self.rs.reg().into()],
        };

        InstructionData {
            keyword: format!("{:?}", self.op),
            kind: InstructionKind::thumb(5),
            args,
        }
    }
}

impl Inspectable for thumb_06::Instruction {
    fn inspect(&self) -> InstructionData {
        let offset = RegisterOffsetData {
            amod: AddrMode::IB,
            base: RegisterOffsetBase::Named(NamedRegister::PC),
            offset: Some(self.nn.imm()),
            wb: false,
        };

        InstructionData {
            keyword: "LDR".to_string(),
            args: vec![self.rd.reg().into(), offset.into()],
            kind: InstructionKind::thumb(6),
        }
    }
}

impl Inspectable for thumb_07::Instruction {
    fn inspect(&self) -> InstructionData {
        let offset = RegisterOffsetData::simple(self.rb, self.ro.reg());

        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![self.rd.reg().into(), offset.into()],
            kind: InstructionKind::thumb(7),
        }
    }
}

impl Inspectable for thumb_08::Instruction {
    fn inspect(&self) -> InstructionData {
        let offset = RegisterOffsetData::simple(self.rb, self.ro.reg());

        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![self.rd.reg().into(), offset.into()],
            kind: InstructionKind::thumb(8),
        }
    }
}

impl Inspectable for thumb_09::Instruction {
    fn inspect(&self) -> InstructionData {
        let offset = RegisterOffsetData::simple(self.rb, self.nn.imm());

        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![self.rd.reg().into(), offset.into()],
            kind: InstructionKind::thumb(9),
        }
    }
}

impl Inspectable for thumb_10::Instruction {
    fn inspect(&self) -> InstructionData {
        let offset = RegisterOffsetData::simple(self.rb, self.nn.imm());

        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![self.rd.reg().into(), offset.into()],
            kind: InstructionKind::thumb(10),
        }
    }
}

impl Inspectable for thumb_11::Instruction {
    fn inspect(&self) -> InstructionData {
        let offset = RegisterOffsetData {
            amod: AddrMode::IB,
            base: RegisterOffsetBase::Named(NamedRegister::SP),
            offset: Some(self.nn.imm()),
            wb: false,
        };

        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![self.rd.reg().into(), offset.into()],
            kind: InstructionKind::thumb(11),
        }
    }
}

impl Inspectable for thumb_12::Instruction {
    fn inspect(&self) -> InstructionData {
        InstructionData {
            keyword: "ADD".to_string(),
            args: vec![self.rd.reg().into(), self.rs.into(), self.nn.imm().into()],
            kind: InstructionKind::thumb(12),
        }
    }
}

impl Inspectable for thumb_13::Instruction {
    fn inspect(&self) -> InstructionData {
        InstructionData {
            keyword: "ADD".to_string(),
            args: vec![NamedRegister::SP.into(), self.nn.clone().into()],
            kind: InstructionKind::thumb(13),
        }
    }
}

impl Inspectable for thumb_14::Instruction {
    fn inspect(&self) -> InstructionData {
        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![RegisterList::new(self.rlist.into()).into()],
            kind: InstructionKind::thumb(14),
        }
    }
}

impl Inspectable for thumb_15::Instruction {
    fn inspect(&self) -> InstructionData {
        let offset = RegisterOffsetData {
            amod: AddrMode::IA,
            base: RegisterOffsetBase::Plain(self.rb),
            offset: None,
            wb: true,
        };
        let rlist = RegisterList::new(self.rlist.into());

        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![offset.into(), rlist.into()],
            kind: InstructionKind::thumb(15),
        }
    }
}

impl Inspectable for thumb_16::Instruction {
    fn inspect(&self) -> InstructionData {
        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![InstructionParam::BranchOffset(self.of.into())],
            kind: InstructionKind::thumb(16),
        }
    }
}

impl Inspectable for thumb_17::Instruction {
    fn inspect(&self) -> InstructionData {
        InstructionData {
            keyword: "SWI".into(),
            args: vec![self.nn.imm().into()],
            kind: InstructionKind::thumb(17),
        }
    }
}

impl Inspectable for thumb_18::Instruction {
    fn inspect(&self) -> InstructionData {
        InstructionData {
            keyword: "B".into(),
            args: vec![InstructionParam::BranchOffset(self.of.into())],
            kind: InstructionKind::thumb(18),
        }
    }
}

impl Inspectable for thumb_19::Instruction {
    fn inspect(&self) -> InstructionData {
        InstructionData {
            keyword: (if self.h { "BLH" } else { "BLL" }).to_string(),
            args: vec![InstructionParam::Operand(self.nn.imm())],
            kind: InstructionKind::thumb(19),
        }
    }
}

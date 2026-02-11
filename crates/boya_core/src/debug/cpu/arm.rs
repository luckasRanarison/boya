use crate::{
    cpu::{arm::*, common::ToOperand},
    debug::cpu::{inspect::Inspectable, types::*},
};

impl Inspectable for Arm {
    fn inspect(&self) -> InstructionData {
        match self {
            Arm::Arm03(op) => op.inspect(),
            Arm::Arm04(op) => op.inspect(),
            Arm::Arm05(op) => op.inspect(),
            Arm::Arm06(op) => op.inspect(),
            Arm::Arm07(op) => op.inspect(),
            Arm::Arm08(op) => op.inspect(),
            Arm::Arm09(op) => op.inspect(),
            Arm::Arm10(op) => op.inspect(),
            Arm::Arm11(op) => op.inspect(),
            Arm::Arm12(op) => op.inspect(),
            Arm::Arm13(op) => op.inspect(),
            Arm::Undefined(op) => InstructionData::undefined_arm(*op),
        }
    }
}

impl Inspectable for arm_03::Instruction {
    fn inspect(&self) -> InstructionData {
        InstructionData {
            keyword: "BX".into(),
            args: vec![self.rn.reg().into()],
            kind: InstructionKind::arm(3, self.cd.into(), None, false),
        }
    }
}

impl Inspectable for arm_04::Instruction {
    fn inspect(&self) -> InstructionData {
        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![InstructionParam::BranchOffset(self.nn)],
            kind: InstructionKind::arm(4, self.cd.into(), None, false),
        }
    }
}

impl Inspectable for arm_05::Instruction {
    fn inspect(&self) -> InstructionData {
        use arm_05::Opcode;

        let op2 = self.op2.clone().into();
        let rn = self.rn.reg().into();
        let rd = self.rd.reg().into();

        let args = match self.op {
            Opcode::TST | Opcode::TEQ | Opcode::CMP | Opcode::CMN => vec![rn, op2],
            Opcode::MOV | Opcode::MVN => vec![rd, op2],
            _ => vec![rd, rn, op2],
        };

        InstructionData {
            keyword: format!("{:?}", self.op),
            args,
            kind: InstructionKind::arm(5, self.cd.into(), self.s.into(), false),
        }
    }
}

impl Inspectable for arm_06::Instruction {
    fn inspect(&self) -> InstructionData {
        use arm_06::Opcode;

        let (keyword, args) = match &self.op {
            Opcode::MRS { rd } => ("MRS", vec![rd.reg().into()]),
            Opcode::MSR { fd, op } => (
                "MSR",
                vec![
                    InstructionParam::PsrUpdate(PsrUpdate {
                        kind: self.psr,
                        fields: fd.clone(),
                    }),
                    op.clone().into(),
                ],
            ),
        };

        InstructionData {
            keyword: keyword.to_string(),
            kind: InstructionKind::arm(6, self.cd.into(), None, false),
            args,
        }
    }
}

impl Inspectable for arm_07::Instruction {
    fn inspect(&self) -> InstructionData {
        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![
                self.rd.reg().into(),
                self.rm.reg().into(),
                self.rs.reg().into(),
            ],
            kind: InstructionKind::arm(7, self.cd.into(), None, false),
        }
    }
}

impl Inspectable for arm_08::Instruction {
    fn inspect(&self) -> InstructionData {
        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![
                self.lo.reg().into(),
                self.hi.reg().into(),
                self.rm.reg().into(),
                self.rs.reg().into(),
            ],
            kind: InstructionKind::arm(8, self.cd.into(), None, false),
        }
    }
}

impl Inspectable for arm_09::Instruction {
    fn inspect(&self) -> InstructionData {
        let offset = RegisterOffsetData {
            amod: self.amod,
            base: RegisterOffsetBase::Plain(self.rn),
            offset: Some(self.of.clone()),
            wb: self.wb,
        };

        InstructionData {
            keyword: format!("{:?}{}", self.op, if self.b { "B" } else { "" }),
            args: vec![self.rd.reg().into(), offset.into()],
            kind: InstructionKind::arm(9, self.cd.into(), None, false),
        }
    }
}

impl Inspectable for arm_10::Instruction {
    fn inspect(&self) -> InstructionData {
        let offset = RegisterOffsetData {
            amod: self.amod,
            base: RegisterOffsetBase::Plain(self.rn),
            offset: Some(self.of.clone()),
            wb: self.wb,
        };

        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![self.rd.reg().into(), offset.into()],
            kind: InstructionKind::arm(10, self.cd.into(), None, false),
        }
    }
}

impl Inspectable for arm_11::Instruction {
    fn inspect(&self) -> InstructionData {
        let offset = RegisterOffsetData {
            amod: self.amod,
            base: RegisterOffsetBase::Plain(self.rn),
            offset: None,
            wb: self.wb,
        };
        let rlist = RegisterList::new(self.rlist);

        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![offset.into(), rlist.into()],
            kind: InstructionKind::arm(11, self.cd.into(), None, self.u),
        }
    }
}

impl Inspectable for arm_12::Instruction {
    fn inspect(&self) -> InstructionData {
        InstructionData {
            keyword: format!("SWP{}", if self.b { "B" } else { "" }),
            args: vec![
                self.rd.reg().into(),
                self.rm.reg().into(),
                InstructionParam::Address(self.rn.reg()),
            ],
            kind: InstructionKind::arm(13, None, None, false),
        }
    }
}

impl Inspectable for arm_13::Instruction {
    fn inspect(&self) -> InstructionData {
        InstructionData {
            keyword: "SWI".into(),
            args: vec![self.nn.imm().into()],
            kind: InstructionKind::arm(13, None, None, false),
        }
    }
}

use crate::{
    bus::types::Cycle,
    cpu::{
        Arm7tdmi,
        common::{AddrMode, Condition, Exception, NamedRegister, Operand},
        isa::Executable,
        psr::{PsrField, PsrKind},
    },
};

#[derive(Debug)]
pub enum InstructionKind {
    Arm {
        format: u8,
        condition: Condition,
        update: Option<bool>,
    },
    Thumb {
        format: u8,
    },
}

impl InstructionKind {
    pub fn thumb(format: u8) -> Self {
        Self::Thumb { format }
    }

    pub fn arm(format: u8, condition: Condition, update: Option<bool>) -> Self {
        Self::Arm {
            format,
            condition,
            update,
        }
    }
}

pub struct DebuggableInstruction {
    pub keyword: String,
    pub args: Vec<InstructionParam>,
    pub kind: InstructionKind,
    pub source: Box<dyn DebuggableExecutable>,
}

#[derive(Debug)]
pub enum InstructionParam {
    Namedregister(NamedRegister),
    Operand(Operand),
    RegisterOffset(DebuggableRegisterOffset),
    RegisterList(RegisterList),
    BranchOffset(i32),
    PsrUpdate { kind: PsrKind, fields: PsrField },
}

impl From<Operand> for InstructionParam {
    fn from(value: Operand) -> Self {
        Self::Operand(value)
    }
}

impl From<DebuggableRegisterOffset> for InstructionParam {
    fn from(value: DebuggableRegisterOffset) -> Self {
        Self::RegisterOffset(value)
    }
}

impl From<NamedRegister> for InstructionParam {
    fn from(value: NamedRegister) -> Self {
        Self::Namedregister(value)
    }
}

impl From<RegisterList> for InstructionParam {
    fn from(value: RegisterList) -> Self {
        Self::RegisterList(value)
    }
}

#[derive(Debug)]
pub enum RegisterOffsetBase {
    Plain(u8),
    Named(NamedRegister),
}

#[derive(Debug)]
pub struct DebuggableRegisterOffset {
    pub amod: AddrMode,
    pub base: RegisterOffsetBase,
    pub offset: Option<Operand>,
    pub wb: bool,
}

impl DebuggableRegisterOffset {
    pub fn simple(base: u8, offset: Operand) -> Self {
        DebuggableRegisterOffset {
            amod: AddrMode::IB,
            base: RegisterOffsetBase::Plain(base),
            offset: Some(offset),
            wb: false,
        }
    }
}

#[derive(Debug)]
pub struct RegisterList {
    pub value: u16,
    pub named: Option<NamedRegister>,
}

impl RegisterList {
    pub fn new(value: u16, named: Option<NamedRegister>) -> Self {
        Self { value, named }
    }
}

#[derive(Debug)]
pub enum UndefinedInstruction {
    Thumb(u16),
    Arm(u32),
}

impl From<UndefinedInstruction> for DebuggableInstruction {
    fn from(value: UndefinedInstruction) -> Self {
        let (opcode, kind) = match value {
            UndefinedInstruction::Thumb(opcode) => (opcode.into(), InstructionKind::thumb(0)),
            UndefinedInstruction::Arm(opcode) => {
                (opcode, InstructionKind::arm(0, Condition::AL, None))
            }
        };

        Self {
            keyword: format!("{:#02X}", opcode),
            args: vec![],
            source: Box::new(value),
            kind,
        }
    }
}

// wrap the `Executable` trait because it cannot be used as a trait object
impl DebuggableExecutable for UndefinedInstruction {
    fn debuggable_dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        cpu.handle_exception(Exception::Undefined)
    }
}

pub trait DebuggableExecutable {
    fn debuggable_dispatch(self, cpu: &mut Arm7tdmi) -> Cycle;
}

impl<T: Executable> DebuggableExecutable for T {
    fn debuggable_dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        self.dispatch_checked(cpu)
    }
}

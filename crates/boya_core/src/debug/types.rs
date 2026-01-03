use crate::{
    bus::{registers::dma::DmaResult, types::Cycle},
    cpu::{
        common::{AddrMode, Condition, NamedRegister, Operand},
        psr::{PsrField, PsrKind},
    },
};

#[derive(Debug)]
pub enum GbaStepKind {
    PlainInstruction(Cycle),
    DetailedInstruction(InstructionResult),
    DirectMemoryAccess(DmaResult),
    Interrupt(Cycle),
}

impl GbaStepKind {
    pub fn cycles(&self) -> Cycle {
        match self {
            GbaStepKind::DetailedInstruction(data) => data.cycles,
            GbaStepKind::DirectMemoryAccess(data) => data.cycles,
            GbaStepKind::PlainInstruction(cycles) | GbaStepKind::Interrupt(cycles) => *cycles,
        }
    }
}

#[derive(Debug)]
pub struct InstructionResult {
    pub data: InstructionData,
    pub cycles: Cycle,
}

#[derive(Debug)]
pub struct InstructionData {
    pub keyword: String,
    pub args: Vec<InstructionParam>,
    pub kind: InstructionKind,
}

impl InstructionData {
    pub fn undefined_thumb(opcode: u16) -> Self {
        Self {
            keyword: format!("{opcode:#02X}"),
            args: vec![],
            kind: InstructionKind::thumb(0),
        }
    }

    pub fn undefined_arm(opcode: u32) -> Self {
        Self {
            keyword: format!("{opcode:#02X}"),
            args: vec![],
            kind: InstructionKind::arm(0, None, None, false),
        }
    }
}

impl InstructionData {
    pub fn format(&self, instr_width: usize) -> String {
        let keyword = self.format_keyword();
        let args = self.format_args();

        format!("{keyword:<instr_width$} {args}")
    }

    pub fn format_args(&self) -> String {
        let args = self
            .args
            .iter()
            .map(|arg| format!("{arg:?}"))
            .collect::<Vec<_>>()
            .join(", ");
        let arg_suffix = self.arg_suffix().unwrap_or_default();

        let cased_args = match self.kind {
            InstructionKind::Arm(_) => args.to_uppercase(),
            InstructionKind::Thumb(_) => args.to_lowercase(),
        };

        format!("{cased_args}{arg_suffix}")
    }

    pub fn format_keyword(&self) -> String {
        match &self.kind {
            InstructionKind::Arm(data) => format!("{}{}", self.keyword, data.instruction_suffix()),
            InstructionKind::Thumb(_) => self.keyword.to_lowercase().to_string(),
        }
    }

    fn arg_suffix(&self) -> Option<&str> {
        match &self.kind {
            InstructionKind::Arm(data) => data.user.then_some("^"),
            InstructionKind::Thumb(_) => None,
        }
    }
}

#[derive(Debug)]
pub enum InstructionKind {
    Arm(ArmData),
    Thumb(ThumbData),
}

impl InstructionKind {
    pub fn thumb(format: u8) -> Self {
        Self::Thumb(ThumbData { format })
    }

    pub fn arm(format: u8, condition: Option<Condition>, update: Option<bool>, user: bool) -> Self {
        Self::Arm(ArmData {
            format,
            condition,
            update,
            user,
        })
    }
}

#[derive(Debug)]
pub struct ArmData {
    pub format: u8,
    pub condition: Option<Condition>,
    pub update: Option<bool>,
    pub user: bool,
}

impl ArmData {
    fn instruction_suffix(&self) -> String {
        let condition = self
            .condition
            .filter(|c| !matches!(c, Condition::AL))
            .map(|c| format!("{c:?}"))
            .unwrap_or_default();

        let update = self
            .update
            .and_then(|s| s.then_some("S".to_string()))
            .unwrap_or_default();

        format!("{condition}{update}")
    }
}

#[derive(Debug)]
pub struct ThumbData {
    pub format: u8,
}

#[derive(Debug)]
pub enum InstructionParam {
    Namedregister(NamedRegister),
    Operand(Operand),
    RegisterOffset(RegisterOffsetData),
    Address(Operand),
    RegisterList(RegisterList),
    BranchOffset(i32),
    PsrUpdate(PsrUpdate),
}

impl From<Operand> for InstructionParam {
    fn from(value: Operand) -> Self {
        Self::Operand(value)
    }
}

impl From<RegisterOffsetData> for InstructionParam {
    fn from(value: RegisterOffsetData) -> Self {
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
pub struct RegisterOffsetData {
    pub amod: AddrMode,
    pub base: RegisterOffsetBase,
    pub offset: Option<Operand>,
    pub wb: bool,
}

impl RegisterOffsetData {
    pub fn simple(base: u8, offset: Operand) -> Self {
        RegisterOffsetData {
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
}

impl RegisterList {
    pub fn new(value: u16) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub struct PsrUpdate {
    pub kind: PsrKind,
    pub fields: PsrField,
}

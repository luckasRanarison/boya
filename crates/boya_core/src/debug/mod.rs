use std::fmt;

use crate::{
    cpu::{
        common::{AddrMode, Condition, NamedRegister, Operand, ToOperand},
        psr::{PsrField, PsrKind},
    },
    utils::bitflags::BitIter,
};

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

pub struct ThumbData {
    pub format: u8,
}

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

pub enum InstructionParam {
    Namedregister(NamedRegister),
    Operand(Operand),
    RegisterOffset(RegisterOffsetData),
    Address(Operand),
    RegisterList(RegisterList),
    BranchOffset(i32),
    PsrUpdate(PsrUpdate),
}

impl fmt::Debug for InstructionParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionParam::Namedregister(reg) => write!(f, "{reg:?}"),
            InstructionParam::Operand(operand) => write!(f, "{operand:?}"),
            InstructionParam::RegisterOffset(offset) => write!(f, "{offset:?}"),
            InstructionParam::Address(addr) => write!(f, "[{addr:?}]"),
            InstructionParam::RegisterList(rlist) => write!(f, "{rlist:?}"),
            InstructionParam::BranchOffset(offset) => write!(f, "{offset}"),
            InstructionParam::PsrUpdate(update) => write!(f, "{update:?}"),
        }
    }
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

pub enum RegisterOffsetBase {
    Plain(u8),
    Named(NamedRegister),
}

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

impl fmt::Debug for RegisterOffsetData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rn = match self.base {
            RegisterOffsetBase::Plain(base) => format!("{:?}", base.reg()),
            RegisterOffsetBase::Named(named) => format!("{named:?}"),
        };

        let addr = match &self.offset {
            Some(offset) => match self.amod {
                AddrMode::IB | AddrMode::DB if offset.is_imm() && offset.value == 0 => {
                    format!("[{rn:?}]")
                }
                AddrMode::IB => format!("[{rn}, {:?}]", offset),
                AddrMode::DB if offset.is_imm() => format!("[{rn}, #-{:?}]", offset.value),
                AddrMode::DB => format!("[{rn}, -{:?}]", offset),
                AddrMode::IA => format!("[{rn}], {:?}", offset),
                AddrMode::DA if offset.is_imm() => format!("[{rn}], #-{:?}", offset.value),
                AddrMode::DA => format!("[{rn}], -{:?}", offset),
            },
            _ => rn,
        };

        match self.wb {
            true => write!(f, "{addr}!"),
            false => write!(f, "{addr}"),
        }
    }
}

pub struct RegisterList {
    pub value: u16,
}

impl RegisterList {
    pub fn new(value: u16) -> Self {
        Self { value }
    }
}

impl fmt::Debug for RegisterList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self
            .value
            .iter_lsb()
            .filter(|(_, bit)| *bit == 1)
            .map(|(i, _)| format!("R{i}"))
            .collect::<Vec<_>>()
            .join(",");

        write!(f, "{{{inner}}}")
    }
}

pub struct PsrUpdate {
    pub kind: PsrKind,
    pub fields: PsrField,
}

impl fmt::Debug for PsrUpdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}_{:?}", self.kind, self.fields)
    }
}

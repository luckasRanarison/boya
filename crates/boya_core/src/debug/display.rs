use std::fmt;

use crate::{
    cpu::{
        common::*,
        psr::{Psr, PsrField},
    },
    debug::types::*,
    utils::bitflags::BitIter,
};

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lhs = match self.kind {
            OperandKind::Imm if self.negate => format!("#-{}", self.value),
            OperandKind::Imm => format!("#{}", self.value),
            OperandKind::Reg => format!("R{}", self.value),
        };

        if let Some(shift) = &self.shift {
            write!(
                f,
                "{lhs}, {:?} {}{}",
                shift.kind,
                if shift.register { "R" } else { "#" },
                if shift.value == 0 { 32 } else { shift.value }
            )
        } else {
            write!(f, "{lhs}")
        }
    }
}
impl fmt::Display for InstructionParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionParam::Namedregister(reg) => write!(f, "{reg:?}"),
            InstructionParam::Operand(operand) => write!(f, "{operand}"),
            InstructionParam::RegisterOffset(offset) => write!(f, "{offset}"),
            InstructionParam::Address(addr) => write!(f, "[{addr}]"),
            InstructionParam::RegisterList(rlist) => write!(f, "{rlist}"),
            InstructionParam::BranchOffset(offset) => write!(f, "{offset}"),
            InstructionParam::PsrUpdate(update) => write!(f, "{update}"),
        }
    }
}

impl fmt::Display for RegisterOffsetData {
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

impl fmt::Display for RegisterList {
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

impl fmt::Display for Psr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "N: {}, Z: {}, C: {}, V: {}, I: {}, F: {}, T: {}, M: {:?}",
            self.get(Self::N),
            self.get(Self::Z),
            self.get(Self::C),
            self.get(Self::V),
            self.get(Self::I),
            self.get(Self::F),
            self.get(Self::T),
            self.op_mode()
        )
    }
}

impl fmt::Display for PsrField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let f_fld = if self.mask & 0xFF000000 != 0 { "F" } else { "" };
        let s_fld = if self.mask & 0x00FF0000 != 0 { "S" } else { "" };
        let x_fld = if self.mask & 0x0000FF00 != 0 { "X" } else { "" };
        let c_fld = if self.mask & 0x000000FF != 0 { "C" } else { "" };

        write!(f, "{f_fld}{s_fld}{x_fld}{c_fld}")
    }
}

impl fmt::Display for PsrUpdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}_{:?}", self.kind, self.fields)
    }
}

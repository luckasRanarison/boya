mod arm;
mod bank;
mod common;
mod isa;
mod ops;
mod pipeline;
mod psr;
mod thumb;

use std::fmt::Debug;

use bank::Bank;
use common::{Operand, OperandKind, RegisterFx};
use pipeline::Pipeline;
use psr::{Exception, Psr};
use thumb::ThumbInstr;

#[cfg(test)]
use common::DataType;

use crate::{arm7tdmi::arm::ArmInstr, bus::Bus};

pub enum Instruction {
    Arm(ArmInstr),
    Thumb(ThumbInstr),
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Arm(op) => write!(f, "{op:?}"),
            Instruction::Thumb(op) => write!(f, "{op:?}"),
        }
    }
}

#[derive(Debug)]
pub struct Arm7tdmi<B: Bus> {
    reg: [u32; 16], // R0-R15
    bank: Bank,
    cpsr: Psr,
    pipeline: Pipeline,
    cycles: u64,
    bus: B,
}

impl<B: Bus> Arm7tdmi<B> {
    const PC: usize = 15;
    const LR: usize = 14;
    const SP: usize = 13;

    pub fn new(bus: B) -> Self {
        Self {
            reg: [0; 16],
            bank: Bank::default(),
            cpsr: Psr::default(),
            pipeline: Pipeline::default(),
            cycles: 0,
            bus,
        }
    }

    pub fn reset(&mut self) {
        self.handle_exception(Exception::Reset);
    }

    pub fn step(&mut self) {
        if let Some(instruction) = self.pipeline.take() {
            self.exec(instruction);
        }

        if self.pipeline.last_pc() != self.pc() {
            self.align_pc();
            self.pipeline.flush();
        }

        self.load_pipeline();
    }

    #[inline(always)]
    pub fn fetch(&mut self) -> u32 {
        let offset = self.instruction_size();
        let word = self.bus.read_word(self.pc());

        self.shift_pc(offset.into());
        word
    }

    #[inline(always)]
    pub fn decode(&self, word: u32) -> Instruction {
        // if self.cpsr.thumb() {
        Instruction::Thumb(self.decode_thumb(word))
        // } else {
        //     Instruction::Arm(self.decode_arm(word))
        // }
    }

    #[inline(always)]
    pub fn exec(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Thumb(op) => self.exec_thumb(op),
            Instruction::Arm(op) => self.exec_arm(op),
        }
    }

    #[inline(always)]
    fn pc(&self) -> u32 {
        self.reg[Self::PC]
    }

    #[inline(always)]
    fn set_pc(&mut self, value: u32) {
        self.reg[Self::PC] = value;
    }

    #[inline(always)]
    fn shift_pc(&mut self, offset: i32) {
        self.reg[Self::PC] = self.reg[Self::PC].wrapping_add_signed(offset);
    }

    fn align_pc(&mut self) {
        let mask = if self.cpsr.thumb() { !0b01 } else { !0b11 }; // half-word | word
        let value = self.pc() & mask;

        self.set_pc(value);
    }

    #[inline(always)]
    fn instruction_size(&self) -> u8 {
        if self.cpsr.thumb() { 2 } else { 4 }
    }

    #[inline(always)]
    fn sp(&self) -> u32 {
        self.get_reg(Self::SP)
    }

    #[inline(always)]
    pub fn set_sp(&mut self, value: u32) {
        *self.get_reg_mut(Self::SP) = value;
    }

    fn store_reg(&mut self, rs: usize, rb: usize, effect: RegisterFx) {
        let value = self.get_reg(rs);

        if matches!(effect, RegisterFx::IncB | RegisterFx::DecB) {
            self.update_reg(rb, effect);
        }

        self.bus.write_word(self.get_reg(rb), value);

        if matches!(effect, RegisterFx::IncA | RegisterFx::DecA) {
            self.update_reg(rb, effect);
        }
    }

    fn load_reg(&mut self, rd: usize, rb: usize, effect: RegisterFx) {
        if matches!(effect, RegisterFx::IncB | RegisterFx::DecB) {
            self.update_reg(rb, effect);
        }

        let addr = self.get_reg(rb);
        let value = self.bus.read_word(addr);

        if matches!(effect, RegisterFx::IncA | RegisterFx::DecA) {
            self.update_reg(rb, effect);
        }

        self.set_reg(rd, value);
    }

    fn get_reg<I: Into<usize>>(&self, index: I) -> u32 {
        let index = index.into();
        let mode = self.cpsr.operating_mode();

        self.bank
            .get_reg(mode, index)
            .unwrap_or_else(|| self.reg[index])
    }

    fn get_reg_mut<I: Into<usize>>(&mut self, index: I) -> &mut u32 {
        let index = index.into();
        let mode = self.cpsr.operating_mode();

        self.bank
            .get_reg_mut(mode, index)
            .unwrap_or_else(|| &mut self.reg[index])
    }

    #[inline(always)]
    fn set_reg<I: Into<usize>>(&mut self, index: I, value: u32) {
        *self.get_reg_mut(index) = value;
    }

    #[inline(always)]
    fn increment_reg<I: Into<usize>>(&mut self, register: I) {
        *self.get_reg_mut(register) += 4;
    }

    #[inline(always)]
    fn decrement_reg<I: Into<usize>>(&mut self, register: I) {
        *self.get_reg_mut(register) -= 4;
    }

    fn update_reg(&mut self, rn: usize, effect: RegisterFx) {
        match effect {
            RegisterFx::IncB | RegisterFx::IncA => self.increment_reg(rn),
            RegisterFx::DecB | RegisterFx::DecA => self.decrement_reg(rn),
        }
    }

    fn get_operand(&self, operand: Operand) -> u32 {
        let value = match operand.kind {
            OperandKind::Imm => operand.value,
            _ => self.get_reg(operand.value as usize),
        };

        if operand.negate { !value } else { value }
    }
}

#[cfg(test)]
impl<B: Bus> Arm7tdmi<B> {
    pub fn force_thumb_mode(&mut self) {
        self.cpsr.set_thumb_mode();
        self.set_pc(0x00);
        self.pipeline.flush();
        self.load_pipeline();
    }

    pub fn assert_mem(&self, assertions: Vec<(u32, u32, DataType)>) {
        for (address, expected, data_type) in assertions {
            let value = match data_type {
                DataType::Byte => self.bus.read_byte(address).into(),
                DataType::HWord => self.bus.read_hword(address).into(),
                DataType::Word => self.bus.read_word(address),
            };

            assert_eq!(
                value, expected,
                "expected 0x{expected:x} at @{address:x}, got 0x{value:x}"
            )
        }
    }

    pub fn assert_reg(&self, assertions: Vec<(usize, u32)>) {
        for (index, expected) in assertions {
            let value = self.get_reg(index);

            assert_eq!(
                value, expected,
                "expected 0x{expected:x} at R{index}, got 0x{value:x}"
            )
        }
    }

    pub fn assert_flag(&self, assertions: Vec<(u32, bool)>) {
        for (flag, expected) in assertions {
            let value = self.cpsr.has(flag);
            let name = Psr::format_flag(flag);
            let status = if expected { "set" } else { "cleared" };

            assert_eq!(
                value, expected,
                "expected flag {name} to be {status}, flags: {:?}",
                self.cpsr
            )
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub use common::DataType;
    pub use psr::Psr;
}

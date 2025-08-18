mod bank;
mod common;
mod isa;
mod ops;
mod pipeline;
mod psr;
mod thumb;

use bank::Bank;
use common::{AddressMove, Operand, OperandKind};
use pipeline::Pipeline;
use psr::{Exception, Psr};

#[cfg(test)]
use common::DataType;

use crate::bus::Bus;

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
        let instruction = self.pipeline.next();

        if self.cpsr.has(Psr::T) {
            let decoded = self.decode_thumb(instruction);
            self.pre_fetch();
            self.exec_thumb(decoded); // PC is PC + 4
        } else {
            todo!()
        }

        if self.pipeline.last_pc() != self.pc() {
            self.reload_pipeline();
        }
    }

    #[inline(always)]
    pub fn fetch(&mut self) -> u32 {
        self.bus.read_word(self.pc())
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
    fn increment_pc(&mut self) {
        self.reg[Self::PC] += if self.cpsr.has(Psr::T) { 2 } else { 4 };
    }

    fn align_pc(&mut self) {
        let mask = if self.cpsr.has(Psr::T) { !0b01 } else { !0b11 }; // half-word | word
        let value = self.pc() & mask;

        self.set_pc(value);
    }

    #[inline(always)]
    fn sp(&self) -> u32 {
        self.get_reg(Self::SP)
    }

    #[inline(always)]
    pub fn set_sp(&mut self, value: u32) {
        *self.get_reg_mut(Self::SP) = value;
    }

    #[inline(always)]
    fn store_reg(&mut self, rs: usize, rb: usize, direction: AddressMove) {
        let value = self.get_reg(rs);

        match direction {
            AddressMove::Up => {
                self.bus.write_word(self.get_reg(rb), value);
                self.increment_reg(rb);
            }
            AddressMove::Down => {
                self.decrement_reg(rb);
                self.bus.write_word(self.get_reg(rb), value);
            }
        }
    }

    #[inline(always)]
    fn load_reg(&mut self, rd: usize, rb: usize) {
        let addr = self.get_reg(rb);
        let value = self.bus.read_word(addr);

        self.increment_reg(rb);
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
        self.reload_pipeline();
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

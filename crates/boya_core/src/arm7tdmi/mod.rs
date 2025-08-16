mod bank;
mod common;
mod isa;
mod ops;
mod pipeline;
mod psr;
mod thumb;

use bank::Bank;
use common::{Operand, OperandKind};
use pipeline::Pipeline;
use psr::{Exception, Psr};

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
        self.bus.read_u32(self.pc())
    }

    #[inline(always)]
    fn sp(&self) -> u32 {
        self.get_reg(Self::SP)
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

    fn get_reg<I>(&self, index: I) -> u32
    where
        I: Into<usize> + Copy,
    {
        let mode = self.cpsr.operating_mode();

        self.bank
            .get_reg(mode, index.into())
            .unwrap_or_else(|| self.reg[index.into()])
    }

    fn get_reg_mut<I>(&mut self, index: I) -> &mut u32
    where
        I: Into<usize> + Copy,
    {
        let mode = self.cpsr.operating_mode();

        self.bank
            .get_reg_mut(mode, index.into())
            .unwrap_or_else(|| &mut self.reg[index.into()])
    }

    #[inline(always)]
    fn set_reg<I>(&mut self, index: I, value: u32)
    where
        I: Into<usize> + Copy,
    {
        *self.get_reg_mut(index) = value;
    }

    fn get_operand(&self, operand: Operand) -> u32 {
        let value = match operand.kind {
            OperandKind::Immediate => operand.value,
            OperandKind::Register => self.get_reg(operand.value as usize),
        };

        if operand.negate {
            !value
        } else {
            value
        }
    }
}

#[cfg(test)]
impl<B: Bus> Arm7tdmi<B> {
    pub fn force_thumb_mode(&mut self) {
        self.cpsr.set_thumb_mode();
        self.set_pc(0x00);
        self.reload_pipeline();
    }

    pub fn assert_mem(&self, assertions: Vec<(u32, u32)>) {
        for (address, expected) in assertions {
            let value = self.bus.read_u32(address);

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
pub mod utils {
    pub use super::psr::Psr;
}

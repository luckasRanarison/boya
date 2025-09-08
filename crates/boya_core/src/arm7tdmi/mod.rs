mod arm;
mod bank;
mod common;
mod isa;
mod ops;
mod pipeline;
mod psr;
mod thumb;

use bank::Bank;
use common::{AddrMode, Operand, OperandKind};
use pipeline::Pipeline;
use psr::Psr;

use crate::{
    arm7tdmi::{
        common::{Cycle, Exception, NamedRegister, Shift},
        isa::Instruction,
    },
    bus::{Bus, GbaBus},
    utils::bitflags::BitIter,
};

pub struct Arm7tdmi {
    pub reg: [u32; 16], // R0-R15
    pub bank: Bank,
    pub cpsr: Psr,
    pub pipeline: Pipeline,
    pub cycles: u64,
    pub bus: GbaBus,
}

impl Arm7tdmi {
    const PC: usize = 15;
    const LR: usize = 14;
    const SP: usize = 13;

    pub fn new(bus: GbaBus) -> Self {
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

    pub fn step(&mut self) -> u8 {
        let instruction = self.pipeline.take();
        let cycle = self.exec(instruction);

        if self.pipeline.last_pc() != self.pc() {
            self.align_pc();
            self.pipeline.flush();
        }

        self.load_pipeline();
        cycle.count()
    }

    #[inline(always)]
    pub fn fetch(&mut self) -> u32 {
        let offset = self.instr_size();
        let word = self.bus.read_word(self.pc());

        self.shift_pc(offset.into());
        word
    }

    #[inline(always)]
    pub fn decode(&self, word: u32) -> Instruction {
        if self.cpsr.thumb() {
            Instruction::Thumb(self.decode_thumb(word))
        } else {
            Instruction::Arm(self.decode_arm(word))
        }
    }

    #[inline(always)]
    pub fn exec(&mut self, instruction: Instruction) -> Cycle {
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
    fn instr_size(&self) -> u8 {
        if self.cpsr.thumb() { 2 } else { 4 }
    }

    fn count_rlist(&self, rlist: u16) -> u8 {
        rlist.iter_lsb().filter(|(_, bit)| *bit == 1).count() as u8
    }

    fn get_lowest_address(&self, rb: usize, n: u8, amod: AddrMode) -> u32 {
        let base = self.get_reg(rb);

        match amod {
            AddrMode::IA => base,
            AddrMode::IB => base + 4,
            AddrMode::DA => base - 4 * n.saturating_sub(1) as u32,
            AddrMode::DB => base - 4 * n as u32,
        }
    }

    fn write_base_address(&mut self, rb: usize, n: u8, amod: AddrMode) {
        let base = self.get_reg(rb);

        let value = match amod {
            AddrMode::IA | AddrMode::IB if n == 0 => base + 64,
            AddrMode::DA | AddrMode::DB if n == 0 => base - 64,
            AddrMode::IA | AddrMode::IB => base + 4 * n as u32,
            AddrMode::DA | AddrMode::DB => base - 4 * n as u32,
        };

        self.set_reg(rb, value);
    }

    fn store_reg(&mut self, rs: usize, addr: &mut u32, usr: bool) {
        let value = match rs == NamedRegister::PC as usize {
            true => self.pc() + 4, // arm 11
            false if usr => self.reg[rs],
            _ => self.get_reg(rs),
        };

        self.bus.write_word(*addr, value);
        *addr += 4;
    }

    fn load_reg(&mut self, rd: usize, offset: &mut u32, usr: bool) {
        let value = self.bus.read_word(*offset);

        if usr {
            self.reg[rd] = value
        } else {
            self.set_reg(rd, value);
        }

        *offset += 4;
    }

    pub fn get_reg<I: Into<usize>>(&self, index: I) -> u32 {
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

    fn get_base_operand(&self, operand: &Operand) -> u32 {
        match operand.kind {
            OperandKind::Imm => operand.value,
            _ => self.get_reg(operand.value as usize),
        }
    }

    fn get_operand_with_shift(&mut self, operand: Operand, update: bool) -> u32 {
        let value = match operand.shift.clone() {
            Some(shift) => self.apply_operand_shift(&operand, shift, update),
            None => self.get_base_operand(&operand),
        };

        if operand.negate { !value } else { value }
    }

    fn get_operand(&mut self, operand: Operand) -> u32 {
        self.get_operand_with_shift(operand, false)
    }

    fn apply_operand_shift(&mut self, operand: &Operand, shift: Shift, update: bool) -> u32 {
        let lhs = match operand.is_pc() && shift.register {
            true => self.pc() + 4,
            false => self.get_base_operand(operand),
        };

        let rhs = match shift.register {
            true => self.get_reg(shift.value),
            false => shift.value.into(),
        };

        self.apply_shift(lhs, rhs, shift.kind, !shift.register, update)
    }

    fn restore_cpsr(&mut self) {
        let op_mode = self.cpsr.operating_mode();

        if let Some(spsr) = self.bank.get_spsr(op_mode) {
            self.cpsr = spsr;
        }
    }
}

#[cfg(test)]
impl Arm7tdmi {
    pub fn set_sp(&mut self, value: u32) {
        *self.get_reg_mut(Self::SP) = value;
    }

    pub fn assert_mem(&self, assertions: Vec<(u32, u32, common::DataType)>) {
        for (address, expected, data_type) in assertions {
            let value = match data_type {
                common::DataType::Byte => self.bus.read_byte(address).into(),
                common::DataType::HWord => self.bus.read_hword(address).into(),
                common::DataType::Word => self.bus.read_word(address),
            };

            assert_eq!(
                value, expected,
                "expected {expected:#x} at {address:#x}, got {value:#x}"
            )
        }
    }

    pub fn assert_reg(&self, assertions: Vec<(usize, u32)>) {
        for (index, expected) in assertions {
            let value = self.get_reg(index);

            assert_eq!(
                value, expected,
                "expected {expected:#x} at R{index}, got {value:#x}"
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

    pub use common::{DataType, OperatingMode};
}

pub mod arm;
pub mod common;
pub mod isa;
pub mod ops;
pub mod pipeline;
pub mod psr;
pub mod register;
pub mod thumb;

use common::{AddrMode, Operand, OperandKind};
use pipeline::Pipeline;
use psr::Psr;

use crate::{
    bus::{
        Bus, GbaBus,
        types::{Cycle, DataType, InterruptResult, MemoryAccess},
    },
    cpu::{
        common::{Exception, NamedRegister, Shift},
        isa::Instruction,
        register::Register,
    },
    debug::types::InstructionResult,
    utils::bitflags::BitIter,
};

#[derive(Default)]
pub struct Arm7tdmi {
    pub registers: Register,
    pub cpsr: Psr,
    pub pipeline: Pipeline,
    pub bus: GbaBus,
}

impl Arm7tdmi {
    pub fn new(bus: GbaBus) -> Self {
        Self {
            registers: Register::default(),
            cpsr: Psr::default(),
            pipeline: Pipeline::default(),
            bus,
        }
    }

    pub fn step(&mut self) -> Cycle {
        let instruction = self.pipeline.take();
        let cycles = self.exec(instruction);

        self.sync_pipeline();

        cycles
    }

    #[inline]
    pub fn fetch(&mut self) -> u32 {
        let offset = self.instr_size();
        let word = self.bus.read_word(self.pc());

        self.registers.shift_pc(offset.into());
        word
    }

    #[inline]
    pub fn decode(&self, word: u32) -> Instruction {
        if self.cpsr.thumb() {
            Instruction::Thumb(self.decode_thumb(word))
        } else {
            Instruction::Arm(self.decode_arm(word))
        }
    }

    #[inline]
    pub fn exec(&mut self, instruction: Instruction) -> Cycle {
        match instruction {
            Instruction::Thumb(op) => self.exec_thumb(op),
            Instruction::Arm(op) => self.exec_arm(op),
        }
    }

    pub fn next_op_address(&self) -> Option<u32> {
        let last_pc = self.pipeline.next_address();
        let instr_size = self.instr_size().into();

        last_pc.checked_sub(instr_size)
    }

    pub fn try_irq(&mut self) -> Option<InterruptResult> {
        if !self.cpsr.has(Psr::I) && self.bus.io.has_pending_irq() {
            Some(InterruptResult {
                cycles: self.handle_exception(Exception::NormalInterrupt),
            })
        } else {
            None
        }
    }

    pub fn lr(&self) -> u32 {
        self.registers.get(Register::LR, self.cpsr.op_mode())
    }

    pub fn sp(&self) -> u32 {
        self.registers.get(Register::SP, self.cpsr.op_mode())
    }

    #[inline(always)]
    fn pc(&self) -> u32 {
        self.registers.pc()
    }

    fn align_pc(&mut self) {
        let mask = if self.cpsr.thumb() { !0b01 } else { !0b11 }; // half-word | word
        let value = self.pc() & mask;

        self.registers.set_pc(value);
    }

    fn pre_fetch_cycle(&self, access_kind: MemoryAccess) -> Cycle {
        let dt = match self.cpsr.thumb() {
            true => DataType::HWord,
            false => DataType::Word,
        };

        self.bus.rw_cycle(self.pc(), dt, access_kind)
    }

    #[inline(always)]
    fn instr_size(&self) -> u8 {
        if self.cpsr.thumb() { 2 } else { 4 }
    }

    fn count_rlist(&self, rlist: u16) -> u8 {
        rlist.iter_lsb().filter(|(_, bit)| *bit == 1).count() as u8
    }

    fn get_lowest_address(&self, rb: usize, n: u8, amod: AddrMode) -> u32 {
        let base = self.registers.get(rb, self.cpsr.op_mode());

        match amod {
            AddrMode::IA => base,
            AddrMode::IB => base + 4,
            AddrMode::DA => base - 4 * n.saturating_sub(1) as u32,
            AddrMode::DB => base - 4 * n as u32,
        }
    }

    fn write_base_address(&mut self, rb: usize, n: u8, amod: AddrMode) {
        let op_mode = self.cpsr.op_mode();
        let base = self.registers.get(rb, op_mode);

        let value = match amod {
            AddrMode::IA | AddrMode::IB if n == 0 => base + 64,
            AddrMode::DA | AddrMode::DB if n == 0 => base - 64,
            AddrMode::IA | AddrMode::IB => base + 4 * n as u32,
            AddrMode::DA | AddrMode::DB => base - 4 * n as u32,
        };

        self.registers.set(rb, value, op_mode);
    }

    fn store_reg(&mut self, rs: usize, addr: &mut u32, usr: bool) {
        let value = match rs == NamedRegister::PC as usize {
            true => self.pc() + 4, // arm 11
            false if usr => self.registers.main[rs],
            _ => self.registers.get(rs, self.cpsr.op_mode()),
        };

        self.bus.write_word(*addr, value);
        *addr += 4;
    }

    fn load_reg(&mut self, rd: usize, offset: &mut u32, usr: bool) {
        let value = self.bus.read_word(*offset);

        if usr {
            self.registers.main[rd] = value
        } else {
            self.registers.set(rd, value, self.cpsr.op_mode());
        }

        *offset += 4;
    }

    fn get_base_operand(&self, operand: &Operand) -> u32 {
        match operand.kind {
            OperandKind::Imm => operand.value,
            _ => self
                .registers
                .get(operand.value as usize, self.cpsr.op_mode()),
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
            true => self.registers.get(shift.value, self.cpsr.op_mode()),
            false => shift.value.into(),
        };

        self.apply_shift(lhs, rhs, shift.kind, !shift.register, update)
    }

    fn restore_cpsr(&mut self) {
        let op_mode = self.cpsr.op_mode();

        if let Some(spsr) = self.registers.get_spsr(op_mode) {
            self.cpsr = spsr;
        }
    }

    pub fn debug_step(&mut self) -> InstructionResult {
        let instruction = self.pipeline.take();
        let data = instruction.get_data();
        let cycles = self.exec(instruction);

        self.sync_pipeline();

        InstructionResult { data, cycles }
    }
}

#[cfg(test)]
impl Arm7tdmi {
    pub fn override_pc(&mut self, value: u32) {
        self.registers.set_pc(value);
        self.pipeline.flush();
        self.load_pipeline();
    }
}

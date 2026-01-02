use crate::{
    bus::{
        Bus,
        types::{Cycle, DataType, MemoryAccess},
    },
    cpu::{common::*, psr::PsrKind, register::Register},
    utils::{
        bitflags::{BitIter, Bitflag},
        ops::ExtendedOps,
    },
};

use super::{
    Arm7tdmi,
    common::{AddrMode, Carry, Operand},
    psr::Psr,
};

impl Arm7tdmi {
    #[inline(always)]
    pub fn add_sub_op(
        &mut self,
        dst: Option<u8>,
        lhs: Operand,
        rhs: Operand,
        carry: Carry,
        update: bool,
    ) -> Cycle {
        let (cycle_op, pc_dst) = self.data_op_cycle(dst, &rhs);
        let reg_shift = rhs.shift.as_ref().filter(|s| s.register).is_some();

        let lhs = match lhs.is_pc() {
            true if self.cpsr.thumb() && rhs.is_imm() => self.pc() & !2, // thumb 12
            true if !self.cpsr.thumb() && reg_shift => self.pc() + 4,    // arm 5
            _ => self.get_operand_with_shift(lhs, update),
        };

        let carry = match carry {
            Carry::One => 1,
            Carry::None => 0,
            Carry::Flag => self.cpsr.get(Psr::C),
        };

        let rhs = self.get_operand(rhs);
        let (res1, ovf1) = lhs.overflowing_add(rhs);
        let (res2, ovf2) = res1.overflowing_add(carry);
        let overflow = ((res2 ^ lhs) & (res2 ^ rhs)).has(31);

        if update {
            self.cpsr.update_zn(res2);
            self.cpsr.update(Psr::C, ovf1 || ovf2);
            self.cpsr.update(Psr::V, overflow);
        }

        if let Some(rd) = dst {
            self.registers.set(rd, res2, self.cpsr.op_mode());
        }

        let extra_fetch_cycle = self.extra_fetch_cycle(pc_dst);

        cycle_op + extra_fetch_cycle
    }

    #[inline(always)]
    pub fn shift_op(&mut self, dst: u8, lhs: u8, rhs: Operand, shift: ShiftKind) -> Cycle {
        let op_mode = self.cpsr.op_mode();
        let (op_cycle, pc_dst) = self.data_op_cycle(dst.into(), &rhs);
        let imm = rhs.is_imm();
        let lhs = self.registers.get(lhs, op_mode);
        let rhs = self.get_operand(rhs) & 0xFF;
        let result = self.apply_shift(lhs, rhs, shift, imm, true);

        self.cpsr.update_zn(result);
        self.registers.set(dst, result, op_mode);

        let extra_fetch_cycle = self.extra_fetch_cycle(pc_dst);

        op_cycle + extra_fetch_cycle
    }

    #[inline(always)]
    pub fn logical_op<F>(
        &mut self,
        func: F,
        dst: Option<u8>,
        lhs: u8,
        rhs: Operand,
        update: bool,
    ) -> Cycle
    where
        F: Fn(u32, u32) -> u32,
    {
        let op_mode = self.cpsr.op_mode();
        let (op_cycle, pc_dst) = self.data_op_cycle(dst, &rhs);
        let lhs = self.registers.get(lhs, op_mode);
        let rhs = self.get_operand_with_shift(rhs, update);
        let result = func(lhs, rhs);

        if update {
            self.cpsr.update_zn(result);
        }

        if let Some(rd) = dst {
            self.registers.set(rd, result, op_mode);
        }

        let extra_fetch_cycle = self.extra_fetch_cycle(pc_dst);

        op_cycle + extra_fetch_cycle
    }

    #[inline(always)]
    pub fn mov_op(&mut self, rd: u8, operand: Operand, update: bool) -> Cycle {
        let (op_cycle, pc_dst) = self.data_op_cycle(rd.into(), &operand);
        let value = self.get_operand_with_shift(operand, update);

        if update {
            self.cpsr.update_zn(value);
        }

        self.registers.set(rd, value, self.cpsr.op_mode());

        let extra_fetch_cycle = self.extra_fetch_cycle(pc_dst);

        op_cycle + extra_fetch_cycle
    }

    #[inline(always)]
    pub fn bx_op(&mut self, rs: u8) -> Cycle {
        let value = self.registers.get(rs, self.cpsr.op_mode());
        let prev_mode = self.cpsr.thumb();
        let first_cycle = self.pre_fetch_cycle(MemoryAccess::NonSeq);

        self.cpsr.update(Psr::T, value.has(0));
        self.registers.set_pc(value);

        if prev_mode != self.cpsr.thumb() {
            self.pipeline.flush();
        }

        let fetch_cycle = self.pre_fetch_cycle(MemoryAccess::Seq);

        first_cycle + fetch_cycle.repeat(2)
    }

    #[inline(always)]
    pub fn mul_op(
        &mut self,
        dst: LongOperand,
        lhs: u8,
        rhs: u8,
        acc: Option<LongOperand>,
        update: bool,
        signed: bool,
    ) -> Cycle {
        let op_mode = self.cpsr.op_mode();
        let lhs = self.registers.get(lhs, op_mode);
        let rhs = self.registers.get(rhs, op_mode);
        let pre_fetch_cycle = self.pre_fetch_cycle(MemoryAccess::Seq);

        let i_base = match rhs {
            _ if rhs.get_bits(24, 31) != 0 => 4,
            _ if rhs.get_bits(16, 23) != 0 => 3,
            _ if rhs.get_bits(8, 15) != 0 => 2,
            _ => 1,
        };

        let (acc, i_extra) = acc.map_or((0, 0), |reg| {
            let hi = reg.hi.map_or(0, |hi| self.registers.get(hi, op_mode)) as u64;
            let lo = self.registers.get(reg.lo, op_mode) as u64;
            let i = if reg.hi.is_some() { 2 } else { 1 };
            ((hi << 32) | lo, i)
        });

        let (lhs, rhs) = if signed {
            let lhs = lhs as i32 as i64 as u64;
            let rhs = rhs as i32 as i64 as u64;
            (lhs, rhs)
        } else {
            (lhs as u64, rhs as u64)
        };

        let res = lhs.wrapping_mul(rhs).wrapping_add(acc);
        let res_hi = res.get_bits(32, 63) as u32;
        let res_lo = res as u32;
        let i = i_base + i_extra;

        if update {
            self.cpsr.update(Psr::Z, res == 0);
            self.cpsr.update(Psr::N, res_hi.has(31));
            self.cpsr.update(Psr::C, false);
        }

        if let Some(hi) = dst.hi {
            self.registers.set(hi, res_hi, op_mode);
        }

        self.registers.set(dst.lo, res_lo, op_mode);

        let internal_cycle = Cycle::internal(i);

        pre_fetch_cycle + internal_cycle
    }

    #[inline(always)]
    pub fn ldr_op(
        &mut self,
        rd: u8,
        rn: u8,
        kind: DataType,
        signed: bool,
        offset: RegisterOffset,
    ) -> Cycle {
        let op_mode = self.cpsr.op_mode();

        let base = match rn.reg().is_pc() {
            true if self.cpsr.thumb() => self.pc() & !2, // thumb 6
            true => self.pc(),
            false => self.registers.get(rn, op_mode),
        };

        let addr = match offset.amod {
            AddrMode::IB => base.wrapping_add(offset.value),
            AddrMode::DB => base.wrapping_sub(offset.value),
            _ => base,
        };

        if offset.wb {
            self.registers.set(rn, addr, op_mode);
        }

        let value = match kind {
            DataType::HWord if signed => {
                (self.bus.read_hword(addr & !1) as i16 >> ((addr & 1) * 8)) as i32 as u32
            }
            DataType::Byte if signed => self.bus.read_byte(addr) as i8 as i32 as u32,
            DataType::Byte => self.bus.read_byte(addr).into(),
            DataType::HWord => (self.bus.read_hword(addr & !1) as u32).rotate_right((addr & 1) * 8),
            DataType::Word => self.bus.read_word(addr & !3).rotate_right((addr & 3) * 8),
        };

        match offset.amod {
            AddrMode::IA => self
                .registers
                .set(rn, base.wrapping_add(offset.value), op_mode),
            AddrMode::DA => self
                .registers
                .set(rn, base.wrapping_sub(offset.value), op_mode),
            _ => {}
        };

        let read_cycle = self.bus.rw_cycle(addr, kind, MemoryAccess::NonSeq);
        let internal_cycle = Cycle::internal(1);
        let pre_fetch_cycle = self.pre_fetch_cycle(MemoryAccess::Seq);

        self.registers.set(rd, value, op_mode);

        let extra_fetch_cycle = self.extra_fetch_cycle(rd == NamedRegister::PC as u8);

        read_cycle + internal_cycle + pre_fetch_cycle + extra_fetch_cycle
    }

    #[inline(always)]
    pub fn str_op(&mut self, rs: u8, rn: u8, kind: DataType, offset: RegisterOffset) -> Cycle {
        let op_mode = self.cpsr.op_mode();
        let base = self.registers.get(rn, op_mode);
        let fetch_cycle = self.pre_fetch_cycle(MemoryAccess::NonSeq);

        let value = match rs.reg().is_pc() {
            true if !self.cpsr.thumb() => self.pc() + 4, // arm 10
            _ => self.registers.get(rs, op_mode),
        };

        let addr = match offset.amod {
            AddrMode::IB => base.wrapping_add(offset.value),
            AddrMode::DB => base.wrapping_sub(offset.value),
            _ => base,
        };

        if offset.wb {
            self.registers.set(rn, addr, op_mode);
        }

        match kind {
            DataType::Byte => self.bus.write_byte(addr, (value & 0xFF) as u8),
            DataType::HWord => self.bus.write_hword(addr & !1, (value & 0xFFFF) as u16),
            DataType::Word => self.bus.write_word(addr & !3, value),
        }

        match offset.amod {
            AddrMode::IA => self
                .registers
                .set(rn, base.wrapping_add(offset.value), op_mode),
            AddrMode::DA => self
                .registers
                .set(rn, base.wrapping_sub(offset.value), op_mode),
            _ => {}
        };

        let write_cycle = self.bus.rw_cycle(addr, kind, MemoryAccess::NonSeq);

        fetch_cycle + write_cycle
    }

    #[inline(always)]
    pub fn stm_op(&mut self, rb: usize, rlist: u16, amod: AddrMode, wb: bool, usr: bool) -> Cycle {
        let op_mode = self.cpsr.op_mode();
        let n = self.count_rlist(rlist);
        let low_addr = self.get_lowest_address(rb, n, amod);
        let pre_fetch_cycle = self.pre_fetch_cycle(MemoryAccess::NonSeq);

        let mut offset = low_addr;
        let mut pre_write = false;
        let mut write_cycle = Cycle::default();

        // https://github.com/jsmolka/gba-tests/issues/2
        if n == 0 {
            let base = self.registers.get(rb, op_mode);
            let offset = match amod {
                AddrMode::DA => -0x3C,
                AddrMode::DB => -0x40,
                AddrMode::IA => 0x00,
                AddrMode::IB => 0x04,
            };
            let addr = base.wrapping_add_signed(offset);
            // TODO: figure why this is one instruction ahead
            let pc = self.pc() + self.instr_size() as u32;

            self.bus.write_word(addr, pc);
        }

        for (idx, bit) in rlist.iter_lsb() {
            if bit != 1 {
                continue;
            }

            if idx == rb && low_addr != offset {
                self.write_base_address(rb, n, amod);
                pre_write = true;
            }

            let access = match idx as u8 != n - 1 {
                true => MemoryAccess::Seq,
                false => MemoryAccess::NonSeq,
            };

            write_cycle += self.bus.rw_cycle(offset, DataType::Word, access);

            self.store_reg(idx, &mut offset, usr);
        }

        if wb && !pre_write {
            self.write_base_address(rb, n, amod);
        }

        pre_fetch_cycle + write_cycle
    }

    #[inline(always)]
    pub fn ldm_op(&mut self, rb: usize, rlist: u16, amod: AddrMode, wb: bool, usr: bool) -> Cycle {
        let op_mode = self.cpsr.op_mode();
        let n = self.count_rlist(rlist);
        let pre_fetch_cycle = self.pre_fetch_cycle(MemoryAccess::NonSeq);

        let mut pc_dst = false;
        let mut skip_write = false;
        let mut read_cycle = Cycle::default();

        // https://github.com/jsmolka/gba-tests/issues/12
        if n == 0 {
            let addr = self.registers.get(rb, op_mode);
            let word = self.bus.read_word(addr);
            self.registers.set_pc(word);
        }

        let mut offset = self.get_lowest_address(rb, n, amod);

        for (idx, bit) in rlist.iter_lsb() {
            if bit != 1 {
                continue;
            }

            if idx == rb {
                skip_write = true;
            }

            if idx == Register::PC {
                pc_dst = true;
            }

            read_cycle += self.bus.rw_cycle(offset, DataType::Word, MemoryAccess::Seq);

            self.load_reg(idx, &mut offset, usr);
        }

        let internal_cycle = Cycle::internal(1);

        if wb && !skip_write {
            self.write_base_address(rb, n, amod);
        }

        let extra_fetch_cycle = self.extra_fetch_cycle(pc_dst);

        // FIXME: not quite accurate
        pre_fetch_cycle + read_cycle + internal_cycle + extra_fetch_cycle
    }

    #[inline(always)]
    pub fn branch_op(&mut self, cond: Condition, offset: i32) -> Cycle {
        let first_cycle = self.pre_fetch_cycle(MemoryAccess::NonSeq);

        if !self.cpsr.matches(cond) {
            return first_cycle;
        }

        if offset != 0 {
            self.registers.shift_pc(offset);
        } else {
            self.pipeline.flush();
        }

        let extra_cycle = self.pre_fetch_cycle(MemoryAccess::Seq);

        first_cycle + extra_cycle.repeat(2)
    }

    pub fn branch_long_first_op(&mut self, nn: u16) -> Cycle {
        let op_mode = self.cpsr.op_mode();
        let nn = ((nn as i32) << 21) >> 21; // sign-extend 11 bits
        let upper = (nn as u32) << 12;
        let result = self.pc().wrapping_add(upper);

        self.registers.set(Register::LR, result, op_mode);
        self.pre_fetch_cycle(MemoryAccess::Seq)
    }

    pub fn branch_long_second_op(&mut self, nn: u16) -> Cycle {
        let op_mode = self.cpsr.op_mode();
        let lower = (nn as u32) << 1;
        let lr = self.registers.get(Register::LR, op_mode) as i32;
        let offset = lr.wrapping_add(lower as i32);
        let lr = self.next_op_address().unwrap_or_default() | 1;
        let first_cycle = self.pre_fetch_cycle(MemoryAccess::NonSeq);

        self.registers.set_pc(offset as u32);
        self.registers.set(Register::LR, lr, op_mode);
        self.pipeline.flush();

        let extra_cycle = self.pre_fetch_cycle(MemoryAccess::Seq);

        first_cycle + extra_cycle.repeat(2)
    }

    pub fn handle_exception(&mut self, exception: Exception) -> Cycle {
        let vector = exception.vector();
        let op_mode = exception.operating_mode();
        let irq = exception.disable_irq() || self.cpsr.has(Psr::I);
        let fiq = exception.disable_fiq() || self.cpsr.has(Psr::F);
        let first_cycle = self.pre_fetch_cycle(MemoryAccess::NonSeq);

        self.cpsr.set_operating_mode(op_mode);
        self.registers.set_spsr(op_mode, self.cpsr);

        if let Some(next_addr) = self.next_op_address() {
            self.registers.set(Register::LR, next_addr, op_mode);
        }

        self.cpsr.update(Psr::T, false);
        self.cpsr.update(Psr::I, irq);
        self.cpsr.update(Psr::F, fiq);

        self.registers.set_pc(vector);
        self.load_pipeline();

        let extra_cycle = self.pre_fetch_cycle(MemoryAccess::Seq);

        first_cycle + extra_cycle.repeat(2)
    }

    #[inline(always)]
    pub fn store_psr_op(&mut self, rd: u8, kind: PsrKind) -> Cycle {
        let op_mode = self.cpsr.op_mode();

        let psr = match kind {
            PsrKind::CPSR => self.cpsr,
            PsrKind::SPSR => self.registers.get_spsr_unchecked(op_mode),
        };

        self.registers.set(rd, psr.value(), op_mode);
        self.pre_fetch_cycle(MemoryAccess::Seq)
    }

    #[inline(always)]
    pub fn update_psr_op(&mut self, op: Operand, mask: u32, kind: PsrKind) -> Cycle {
        let op_mode = self.cpsr.op_mode();
        let value = self.get_operand(op) & mask;

        match kind {
            PsrKind::CPSR => self.cpsr = Psr::from((self.cpsr.value() & !mask) | value),
            PsrKind::SPSR => self.registers.update_spsr(op_mode, value, mask),
        }

        self.pre_fetch_cycle(MemoryAccess::Seq)
    }

    #[inline(always)]
    pub fn swap_op(&mut self, rd: u8, rm: u8, rn: u8, byte: bool) -> Cycle {
        let op_mode = self.cpsr.op_mode();
        let addr = self.registers.get(rn, op_mode);

        if byte {
            let byte = self.bus.read_byte(addr);
            let reg = self.registers.get(rm, op_mode);

            self.registers.set(rd, byte.into(), op_mode);
            self.bus.write_byte(addr, reg as u8);
        } else {
            let word = self.bus.read_word(addr & !3);
            let value = word.rotate_right((addr & 3) * 8);
            let reg = self.registers.get(rm, op_mode);

            self.bus.write_word(addr & !3, reg);
            self.registers.set(rd, value, op_mode);
        }

        let dt = if byte { DataType::Byte } else { DataType::Word };
        let rw_cycle = self.bus.rw_cycle(addr, dt, MemoryAccess::NonSeq);
        let internal_cycle = Cycle::internal(1);
        let fetch_cycle = self.pre_fetch_cycle(MemoryAccess::Seq);

        rw_cycle.repeat(2) + internal_cycle + fetch_cycle
    }

    pub fn apply_shift(
        &mut self,
        lhs: u32,
        rhs: u32,
        shift: ShiftKind,
        imm: bool,
        update: bool,
    ) -> u32 {
        let (result, carry) = match (shift, rhs) {
            (ShiftKind::LSL, 33..) if update => (0, CarryUpdate::Clear),
            (ShiftKind::LSL, 32) => (0, CarryUpdate::Bit(0)),
            (ShiftKind::LSL, 1..) => (lhs.wrapping_shl(rhs), CarryUpdate::Bit(32 - (rhs & 31))),
            (ShiftKind::LSL, 0) if imm => (lhs, CarryUpdate::Unchanged),

            (ShiftKind::LSR, 33..) if update => (0, CarryUpdate::Clear),
            (ShiftKind::LSR, 32) => (0, CarryUpdate::Bit(31)),
            (ShiftKind::LSR, 1..) => (lhs.wrapping_shr(rhs), CarryUpdate::Bit(rhs - 1)),
            (ShiftKind::LSR, 0) if imm => (0, CarryUpdate::Bit(31)),

            (ShiftKind::ASR, 32) => (lhs.extended_asr(rhs), CarryUpdate::Bit(31)),
            (ShiftKind::ASR, 1..) => (lhs.extended_asr(rhs), CarryUpdate::Bit(rhs - 1)),
            (ShiftKind::ASR, 0) if imm => (lhs.extended_asr(31), CarryUpdate::Bit(31)),

            (ShiftKind::ROR, 1..) => (lhs.rotate_right(rhs), CarryUpdate::Bit(rhs - 1)),
            (ShiftKind::ROR, 0) if imm => (
                (self.cpsr.get(Psr::C) << 31) | (lhs >> 1),
                CarryUpdate::Bit(0),
            ),

            (_, 0) => (lhs, CarryUpdate::Unchanged),
        };

        if update {
            match carry {
                CarryUpdate::Clear => self.cpsr.update(Psr::C, false),
                CarryUpdate::Bit(bit) => self.cpsr.update(Psr::C, lhs.has(bit & 31)),
                _ => {}
            }
        }

        result
    }

    fn data_op_cycle(&self, dst: Option<u8>, operand: &Operand) -> (Cycle, bool) {
        let reg_shift = operand.shift.as_ref().filter(|s| s.register).is_some();
        let pc_dst = dst.filter(|r| *r == NamedRegister::PC as u8).is_some();

        let cycle = match (pc_dst, reg_shift) {
            (true, true) => self.pre_fetch_cycle(MemoryAccess::NonSeq) + Cycle::internal(1),
            (false, true) => self.pre_fetch_cycle(MemoryAccess::Seq) + Cycle::internal(1),
            (true, false) => self.pre_fetch_cycle(MemoryAccess::NonSeq),
            (false, false) => self.pre_fetch_cycle(MemoryAccess::Seq),
        };

        (cycle, pc_dst)
    }

    fn extra_fetch_cycle(&self, cond: bool) -> Cycle {
        if cond {
            self.pre_fetch_cycle(MemoryAccess::Seq).repeat(2)
        } else {
            Cycle::default()
        }
    }
}

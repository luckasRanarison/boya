use crate::{
    arm7tdmi::{common::*, psr::PsrKind},
    bus::Bus,
    common::types::DataType,
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
        let cycle = self.get_variable_cycle(dst, &rhs);
        let reg_shift = rhs.shift.as_ref().filter(|s| s.register).is_some();
        let thumb = self.cpsr.thumb();

        let lhs = match lhs.is_pc() {
            true if !thumb && reg_shift => self.pc() + 4, // arm 5
            true if thumb && rhs.is_imm() => self.pc() & !2, // thumb 12
            _ => self.get_operand_with_shift(lhs, update),
        };

        let carry = match carry {
            Carry::One => 1,
            Carry::None => 0,
            Carry::Flag => self.cpsr.get(Psr::C),
        };

        let rhs = self.get_operand(rhs);
        let (res1, ov1) = lhs.overflowing_add(rhs);
        let (res2, ov2) = res1.overflowing_add(carry);
        let overflow = ((res2 ^ lhs) & (res2 ^ rhs)).has(31);

        if update {
            self.cpsr.update_zn(res2);
            self.cpsr.update(Psr::C, ov1 || ov2);
            self.cpsr.update(Psr::V, overflow);
        }

        if let Some(rd) = dst {
            self.set_reg(rd, res2);
        }

        cycle
    }

    #[inline(always)]
    pub fn shift_op(&mut self, dst: u8, lhs: u8, rhs: Operand, shift: ShiftKind) -> Cycle {
        let i = if rhs.kind == OperandKind::Reg { 1 } else { 0 };
        let imm = rhs.is_imm();
        let lhs = self.get_reg(lhs);
        let rhs = self.get_operand(rhs) & 0xFF;
        let result = self.apply_shift(lhs, rhs, shift, imm, true);

        self.cpsr.update_zn(result);
        self.set_reg(dst, result);

        Cycle { i, s: 1, n: 0 }
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
        let cycle = self.get_variable_cycle(dst, &rhs);
        let lhs = self.get_reg(lhs);
        let rhs = self.get_operand_with_shift(rhs, update);
        let result = func(lhs, rhs);

        if update {
            self.cpsr.update_zn(result);
        }

        if let Some(rd) = dst {
            self.set_reg(rd, result);
        }

        cycle
    }

    #[inline(always)]
    pub fn mov_op(&mut self, rd: u8, operand: Operand, update: bool) -> Cycle {
        let cycle = self.get_variable_cycle(rd.into(), &operand);
        let value = self.get_operand_with_shift(operand, update);

        if update {
            self.cpsr.update_zn(value);
        }

        self.set_reg(rd, value);

        cycle
    }

    #[inline(always)]
    pub fn bx_op(&mut self, rs: u8) -> Cycle {
        let value = self.get_reg(rs);
        let prev_mode = self.cpsr.thumb();

        self.cpsr.update(Psr::T, value.has(0));
        self.set_pc(value);

        if prev_mode != self.cpsr.thumb() {
            self.pipeline.flush();
        }

        Cycle { i: 0, s: 2, n: 1 }
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
        let lhs = self.get_reg(lhs);
        let rhs = self.get_reg(rhs);

        let i_base = match rhs {
            _ if rhs.get_bits(24, 31) != 0 => 4,
            _ if rhs.get_bits(16, 23) != 0 => 3,
            _ if rhs.get_bits(8, 15) != 0 => 2,
            _ => 1,
        };

        let (acc, i_extra) = acc.map_or((0, 0), |reg| {
            let hi = reg.hi.map_or(0, |hi| self.get_reg(hi)) as u64;
            let lo = self.get_reg(reg.lo) as u64;
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
            self.set_reg(hi, res_hi);
        }

        self.set_reg(dst.lo, res_lo);

        Cycle { i, s: 1, n: 0 }
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
        let (base, s, n) = match rn.reg().is_pc() {
            true if self.cpsr.thumb() => (self.pc() & !2, 1, 1), // thumb 6
            true => (self.pc(), 2, 2),
            false => (self.get_reg(rn), 1, 1),
        };

        let addr = match offset.amod {
            AddrMode::IB => base.wrapping_add(offset.value),
            AddrMode::DB => base.wrapping_sub(offset.value),
            _ => base,
        };

        if offset.wb {
            self.set_reg(rn, addr);
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
            AddrMode::IA => self.set_reg(rn, base.wrapping_add(offset.value)),
            AddrMode::DA => self.set_reg(rn, base.wrapping_sub(offset.value)),
            _ => {}
        };

        self.set_reg(rd, value);

        Cycle { i: 1, s, n }
    }

    #[inline(always)]
    pub fn str_op(&mut self, rs: u8, rn: u8, kind: DataType, offset: RegisterOffset) -> Cycle {
        let base = self.get_reg(rn);

        let value = match rs.reg().is_pc() {
            true if !self.cpsr.thumb() => self.pc() + 4, // arm 10
            _ => self.get_reg(rs),
        };

        let addr = match offset.amod {
            AddrMode::IB => base.wrapping_add(offset.value),
            AddrMode::DB => base.wrapping_sub(offset.value),
            _ => base,
        };

        if offset.wb {
            self.set_reg(rn, addr);
        }

        match kind {
            DataType::Byte => self.bus.write_byte(addr, (value & 0xFF) as u8),
            DataType::HWord => self.bus.write_hword(addr & !1, (value & 0xFFFF) as u16),
            DataType::Word => self.bus.write_word(addr & !3, value),
        }

        match offset.amod {
            AddrMode::IA => self.set_reg(rn, base.wrapping_add(offset.value)),
            AddrMode::DA => self.set_reg(rn, base.wrapping_sub(offset.value)),
            _ => {}
        };

        Cycle { i: 0, s: 0, n: 2 }
    }

    #[inline(always)]
    pub fn stm_op(&mut self, rb: usize, rlist: u16, amod: AddrMode, wb: bool, usr: bool) -> Cycle {
        let n = self.count_rlist(rlist);
        let low_addr = self.get_lowest_address(rb, n, amod);
        let mut offset = low_addr;
        let mut pre_write = false;

        // https://github.com/jsmolka/gba-tests/issues/2
        if n == 0 {
            let offset = match amod {
                AddrMode::DA => -0x3C,
                AddrMode::DB => -0x40,
                AddrMode::IA => 0x00,
                AddrMode::IB => 0x04,
            };

            let base = self.get_reg(rb);
            let addr = base.wrapping_add_signed(offset);
            let pc = self.pipeline.curr_pc + self.instr_size() as u32 * 2; // FIXME: self.pc() ???

            self.bus.write_word(addr, pc);
        }

        for (idx, bit) in rlist.iter_lsb() {
            if bit == 1 {
                if idx == rb && low_addr != offset {
                    self.write_base_address(rb, n, amod);
                    pre_write = true;
                }

                self.store_reg(idx, &mut offset, usr);
            }
        }

        if wb && !pre_write {
            self.write_base_address(rb, n, amod);
        }

        Cycle { i: 1, s: n, n: 1 }
    }

    #[inline(always)]
    pub fn ldm_op(&mut self, rb: usize, rlist: u16, amod: AddrMode, wb: bool, usr: bool) -> Cycle {
        let n = self.count_rlist(rlist);
        let s = n.saturating_sub(1);
        let mut skip_write = false;

        // https://github.com/jsmolka/gba-tests/issues/12
        if n == 0 {
            let addr = self.get_reg(rb);
            let value = self.bus.read_word(addr);

            self.set_pc(value);
        }

        let mut offset = self.get_lowest_address(rb, n, amod);

        for (idx, bit) in rlist.iter_lsb() {
            if bit == 1 {
                if idx == rb {
                    skip_write = true;
                }

                self.load_reg(idx, &mut offset, usr);
            }
        }

        if wb && !skip_write {
            self.write_base_address(rb, n, amod);
        }

        Cycle { i: 0, s, n: 2 }
    }

    #[inline(always)]
    pub fn branch_op(&mut self, cond: Condition, offset: i32) -> Cycle {
        if !self.cpsr.matches(cond) {
            return Cycle { i: 0, s: 1, n: 0 };
        }

        if offset != 0 {
            self.shift_pc(offset);
        } else {
            self.pipeline.flush();
        }

        Cycle { i: 0, s: 2, n: 1 }
    }

    pub fn branch_long_first_op(&mut self, nn: u16) -> Cycle {
        let nn = ((nn as i32) << 21) >> 21; // sign-extend 11 bits
        let upper = (nn as u32) << 12;
        let result = self.pc().wrapping_add(upper);

        self.set_reg(Self::LR, result);

        Cycle { i: 0, s: 1, n: 0 }
    }

    pub fn branch_long_second_op(&mut self, nn: u16) -> Cycle {
        let lower = (nn as u32) << 1;
        let lr = self.get_reg(Self::LR) as i32;
        let offset = lr.wrapping_add(lower as i32);
        let lr = self.next_instr_addr().unwrap_or_default() | 1;

        self.set_pc(offset as u32);
        self.set_reg(Self::LR, lr);

        self.pipeline.flush();

        Cycle { i: 0, s: 2, n: 1 }
    }

    pub fn handle_exception(&mut self, exception: Exception) -> Cycle {
        let vector = exception.vector();
        let op_mode = exception.operating_mode();
        let irq = exception.disable_irq() || self.cpsr.has(Psr::I);
        let fiq = exception.disable_fiq() || self.cpsr.has(Psr::F);

        if let Some(next_addr) = self.next_instr_addr() {
            self.set_reg(Self::LR, next_addr);
        }

        self.bank.set_spsr(op_mode, self.cpsr);
        self.cpsr.set_operating_mode(op_mode);
        self.cpsr.update(Psr::T, false);
        self.cpsr.update(Psr::I, irq);
        self.cpsr.update(Psr::F, fiq);

        self.set_pc(vector);
        self.load_pipeline();

        Cycle { i: 0, s: 2, n: 1 }
    }

    #[inline(always)]
    pub fn store_psr_op(&mut self, rd: u8, kind: PsrKind) -> Cycle {
        let psr = match kind {
            PsrKind::CPSR => self.cpsr,
            PsrKind::SPSR => self.bank.get_spsr_unchecked(self.cpsr.operating_mode()),
        };

        self.set_reg(rd, psr.value());

        Cycle { i: 0, s: 1, n: 0 }
    }

    #[inline(always)]
    pub fn update_psr_op(&mut self, op: Operand, mask: u32, kind: PsrKind) -> Cycle {
        let value = self.get_operand(op) & mask;
        let op_mode = self.cpsr.operating_mode();

        match kind {
            PsrKind::CPSR => self.cpsr = Psr::from((self.cpsr.value() & !mask) | value),
            PsrKind::SPSR => self.bank.update_spsr(op_mode, value, mask),
        }

        Cycle { i: 0, s: 1, n: 0 }
    }

    #[inline(always)]
    pub fn swap_op(&mut self, rd: u8, rm: u8, rn: u8, byte: bool) -> Cycle {
        let addr = self.get_reg(rn);

        if byte {
            let byte = self.bus.read_byte(addr).into();
            self.set_reg(rd, byte);
            self.bus.write_byte(addr, self.get_reg(rm) as u8);
        } else {
            let value = self.bus.read_word(addr & !3);

            self.bus.write_word(addr & !3, self.get_reg(rm));
            self.set_reg(rd, value.rotate_right((addr & 3) * 8));
        }

        Cycle { i: 1, s: 1, n: 2 }
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

    fn get_variable_cycle(&self, dst: Option<u8>, operand: &Operand) -> Cycle {
        let reg_shift = operand.shift.as_ref().filter(|s| s.register).is_some();
        let pc_operand = dst.filter(|r| *r == NamedRegister::PC as u8).is_some();
        let r = if operand.is_reg() && reg_shift { 1 } else { 0 };
        let p = if pc_operand { 1 } else { 0 };

        Cycle {
            i: r,
            s: 1 + p,
            n: p,
        }
    }
}

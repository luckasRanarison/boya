use crate::{
    arm7tdmi::{
        common::{
            Condition, Cycle, Exception, LongOperand, OperandKind, OperatingMode, RegisterOffset,
            ToOperand,
        },
        psr::PsrKind,
    },
    bus::Bus,
    utils::bitflags::{BitIter, Bitflag},
};

use super::{
    Arm7tdmi,
    common::{Carry, DataType, Operand, RegisterFx},
    psr::Psr,
};

impl<B: Bus> Arm7tdmi<B> {
    #[inline(always)]
    pub fn add_sub_op(
        &mut self,
        dst: Option<u8>,
        lhs: Operand,
        rhs: Operand,
        carry: Carry,
        update: bool,
    ) -> Cycle {
        let (s, n) = self.get_sn_cycle(&lhs);
        let lhs = self.get_operand(lhs);
        let rhs = self.get_operand(rhs);

        let carry = match carry {
            Carry::One => 1,
            Carry::None => 0,
            Carry::Flag => self.cpsr.get(Psr::C),
        };

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

        Cycle { i: 0, s, n }
    }

    #[inline(always)]
    pub fn shift_op<F>(&mut self, func: F, dst: u8, lhs: u8, rhs: Operand) -> Cycle
    where
        F: Fn(u32, u32) -> u32,
    {
        let i = if rhs.kind == OperandKind::Reg { 1 } else { 0 };
        let lhs = self.get_reg(lhs);
        let rhs = self.get_operand(rhs) & 0xFF;
        let result = func(lhs, rhs);

        self.cpsr.update_zn(result);
        self.cpsr.update(Psr::C, rhs > 0);

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
        let (s, n) = self.get_sn_cycle(&lhs.reg());
        let lhs = self.get_reg(lhs);
        let rhs = self.get_operand(rhs);
        let result = func(lhs, rhs);

        if update {
            self.cpsr.update_zn(result);
        }

        if let Some(rd) = dst {
            self.set_reg(rd, result);
        }

        Cycle { i: 0, s, n }
    }

    #[inline(always)]
    pub fn mov_op(&mut self, rd: u8, operand: Operand, update: bool) -> Cycle {
        let (s, n) = self.get_sn_cycle(&rd.reg());
        let value = self.get_operand(operand);

        if update {
            self.cpsr.update_zn(value);
        }

        self.set_reg(rd, value);

        Cycle { i: 0, s, n }
    }

    #[inline(always)]
    pub fn bx_op(&mut self, rs: u8) -> Cycle {
        let value = self.get_reg(rs);

        self.cpsr.update(Psr::T, value.has(0));
        self.set_pc(value);

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
        let base = self.get_reg(rn);

        let addr = match offset.fx {
            RegisterFx::IncB => base.wrapping_add(offset.value),
            RegisterFx::DecB => base.wrapping_sub(offset.value),
            _ => base,
        };

        if offset.wb {
            self.set_reg(rn, addr);
        }

        let value = match kind {
            DataType::Byte if signed => self.bus.read_byte(addr) as i8 as i32 as u32,
            DataType::HWord if signed => self.bus.read_hword(addr) as i8 as i32 as u32,
            DataType::Byte => self.bus.read_byte(addr).into(),
            DataType::HWord => self.bus.read_hword(addr).into(),
            DataType::Word => self.bus.read_word(addr),
        };

        match offset.fx {
            RegisterFx::IncA => self.set_reg(rn, base.wrapping_add(offset.value)),
            RegisterFx::DecA => self.set_reg(rn, base.wrapping_sub(offset.value)),
            _ => {}
        };

        println!("set R{rd}: {value}, addr: {addr}, offset: {offset:?}");

        self.set_reg(rd, value);

        Cycle { i: 1, s: 1, n: 1 }
    }

    #[inline(always)]
    pub fn str_op(&mut self, rs: u8, rn: u8, kind: DataType, offset: RegisterOffset) -> Cycle {
        let base = self.get_reg(rn);
        let value = self.get_reg(rs);

        let addr = match offset.fx {
            RegisterFx::IncB => base.wrapping_add(offset.value),
            RegisterFx::DecB => base.wrapping_sub(offset.value),
            _ => base,
        };

        if offset.wb {
            self.set_reg(rn, addr);
        }

        match kind {
            DataType::Byte => self.bus.write_byte(addr, (value & 0xFF) as u8),
            DataType::HWord => self.bus.write_hword(addr, (value & 0xFFFF) as u16),
            DataType::Word => self.bus.write_word(addr, value),
        }

        match offset.fx {
            RegisterFx::IncA => self.set_reg(rn, base.wrapping_add(offset.value)),
            RegisterFx::DecA => self.set_reg(rn, base.wrapping_sub(offset.value)),
            _ => {}
        };

        Cycle { i: 2, s: 0, n: 0 }
    }

    #[inline(always)]
    pub fn stm_op<I: BitIter>(
        &mut self,
        rb: usize,
        rlist: I,
        rn: Option<usize>,
        effect: RegisterFx,
    ) -> Cycle {
        let mut s = 0;

        for (idx, bit) in rlist.iter_lsb() {
            if bit == 1 {
                self.store_reg(idx, rb, effect);
                s += 1;
            }
        }

        if let Some(rn) = rn {
            self.store_reg(rn, rb, effect);
            s += 1;
        }

        Cycle { i: 1, s, n: 1 }
    }

    #[inline(always)]
    pub fn ldm_op<I: BitIter>(
        &mut self,
        rb: usize,
        rlist: I,
        rn: Option<usize>,
        effect: RegisterFx,
    ) -> Cycle {
        let mut s = 0;

        for (idx, bit) in rlist.iter_lsb() {
            if bit == 1 {
                self.load_reg(idx, rb, effect);
                s += 1;
            }
        }

        if let Some(rn) = rn {
            self.load_reg(rn, rb, effect);
            s += 1;
        }

        Cycle {
            i: 0,
            s: s - 1,
            n: 2,
        }
    }

    #[inline(always)]
    pub fn branch_op(&mut self, cond: Condition, offset: i16) -> Cycle {
        if !self.cpsr.matches(cond) {
            return Cycle { i: 0, s: 1, n: 0 };
        }

        if offset != 0 {
            self.shift_pc(offset.into());
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
        let (op_mode, irq, fiq, vector) = match exception {
            Exception::Reset => (OperatingMode::SVC, true, true, 0x00),
            Exception::UndefinedInstruction => todo!(),
            Exception::SoftwareInterrupt => (OperatingMode::SVC, true, self.cpsr.has(Psr::F), 0x08),
            Exception::PrefetchAbort => todo!(),
            Exception::DataAbort => todo!(),
            Exception::NormalInterrupt => todo!(),
            Exception::FastInterrupt => todo!(),
        };

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
            PsrKind::SPSR => self.bank.get_spsr(self.cpsr.operating_mode()),
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
            self.set_reg(rd, self.bus.read_byte(addr).into());
            self.bus.write_byte(addr, self.get_reg(rm) as u8);
        } else {
            self.set_reg(rd, self.bus.read_word(addr));
            self.bus.write_word(addr, self.get_reg(rm));
        }

        Cycle { i: 1, s: 1, n: 2 }
    }

    fn get_sn_cycle(&self, operand: &Operand) -> (u8, u8) {
        if operand.is_pc() { (2, 1) } else { (1, 0) }
    }
}

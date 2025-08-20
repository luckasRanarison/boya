use crate::{
    arm7tdmi::common::{Cycle, Exception, OperandKind, OperatingMode, ToOperand},
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
        lhs: Operand,
        rhs: Operand,
        dst: Option<u8>,
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
    pub fn shift_op<F>(&mut self, func: F, lhs: u8, rhs: Operand, dst: u8) -> Cycle
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
    pub fn logical_op<F>(&mut self, func: F, lhs: u8, rhs: Operand, dst: Option<u8>) -> Cycle
    where
        F: Fn(u32, u32) -> u32,
    {
        let (s, n) = self.get_sn_cycle(&lhs.reg());
        let lhs = self.get_reg(lhs);
        let rhs = self.get_operand(rhs);
        let result = func(lhs, rhs);

        self.cpsr.update_zn(result);

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

        if !value.has(0) {
            self.cpsr.set_arm_mode();
        }

        self.set_pc(value);

        Cycle { i: 0, s: 2, n: 1 }
    }

    #[inline(always)]
    pub fn mul_op(&mut self, lhs: u8, rhs: Operand, dst: u8) -> Cycle {
        let lhs = self.get_reg(lhs);
        let rhs = self.get_operand(rhs);
        let result = lhs.wrapping_mul(rhs);

        let i = match rhs {
            _ if rhs.get_bits(24, 31) != 0 => 4,
            _ if rhs.get_bits(16, 23) != 0 => 3,
            _ if rhs.get_bits(8, 15) != 0 => 2,
            _ => 1,
        };

        self.cpsr.update_zn(result);
        self.cpsr.update(Psr::C, false);

        self.set_reg(dst, result);

        Cycle { i, s: 1, n: 0 }
    }

    #[inline(always)]
    pub fn ldr_op(&mut self, rd: u8, addr: u32, kind: DataType, signed: bool) -> Cycle {
        let value = match kind {
            DataType::Byte if signed => self.bus.read_byte(addr) as i8 as i32 as u32,
            DataType::HWord if signed => self.bus.read_hword(addr) as i8 as i32 as u32,
            DataType::Byte => self.bus.read_byte(addr).into(),
            DataType::HWord => self.bus.read_hword(addr).into(),
            DataType::Word => self.bus.read_word(addr),
        };

        self.set_reg(rd, value);

        Cycle { i: 1, s: 1, n: 1 }
    }

    #[inline(always)]
    pub fn str_op(&mut self, rs: u8, addr: u32, kind: DataType) -> Cycle {
        let value = self.get_reg(rs);

        match kind {
            DataType::Byte => self.bus.write_byte(addr, (value & 0xFF) as u8),
            DataType::HWord => self.bus.write_hword(addr, (value & 0xFFFF) as u16),
            DataType::Word => self.bus.write_word(addr, value),
        }

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
    pub fn branch_op(&mut self, condition: bool, offset: i16) -> Cycle {
        if !condition {
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

        self.set_reg(Self::LR as usize, result);

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
        self.cpsr.set_arm_mode();
        self.cpsr.update(Psr::I, irq);
        self.cpsr.update(Psr::F, fiq);

        self.set_pc(vector);
        self.load_pipeline();

        Cycle { i: 0, s: 2, n: 1 }
    }

    fn get_sn_cycle(&self, operand: &Operand) -> (u8, u8) {
        if operand.is_pc() { (2, 1) } else { (1, 0) }
    }
}

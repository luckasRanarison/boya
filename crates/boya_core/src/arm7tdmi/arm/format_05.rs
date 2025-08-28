use crate::arm7tdmi::isa::prelude::*;

/// Multiply long and Multiply-Accumulate long
/// +-----------------------------------------------------------------+
/// |...3 ..................2 ..................1 ..................0.|
/// |-----------------------------------------------------------------|
/// |_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_|
/// |-----------------------------------------------------------------|
/// |  Cond  |0 0 0 0 1|U|A|S|  RdHi |  RdLo |  Rn   |1 0 0 1|  Rm    |
/// +-----------------------------------------------------------------+
pub struct Instruction {
    cd: Condition,
    op: Opcode,
    s: bool,
    hi: u8,
    lo: u8,
    rs: u8,
    rm: u8,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?} {:?}, {:?}, {:?}, {:?}",
            self.op,
            self.cd,
            self.lo.reg(),
            self.hi.reg(),
            self.rm.reg(),
            self.rs.reg()
        )
    }
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        let cd = value.get_bits_u8(28, 31).into();
        let op = value.get_bits_u8(21, 22).into();
        let s = value.has(20);
        let hi = value.get_bits_u8(16, 19);
        let lo = value.get_bits_u8(12, 15);
        let rs = value.get_bits_u8(8, 11);
        let rm = value.get_bits_u8(0, 3);

        Self {
            cd,
            op,
            s,
            hi,
            lo,
            rs,
            rm,
        }
    }
}

#[derive(Debug)]
enum Opcode {
    UMULL,
    UMLAL,
    SMULL,
    SMLAL,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x0 => Self::UMULL,
            0x1 => Self::UMLAL,
            0x2 => Self::SMULL,
            0x3 => Self::SMLAL,
            _ => panic!("invalid format 5 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Executable<B> for Instruction {
    fn condition(&self) -> Condition {
        self.cd
    }

    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        let rd = LongOperand::long(self.lo, self.hi);

        match self.op {
            Opcode::UMULL => cpu.umull(rd, self.rm, self.rs, self.s),
            Opcode::UMLAL => cpu.umula(rd, self.rm, self.rs, self.s),
            Opcode::SMULL => cpu.smull(rd, self.rm, self.rs, self.s),
            Opcode::SMLAL => cpu.smula(rd, self.rm, self.rs, self.s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_long() {
        let asm = r"
            MOV    R0, #-5
            MOV    R1, #2
            SMULL  R2, R3, R0, R1
            MOV    R4, R2
            MOV    R5, R3
            SMLAL  R4, R5, R0, R1
        ";

        AsmTestBuilder::new()
            .asm(asm)
            .assert_reg(2, -10_i64 as u32)
            .assert_reg(3, (-10_i64 >> 32) as u32)
            .assert_reg(4, -20_i64 as u32)
            .assert_reg(5, (-20_i64 >> 32) as u32)
            .run(6);
    }
}

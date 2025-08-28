use crate::arm7tdmi::isa::prelude::*;

/// Multiply and Multiply-Accumulate
/// +-----------------------------------------------------------------+
/// |...3 ..................2 ..................1 ..................0.|
/// |-----------------------------------------------------------------|
/// |_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_|
/// |-----------------------------------------------------------------|
/// |  Cond  |0 0 0 0 0 0|A|S|  Rd   |  Rn   |  Rs   |1 0 0 1|  Rm    |
/// +-----------------------------------------------------------------+
pub struct Format4 {
    cd: Condition,
    op: Opcode,
    s: bool,
    rd: u8,
    rn: u8,
    rs: u8,
    rm: u8,
}

impl Debug for Format4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.op)
    }
}

impl From<u32> for Format4 {
    fn from(value: u32) -> Self {
        let cd = value.get_bits_u8(28, 31).into();
        let op = value.get_u8(21).into();
        let s = value.has(20);
        let rd = value.get_bits_u8(16, 19);
        let rn = value.get_bits_u8(12, 15);
        let rs = value.get_bits_u8(8, 11);
        let rm = value.get_bits_u8(0, 3);

        Self {
            cd,
            op,
            s,
            rd,
            rn,
            rs,
            rm,
        }
    }
}

#[derive(Debug)]
enum Opcode {
    MUL,
    MLA,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x0 => Self::MUL,
            0x1 => Self::MLA,
            _ => panic!("invalid format 4 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Executable<B> for Format4 {
    fn condition(&self) -> Condition {
        self.cd
    }

    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        match self.op {
            Opcode::MUL => cpu.mul(self.rd, self.rm, self.rs, self.s),
            Opcode::MLA => cpu.mla(self.rd, self.rm, self.rs, self.rn, self.s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let asm = r"
            MOV    R0, #0xFFFF_FFFF
            MOV    R1, #0x2
            MOV    R2, #0x5
            MOV    R3, #0x1
            MUL    R4, R0, R1
            MLA    R5, R1, R2, R3
        ";

        AsmTestBuilder::new()
            .asm(asm)
            .assert_reg(4, 0xFFFF_FFFE)
            .assert_reg(5, 11)
            .run(6);
    }
}

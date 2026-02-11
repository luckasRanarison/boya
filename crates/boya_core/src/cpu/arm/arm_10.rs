use crate::cpu::isa::prelude::*;

/// Halfword and Signed data transfer
/// +-----------------------------------------------------------------+
/// |...3 ..................2 ..................1 ..................0.|
/// |-----------------------------------------------------------------|
/// |_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_|
/// |-----------------------------------------------------------------|
/// |  Cond  |0 0 0|P|U|I|W|L|  Rn   |  Rd   |0 0 0 0 1|S|H|1|  Rm    |
/// |-----------------------------------------------------------------|
/// |  Cond  |0 0 0|P|U|I|W|L|  Rn   |  Rd   | Offset|1|S|H|1| Offset |
/// +-----------------------------------------------------------------+
#[derive(Debug)]
pub struct Instruction {
    pub op: Opcode,
    pub cd: Condition,
    pub amod: AddrMode,
    pub wb: bool,
    pub rd: u8,
    pub rn: u8,
    pub of: Operand,
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        let cd = value.get_bits_u8(28, 31).into();
        let p = value.get_u8(24);
        let u = value.get_u8(23);
        let wb = value.has(21) || p == 0;
        let rn = value.get_bits_u8(16, 19);
        let rd = value.get_bits_u8(12, 15);
        let op = value.into();
        let amod = AddrMode::new(p, u);

        let of = match value.has(22) {
            true => ((value.get_bits(8, 11) << 4) | (value.get_bits(0, 3))).imm(),
            false => value.get_bits(0, 3).reg(),
        };

        Self {
            cd,
            amod,
            wb,
            rd,
            rn,
            op,
            of,
        }
    }
}

#[derive(Debug)]
pub enum Opcode {
    STRH,
    LDRH,
    LDSB,
    LDSH,
}

impl From<u32> for Opcode {
    fn from(value: u32) -> Self {
        match (value.get(20), value.get_bits(5, 6)) {
            (0, 0x1) => Self::STRH,
            (1, 0x1) => Self::LDRH,
            (1, 0x2) => Self::LDSB,
            (1, 0x3) => Self::LDSH,
            (_, op) => unreachable!("invalid arm 10 opcode: {op:#b}"),
        }
    }
}

impl Executable for Instruction {
    fn condition(&self) -> Condition {
        self.cd
    }

    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        let value = cpu.get_operand(self.of);
        let offset = RegisterOffset::new(value, self.amod, self.wb);

        match self.op {
            Opcode::STRH => cpu.strh(self.rd, self.rn, offset),
            Opcode::LDRH => cpu.ldrh(self.rd, self.rn, offset),
            Opcode::LDSB => cpu.ldsb(self.rd, self.rn, offset),
            Opcode::LDSH => cpu.ldsh(self.rd, self.rn, offset),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hword_signed_transfer() {
        let asm = r"
            MOV     R0, #3
            MOV     R1, 0x0200_0000
            STRH    R0, [R1, 0x50]!
            MOV     R2, R1
            LDRH    R3, [R2], -R0
        ";

        GbaTestBuilder::new()
            .asm(asm)
            .assert_hword(0x0200_0050, 3)
            .assert_reg(1, 0x0200_0050)
            .assert_reg(2, 0x0200_004D)
            .assert_reg(3, 3)
            .run(5);
    }
}
